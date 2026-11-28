//! The encrypted store core (spec 002 T023; FR-001, FR-003, FR-007, PLAN-14, PLAN-16, FOUND-07).
//! A SQLCipher database opened with a raw 256-bit key, one `store_header` row, and strict
//! settings that are read back after they are set. Errors are mapped to `meep_core::errors::Error`
//! with a content-free `Cause`; no error text from rusqlite or std is kept (ERR-01 item 3).
//! There are no `From` impls and no logging here.

use data_encoding::HEXLOWER;
use meep_core::clock::{Clock, to_rfc3339};
use meep_core::errors::{Cause, Error};
use meep_core::ids::{AccountId, StoreId};
use rusqlite::{Connection, OpenFlags, params};
use std::fmt;
use std::path::Path;
use std::time::Duration;
use zeroize::Zeroizing;

/// File names (data-model.md).
const STORE_FILE: &str = "store.db";
const STORE_WAL_FILE: &str = "store.db-wal";

/// D9: the layout version a new store starts at.
const FORMAT_VERSION: u32 = 1;
/// D9 item d: how long a busy database is waited for.
const BUSY_TIMEOUT_MS: u64 = 5000;

const KEY_PREFIX: &str = "PRAGMA key = \"x'";
/// A raw 256-bit key is 64 lowercase hex characters.
const KEY_HEX_LEN: usize = 64;

const CREATE_STORE_HEADER: &str = "CREATE TABLE store_header (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    store_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    format_version INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_successful_backup_at TEXT
) STRICT";

const INSERT_STORE_HEADER: &str = "INSERT INTO store_header
    (id, store_id, account_id, format_version, created_at)
    VALUES (1, ?1, ?2, ?3, ?4)";

/// A raw 256-bit database key. It is wiped when dropped, cannot be copied, cloned, printed or
/// serialized, and its `Debug` output never shows the bytes (FR-003).
pub struct DbKey(Zeroizing<[u8; 32]>);

impl DbKey {
    pub fn from_bytes(bytes: [u8; 32]) -> DbKey {
        DbKey(Zeroizing::new(bytes))
    }
}

impl fmt::Debug for DbKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DbKey(<redacted>)")
    }
}

/// The one `store_header` row (data-model.md).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreHeader {
    pub store_id: StoreId,
    pub account_id: AccountId,
    pub format_version: u32,
    /// UTC, RFC 3339.
    pub created_at: String,
    /// NULL until the first verified backup (FR-044).
    pub last_successful_backup_at: Option<String>,
}

/// The settings that are in effect on an open connection, as the database reports them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSettings {
    pub journal_mode: String,
    pub synchronous: i64,
    pub secure_delete: bool,
    pub foreign_keys: bool,
    pub busy_timeout_ms: u64,
    pub cipher_memory_security: bool,
}

