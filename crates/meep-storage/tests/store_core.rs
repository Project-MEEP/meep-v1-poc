//! The encrypted store core (spec 002 T022; FR-001, FR-003, FR-007, SC-001, PLAN-14, PLAN-18,
//! directive 0730 S3). Synthetic keys and ids only.

// ERR-01: helpers outside #[test] functions may unwrap and expect in test code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::clock::{FixedClock, parse_rfc3339};
use meep_core::errors::Error;
use meep_core::ids::{AccountId, StoreId};
use meep_storage::store::{DbKey, Store};
use std::path::Path;

const STORE_FILE: &str = "store.db";
const CREATED_AT: &str = "2026-11-28T07:30:00Z";

fn key(fill: u8) -> DbKey {
    DbKey::from_bytes([fill; 32])
}

fn clock() -> FixedClock {
    FixedClock::new(parse_rfc3339(CREATED_AT).unwrap())
}

fn store_path(dir: &Path) -> std::path::PathBuf {
    dir.join(STORE_FILE)
}

/// Create a store, then close it so the write-ahead log is folded into the file.
fn create_closed(dir: &Path, fill: u8) {
    let store = Store::create(dir, &key(fill), &clock()).unwrap();
    drop(store);
}

fn dir_entries(dir: &Path) -> usize {
    std::fs::read_dir(dir).unwrap().count()
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// FR-001, PLAN-14: a store is created with a raw key and opens again with the same key.
#[test]
fn a_store_is_created_and_reopened_with_its_key() {
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    assert!(Store::open(dir.path(), &key(0x11)).is_ok());
}

/// FR-007, data-model.md store_header: version 1, two different random ids, the clock's time.
#[test]
fn the_header_holds_format_version_one_and_fresh_ids() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::create(dir.path(), &key(0x11), &clock()).unwrap();
    let header = store.header().unwrap();
    assert_eq!(header.format_version, 1);
    // Canonical hyphenated UUID v4 only: FromStr refuses anything else.
    assert!(header.store_id.to_string().parse::<StoreId>().is_ok());
    assert!(header.account_id.to_string().parse::<AccountId>().is_ok());
    assert_ne!(header.store_id.to_string(), header.account_id.to_string());
    assert_eq!(header.created_at, CREATED_AT);
    assert_eq!(header.last_successful_backup_at, None);

    // The header read from a reopened store is the same one.
    drop(store);
    let reopened = Store::open(dir.path(), &key(0x11)).unwrap();
    assert_eq!(reopened.header().unwrap(), header);
}

/// FR-003, D10: a wrong key is refused, and the refusal does not say why.
#[test]
fn opening_with_a_wrong_key_is_refused_as_key_rejected() {
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    let refused = Store::open(dir.path(), &key(0x22)).err();
    assert_eq!(refused, Some(Error::StoreKeyRejected));
}

/// FR-003, D10: a file that is not a store looks the same as a wrong key, and is left untouched.
#[test]
fn a_file_that_is_not_a_store_is_refused_as_key_rejected() {
    let dir = tempfile::tempdir().unwrap();
    // Synthetic pseudo-random bytes (a fixed linear congruential sequence, not secret).
    let mut state: u32 = 0x1234_5678;
    let junk: Vec<u8> = (0..8192)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (state >> 24) as u8
        })
        .collect();
    std::fs::write(store_path(dir.path()), &junk).unwrap();

    let refused = Store::open(dir.path(), &key(0x11)).err();
    assert_eq!(refused, Some(Error::StoreKeyRejected));
    assert_eq!(std::fs::read(store_path(dir.path())).unwrap(), junk);
}

/// FR-003, SC-001: without the key the file cannot be read as a database.
#[test]
fn the_store_cannot_be_read_without_a_key() {
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    let plain = rusqlite::Connection::open(store_path(dir.path())).unwrap();
    let read = plain.query_row("SELECT count(*) FROM sqlite_master", [], |r| {
        r.get::<_, i64>(0)
    });
    assert!(read.is_err());
}

/// D10: opening where no store exists is refused, and opening does not create one.
#[test]
fn opening_a_missing_store_is_refused_and_creates_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let refused = Store::open(dir.path(), &key(0x11)).err();
    assert_eq!(refused, Some(Error::StoreMissing));
    assert_eq!(dir_entries(dir.path()), 0);
}

/// D10: create never overwrites. The refusal changes nothing, and the first key still works.
#[test]
fn create_refuses_an_existing_store_and_leaves_it_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    let before = std::fs::read(store_path(dir.path())).unwrap();

    let refused = Store::create(dir.path(), &key(0x22), &clock()).err();
    assert_eq!(refused, Some(Error::StoreExists));

    assert_eq!(std::fs::read(store_path(dir.path())).unwrap(), before);
    assert!(Store::open(dir.path(), &key(0x11)).is_ok());
}

/// PLAN-14, D8-A, D9: the pinned settings are in effect on the open connection.
#[test]
fn the_settings_are_in_effect_after_open() {
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    let store = Store::open(dir.path(), &key(0x11)).unwrap();
    let s = store.settings().unwrap();
    assert_eq!(s.journal_mode, "wal");
    assert_eq!(s.synchronous, 2);
    assert!(s.secure_delete);
    assert!(s.foreign_keys);
    assert_eq!(s.busy_timeout_ms, 5000);
    assert!(s.cipher_memory_security);
}

/// FR-003, SC-001: neither the file nor its log shows plaintext, the ids, or the SQLite magic.
#[test]
fn the_store_file_holds_no_plaintext() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::create(dir.path(), &key(0x11), &clock()).unwrap();
    let header = store.header().unwrap();
    let account = header.account_id.to_string();
    let store_id = header.store_id.to_string();

    let check = |dir: &Path| {
        let mut files = vec![std::fs::read(store_path(dir)).unwrap()];
        let wal = dir.join("store.db-wal");
        if wal.exists() {
            files.push(std::fs::read(wal).unwrap());
        }
        for bytes in files {
            assert!(!contains(&bytes, account.as_bytes()));
            assert!(!contains(&bytes, store_id.as_bytes()));
            assert!(!contains(&bytes, b"SQLite format 3"));
        }
    };
    // While the store is open (the log may hold the newest pages), and after it is closed.
    check(dir.path());
    drop(store);
    check(dir.path());
}

/// PLAN-14: a key never shows in debug output.
#[test]
fn a_database_key_never_shows_in_debug_output() {
    let key = DbKey::from_bytes([0x42; 32]);
    assert_eq!(format!("{key:?}"), "DbKey(<redacted>)");
}

/// FR-003: only the owner can read the store file (Unix).
#[cfg(unix)]
#[test]
fn the_store_file_is_owner_only_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    create_closed(dir.path(), 0x11);
    let mode = std::fs::metadata(store_path(dir.path()))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600);
}

/// FOUND-07: the test-only plaintext store is a plain SQLite file anyone can read.
#[cfg(feature = "plaintext-test-store")]
#[test]
fn the_plaintext_test_store_is_readable_without_a_key() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::create_plaintext(dir.path(), &clock()).unwrap();
    drop(store);
    let bytes = std::fs::read(store_path(dir.path())).unwrap();
    assert!(bytes.starts_with(b"SQLite format 3\0"));
    let plain = rusqlite::Connection::open(store_path(dir.path())).unwrap();
    let rows: i64 = plain
        .query_row("SELECT count(*) FROM store_header", [], |r| r.get(0))
        .unwrap();
    assert_eq!(rows, 1);
}