/// An open store: the SQLCipher connection.
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Create a new encrypted store in `dir` and write its header. Refuses, writing nothing, where
    /// a store already exists.
    pub fn create(dir: &Path, key: &DbKey, clock: &dyn Clock) -> Result<Store, Error> {
        Store::create_with(dir, Some(key), clock)
    }

    /// Open the existing encrypted store in `dir`. No key, a wrong key and a file that is not a
    /// store are one refusal (`StoreKeyRejected`): SQLCipher cannot tell them apart (FR-003).
    pub fn open(dir: &Path, key: &DbKey) -> Result<Store, Error> {
        Store::open_with(dir, Some(key))
    }

    /// Create a store without encryption, for tests only (FOUND-07).
    #[cfg(feature = "plaintext-test-store")]
    pub fn create_plaintext(dir: &Path, clock: &dyn Clock) -> Result<Store, Error> {
        Store::create_with(dir, None, clock)
    }

    /// Open a store made by `create_plaintext`, for tests only (FOUND-07).
    #[cfg(feature = "plaintext-test-store")]
    pub fn open_plaintext(dir: &Path) -> Result<Store, Error> {
        Store::open_with(dir, None)
    }

    /// The store header row.
    pub fn header(&self) -> Result<StoreHeader, Error> {
        let (store_id, account_id, version, created_at, backup) = self
            .conn
            .query_row(
                "SELECT store_id, account_id, format_version, created_at, last_successful_backup_at
                 FROM store_header WHERE id = 1",
                [],
                |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, i64>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, Option<String>>(4)?,
                    ))
                },
            )
            .map_err(|e| store_failed(&e))?;
        let unreadable = Error::StoreFailed { cause: Cause::None };
        Ok(StoreHeader {
            store_id: store_id.parse().map_err(|_| unreadable.clone())?,
            account_id: account_id.parse().map_err(|_| unreadable.clone())?,
            format_version: u32::try_from(version).map_err(|_| unreadable)?,
            created_at,
            last_successful_backup_at: backup,
        })
    }

    /// The settings in effect on this connection, read from the database.
    pub fn settings(&self) -> Result<StoreSettings, Error> {
        read_settings(&self.conn).map_err(|e| store_failed(&e))
    }

    fn create_with(dir: &Path, key: Option<&DbKey>, clock: &dyn Clock) -> Result<Store, Error> {
        let path = dir.join(STORE_FILE);
        if exists(&path)? || exists(&dir.join(STORE_WAL_FILE))? {
            return Err(Error::StoreExists);
        }
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let mut conn = Connection::open_with_flags(&path, flags).map_err(|e| open_failed(&e))?;
        start(&conn, key).map_err(|e| match e {
            Start::NotADatabase(c) | Start::Failed(c) => Error::StoreOpenFailed { cause: c },
        })?;
        write_header(&mut conn, clock)?;
        restrict_to_owner(&path)?;
        Ok(Store { conn })
    }

    fn open_with(dir: &Path, key: Option<&DbKey>) -> Result<Store, Error> {
        let path = dir.join(STORE_FILE);
        if !exists(&path)? {
            return Err(Error::StoreMissing);
        }
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let conn = Connection::open_with_flags(&path, flags).map_err(|e| open_failed(&e))?;
        start(&conn, key).map_err(|e| match e {
            Start::NotADatabase(_) => Error::StoreKeyRejected,
            Start::Failed(c) => Error::StoreOpenFailed { cause: c },
        })?;
        // A database the key opens but that has no header is not a Meep store.
        let has_header: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'store_header'",
                [],
                |r| r.get(0),
            )
            .map_err(|e| open_failed(&e))?;
        if has_header == 0 {
            return Err(Error::StoreKeyRejected);
        }
        Ok(Store { conn })
    }
}

/// Why the opening sequence stopped.
enum Start {
    /// SQLCipher read the file and it did not decrypt (`SQLITE_NOTADB`).
    NotADatabase(Cause),
    Failed(Cause),
}

/// The opening order (directive S4.2 steps 2 to 7), shared by create and open.
fn start(conn: &Connection, key: Option<&DbKey>) -> Result<(), Start> {
    let failed = |e: rusqlite::Error| Start::Failed(sqlite_cause(&e));
    if let Some(key) = key {
        apply_key(conn, key).map_err(failed)?;
    }
    conn.execute_batch("PRAGMA cipher_compatibility = 4;")
        .map_err(failed)?;
    conn.execute_batch("PRAGMA cipher_memory_security = ON;")
        .map_err(failed)?;
    // The first read: this is where SQLCipher finds out whether the key fits the file.
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| {
        r.get::<_, i64>(0)
    })
    .map_err(|e| {
        if is_not_a_database(&e) {
            Start::NotADatabase(sqlite_cause(&e))
        } else {
            failed(e)
        }
    })?;
    conn.execute_batch("PRAGMA journal_mode = WAL;")
        .map_err(failed)?;
    conn.execute_batch("PRAGMA synchronous = FULL;")
        .map_err(failed)?;
    conn.execute_batch("PRAGMA secure_delete = ON;")
        .map_err(failed)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(failed)?;
    conn.busy_timeout(Duration::from_millis(BUSY_TIMEOUT_MS))
        .map_err(failed)?;
    let in_effect = read_settings(conn).map_err(failed)?;
    if in_effect != expected_settings() {
        return Err(Start::Failed(Cause::None));
    }
    Ok(())
}

fn expected_settings() -> StoreSettings {
    StoreSettings {
        journal_mode: "wal".to_string(),
        synchronous: 2, // FULL
        secure_delete: true,
        foreign_keys: true,
        busy_timeout_ms: BUSY_TIMEOUT_MS,
        cipher_memory_security: true,
    }
}

fn read_settings(conn: &Connection) -> Result<StoreSettings, rusqlite::Error> {
    let journal_mode: String = conn.query_row("PRAGMA journal_mode", [], |r| r.get(0))?;
    let synchronous: i64 = conn.query_row("PRAGMA synchronous", [], |r| r.get(0))?;
    let secure_delete: i64 = conn.query_row("PRAGMA secure_delete", [], |r| r.get(0))?;
    let foreign_keys: i64 = conn.query_row("PRAGMA foreign_keys", [], |r| r.get(0))?;
    let busy_timeout: i64 = conn.query_row("PRAGMA busy_timeout", [], |r| r.get(0))?;
    // SQLCipher reports this one as text: "1" once memory security is on and has run.
    let memory_security: String =
        conn.query_row("PRAGMA cipher_memory_security", [], |r| r.get(0))?;
    Ok(StoreSettings {
        journal_mode,
        synchronous,
        secure_delete: secure_delete == 1,
        foreign_keys: foreign_keys == 1,
        busy_timeout_ms: u64::try_from(busy_timeout)
            .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, busy_timeout))?,
        cipher_memory_security: memory_security == "1",
    })
}

/// The one place SQL is built at run time (SEC-05 item 1, owner decision D11-A): SQLCipher cannot
/// take its key as a bound parameter. The statement holds only the key's 64 lowercase hex
/// characters, in one buffer that is wiped when this function returns.
fn apply_key(conn: &Connection, key: &DbKey) -> Result<(), rusqlite::Error> {
    let mut stmt = Zeroizing::new(String::from(KEY_PREFIX));
    // Make all the room first, so the key characters are never copied to a new allocation.
    stmt.reserve_exact(KEY_HEX_LEN + 2);
    HEXLOWER.encode_append(key.0.as_ref(), &mut stmt);
    stmt.push('\'');
    stmt.push('"');
    conn.execute_batch(&stmt) // SEC-05 exception (D11)
}

/// Create the header table and its one row, in one transaction (data-model.md store_header).
fn write_header(conn: &mut Connection, clock: &dyn Clock) -> Result<(), Error> {
    let created_at =
        to_rfc3339(clock.now()).map_err(|_| Error::StoreOpenFailed { cause: Cause::None })?;
    let tx = conn.transaction().map_err(|e| open_failed(&e))?;
    tx.execute_batch(CREATE_STORE_HEADER)
        .map_err(|e| open_failed(&e))?;
    tx.execute(
        INSERT_STORE_HEADER,
        params![
            StoreId::new().to_string(),
            AccountId::new().to_string(),
            FORMAT_VERSION,
            created_at
        ],
    )
    .map_err(|e| open_failed(&e))?;
    tx.commit().map_err(|e| open_failed(&e))
}

/// `store.db` readable and writable by the owner only (FR-003).
#[cfg(unix)]
fn restrict_to_owner(path: &Path) -> Result<(), Error> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).map_err(|e| {
        Error::StoreOpenFailed {
            cause: Cause::Io(e.kind()),
        }
    })
}

#[cfg(not(unix))]
fn restrict_to_owner(_path: &Path) -> Result<(), Error> {
    Ok(())
}

fn exists(path: &Path) -> Result<bool, Error> {
    path.try_exists().map_err(|e| Error::StoreOpenFailed {
        cause: Cause::Io(e.kind()),
    })
}

fn sqlite_cause(e: &rusqlite::Error) -> Cause {
    match e {
        rusqlite::Error::SqliteFailure(err, _) => Cause::Sqlite(err.extended_code),
        rusqlite::Error::SqlInputError { error, .. } => Cause::Sqlite(error.extended_code),
        _ => Cause::None,
    }
}

fn is_not_a_database(e: &rusqlite::Error) -> bool {
    matches!(e, rusqlite::Error::SqliteFailure(err, _)
        if err.code == rusqlite::ErrorCode::NotADatabase)
}

fn open_failed(e: &rusqlite::Error) -> Error {
    Error::StoreOpenFailed {
        cause: sqlite_cause(e),
    }
}

fn store_failed(e: &rusqlite::Error) -> Error {
    Error::StoreFailed {
        cause: sqlite_cause(e),
    }
}

/// Test-only marker the release check looks for (FOUND-07, S4.6). Present only with the feature.
#[cfg(feature = "plaintext-test-store")]
#[used]
static PLAINTEXT_TEST_STORE_MARKER: &str = "MEEP_TEST_FEATURE_PLAINTEXT_STORE";
