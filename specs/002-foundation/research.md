# Research: Foundation (v1.0)

Phase 0 output. Every technology choice is **Proposed** until the owner confirms its decision record (`docs/decisions/PLAN-13.md` to `PLAN-27.md`). Choices that implement a Confirmed decision say which one. PLAN-01 to PLAN-12 (from the names plan) are treated as Proposed inputs: reused where they fit, refined by a new record where the foundation needs something different, and never rewritten.

## Verification log (2026-11-19)

| Item | Source or method | Result |
|---|---|---|
| Crate names, versions, licenses, last update | crates.io API | All found. `rusqlite` 0.43.0 (MIT), `libsqlite3-sys` 0.40.0, `sqlcipher-src` 420.0.0 (SQLCipher 4.20.0 on SQLite 3.54.1, libtomcrypt 1.18.2; license "MIT AND BSD-3-Clause AND blessing AND WTFPL"), `keyring` 4.2.0, `zip` 9.8.0 (MIT), `schemars` 1.2.3 (MIT), `jsonschema` 0.61.2 (MIT), `serde_json_canonicalizer` 0.4.1 (MIT), `serde_jcs` 0.3.0, `data-encoding` 2.11.1, `subtle` 2.6.1 (**BSD-3-Clause**), `tower` 0.5.3, `tower-http` 0.7.1, `cookie` 0.18.2, `tempfile` 3.27.0, `time` 0.3.55. `fd-lock` 4.0.4 and `fs4` 1.1.0 exist but are not needed (see lock row). `fail` 0.5.1 was last updated in 2022 and is not used. |
| `rusqlite` SQLCipher features | crates.io feature list for `libsqlite3-sys` 0.40.0 | `bundled-sqlcipher` (pulls `sqlcipher-src`), `bundled-sqlcipher-vendored-openssl` (adds `openssl-sys/vendored`), `sqlcipher` (link a system SQLCipher) |
| **SQLCipher build on this PC, `bundled-sqlcipher`** | Throwaway project in the session scratchpad, outside the repo | **Fails**: the build script stops with "Missing environment variable OPENSSL_DIR or OPENSSL_DIR is not set". `sqlcipher-src` mentions a libtomcrypt provider "for targets without a system crypto library", but `libsqlite3-sys` still asks for OpenSSL on Windows MSVC. |
| **SQLCipher build, `bundled-sqlcipher-vendored-openssl`** | Same throwaway project | **Fails at the first attempt**: OpenSSL is built from source (`no-asm`, so no NASM needed) but its configure step needs a native `perl` on the Windows PATH, and only Git's bash-side Perl exists. A winget install of Strawberry Perl was started; see "Spike status" below. |
| One-running-copy lock | Throwaway project using `std::fs::File::try_lock` (rustc 1.100.0) | **Verified on this PC**: a second process was refused ("lock held by another") while the first held the lock; after the first was hard-killed (a simulated crash) a third process acquired the lock. No crate is needed. |
| Operating-system keychain backends | `keyring` 4.2.0 feature list | The feature list shows `cli`, `default`, `v1`, so per-platform backend selection is **not confirmed** from metadata; to be confirmed in the first scaffold task against `keyring`'s documentation and a throwaway test on Windows. |
| SQLite and SQLCipher behaviors (WAL recovery on open, `secure_delete`, `wal_checkpoint(TRUNCATE)`, `cipher_integrity_check`) | Documented behavior known to the author | **Not machine-verified in this session.** Each is covered by a required test in the plan (PLAN-16, PLAN-20) so the claim is proven at implementation. |
| Standard-library file locking | Rust release notes | `File::lock`, `try_lock` and `unlock` were added to the standard library; confirmed present in the installed 1.100.0 by compiling the spike. |
| License fit with MIT (principle 18) | crate licenses | All MIT, Apache-2.0, BSD-3-Clause or Unicode-3.0 family. `deny.toml` must allow **Unicode-3.0 and BSD-3-Clause** and handle the **blessing** (SQLite public-domain notice) and **WTFPL** terms that `sqlcipher-src` carries; confirm they pass `cargo-deny` or add reviewed exceptions. |

## Spike status

- **Done and verified:** the operating-system lock (works on Windows, releases on a kill); the crate and feature inventory.
- **Blocked on this PC:** SQLCipher needs either a system OpenSSL (`OPENSSL_DIR` with headers and libraries) or the vendored OpenSSL build, which needs a native Windows Perl. The Perl install is in progress; if it finishes, the vendored build is retried and the result is added to PLAN-14. If it does not finish, the first scaffold task must settle it, and the owner can choose between installing Strawberry Perl, installing a prebuilt OpenSSL development package, or both. The design below does not depend on which.

## Decisions

### D1. Workspace layout (PLAN-13)

- **Decision**: four crates: `meep-core` (pure domain and ports), `meep-storage` (encrypted store, keys, lock, damaged-store open, restore), `meep-backup` (archive writer and reader, read-me, schemas, verification), `meep-web` (server shell and message catalog; screens added later by specs 010 and 001). The server and anti-forgery parts of PLAN-07 move into the foundation.
- **Rationale**: keeps the domain testable without a database, keeps backup logic testable against the ports alone, and keeps web types out of the crates an Android wrapper will reuse (R3).
- **Alternatives**: three crates with backup inside storage (smaller, but mixes zip and schema code into the store crate); one crate (fastest, blocks reuse).

### D2. Encrypted store access (PLAN-14, refines PLAN-02)

- **Decision**: `rusqlite` 0.43.0 with `bundled-sqlcipher`, or `bundled-sqlcipher-vendored-openssl` when no system OpenSSL exists (the owner's R2 choice at product level). Open with a raw 256-bit key derived from the store key, WAL journal mode, `synchronous=FULL`, `secure_delete=ON`, foreign keys on.
- **Rationale**: SQLCipher gives whole-file encryption and is the confirmed choice. The spike showed the Windows build needs more than the crate alone.
- **Alternatives**: plain SQLite with application-level encryption of every column (loses whole-file encryption and the header), a different encrypted database (not the owner's choice), linking a system SQLCipher (`sqlcipher` feature; fewer build steps but a second thing to install and version).
- **Open**: which of the two build routes to use on Windows (see Spike status); Linux and macOS builds to confirm in the scaffold.

### D3. Key hierarchy (PLAN-15, refines PLAN-03, PLAN-04, PLAN-05)

- **Decision**: a random 256-bit store key in the operating-system keychain through `keyring` 4.2.0; a random 256-bit recovery key shown once, encoded as base32 groups with `data-encoding`, never stored; an HKDF-SHA-256 (`hkdf` 0.13.0, `sha2` 0.11.1) derivation from the recovery key and the store id to a wrap key that encrypts a copy of the store key in the header; the database key, the record-wrapping key and the recovery wrap key are separate HKDF subkeys with distinct labels; each record has a random 256-bit record key encrypted under the record-wrapping key with XChaCha20-Poly1305 (`chacha20poly1305` 0.11.0, record id as associated data); purpose subkeys are HKDF outputs of a record key with the label `meep/purpose/<purpose>`; secrets live in `zeroize`-wrapped memory; comparisons use `subtle`. No recovery-key replacement in v1.0 (FOUND-02); a restored store gets a new store key and a new recovery key (FOUND-03).
- **Rationale**: both keys are full-strength random values, so no slow password-stretching function is needed; separate labels mean one derived key cannot stand in for another; per-record keys give R8 its guarantee.
- **Alternatives**: a passphrase-derived recovery key (needs a stretching function and invites weak secrets); a 24-word phrase encoding (friendlier to write down; a close call, left to the owner as a presentation choice); a single key for everything (cannot destroy one record).
- **Open**: exact recovery-key encoding and length as shown to the owner; per-platform `keyring` backend features.

### D4. Crypto-shredding and proof (PLAN-16, refines PLAN-04)

- **Decision**: delete runs in one transaction: overwrite the record's wrapped-key row with zeros, delete the row, delete the record, append `record_deleted`; then checkpoint and truncate the write-ahead log; `secure_delete=ON` zeroes freed content. Proof: a test **opens the same code path against an unencrypted database** so page bytes are visible, plants sentinel content and a known fake key, deletes, and scans the raw database file, its log and its free pages for the sentinel and the key; a second test confirms the encrypted store still has no row for the key.
- **Rationale**: SQLCipher encrypts every page, so scanning the encrypted file cannot show whether content survived. The guarantee R8 needs is that content and key are gone even if the store key later leaks, and that is what an unencrypted-mode raw scan demonstrates.
- **Alternatives**: scanning only the encrypted file (cannot prove anything); trusting the database engine's behavior without a test (the spec's Risks table asks for purge tests).
- **Limit (stated plainly)**: this covers what Meep manages. Copies held by the operating system (file-system journals, SSD wear leveling, backups and exports made earlier) are outside it, which is why the spec says earlier backups cannot be recalled (FR-033 in spec 001, FR-010 here).

### D5. Change-log entry and hash chain (PLAN-17)

- **Decision**: entry body encoded with RFC 8785 JSON canonicalization (`serde_json_canonicalizer` 0.4.1, with `serde_jcs` 0.3.0 as the alternative), `hash` = SHA-256 of those bytes, `prev_hash` inside the body, genesis entry at `seq` 0 with 64 zeros, kinds and detail keys registered per feature, triggers refusing updates and deletes, verification that returns the first broken `seq`. Full contract in contracts/change-log-entry.md.
- **Rationale**: canonical encoding makes the hash the same on every platform; an unkeyed chain can be verified by anyone with the file, including a rescue reader; the spec asks for detectable alteration (FR-013), not protection against the key holder.
- **Alternatives**: a keyed chain (HMAC; verification then needs the key, and a damaged store may not have it); a Merkle tree (more than a linear log needs); ordering by timestamp (the spec says timestamps are informational only).

### D6. All-or-nothing writes and crash safety (PLAN-18)

- **Decision**: every record operation is one `BEGIN IMMEDIATE` transaction containing the record, its key and its log entry; WAL mode with `synchronous=FULL`; a failure rolls back and returns `save-failed`; a failpoint hook (a small feature-gated macro, not the stale `fail` crate) lets tests stop the process between steps.
- **Rationale**: SQLite's transaction guarantees give "nothing changes on failure" (FR-015); the failpoint tests prove it, including for restore.
- **Alternatives**: writing record and log separately (can leave a record with no log entry); the `fail` crate (not updated since 2022).

### D7. One running copy (PLAN-19, implements FOUND-04)

- **Decision**: at start, open or create `store.lock` beside the store and call the standard library's `File::try_lock`; hold the file handle for the life of the process; on `WouldBlock` or any error, refuse with `store-already-open`; never read a "who owns this" marker. After an unexpected stop the operating system has already released the lock, so the next start checks the store and the chain before opening normally (FR-034).
- **Rationale**: verified on this PC that the lock is refused while held and free after a hard kill; no crate, no guessing (FOUND-04).
- **Alternatives**: `fs4` 1.1.0 or `fd-lock` 4.0.4 (same OS mechanism with an extra dependency; the API I tried for `fs4` did not match, and the standard library suffices); a PID marker file (the stale-marker problem FOUND-04 rules out).
- **Limit**: network drives and some cloud-synced folders may not honor file locks; the default store location is a local application-data folder, and the documentation says not to place the store on a network share.

### D8. Damaged-store handling (PLAN-20, implements FOUND-03)

- **Decision**: never open the damaged file in place. Copy `store.db` and `store.db-wal` byte-for-byte into a private temporary folder, compute SHA-256 of the original files before and after, and work on the copy, so pending write-ahead-log content is included and the original cannot be changed by recovery steps the database engine performs on open. Then run, in order: key check, `cipher_integrity_check` and `integrity_check`, change-log verification, and a read and validate of every record. Outcomes: normal; read-only damaged (some records or the chain fail); refused (nothing readable). The read-only mode is a Meep mode in which every write operation returns `store-read-only`; the only offered action is the rescue backup. The rescue backup includes only records that decrypt and validate fully, lists the rest, and carries the old log as an unverified archive. The temporary copy is deleted when done.
- **Rationale**: the database engine can write while opening a log-mode database (even for reading), so any in-place open risks altering the file; a copy plus a hash comparison makes "never modifies" provable (FR-029, SC-012).
- **Alternatives**: opening in place with an immutable flag (ignores log content, so it would miss pending changes); asking the owner to copy files by hand (error-prone).
- **Open**: whether the temporary copy needs extra protection beyond the owner-only folder permissions, since it holds encrypted data only and the key is still required.

### D9. Owner backup archive (PLAN-21, implements FOUND-05, FOUND-06)

- **Decision**: `zip` 9.8.0 writes the archive described in contracts/backup-archive-format.md; `schemars` 1.2.3 derives machine-readable schemas from the same serde types the data uses; a manifest carries the format version, as-of time, counts and SHA-256 of every entry; the backup is written to a `.partial` name, verified by re-reading, flushed and renamed; the snapshot is one read transaction so saves continue normally.
- **Rationale**: one archive meets the spec; deriving schemas from the types keeps data and schema from drifting; the temporary-name step means a failure never leaves a file that looks complete.
- **Alternatives**: a folder of files (not one file, harder to move and verify); tar (less familiar to the owner than zip); hand-written schemas (drift).

### D10. Published format description and CI check (PLAN-22)

- **Decision**: the description lives in the repository under `docs/format/`: `READ-ME-FIRST.txt` (the fixed text with placeholders) and one `fragments/<fragment_id>.txt` per feature. The same files are the sources the backup writer renders. A test renders the template with placeholders left intact and compares byte-for-byte with the published files, and the existing CI `unit` job runs it, so a mismatch fails the pull request.
- **Rationale**: one source for both the backup and the published copy removes drift by construction, and the test catches a mismatch if someone edits one side.
- **Alternatives**: publishing to a separate website (outside the repository, no pull-request check); generating the docs at release time only (drift can ship).
- **Open for the owner**: a rendered web page of the description can be added later; the repository copy is the authoritative one.

### D11. Restore (PLAN-23, implements FR-017, FR-029 to FR-032, NAMES-05)

- **Decision**: validate everything before writing (every fragment, address and limit), collect all problems, refuse the whole restore if there is one; otherwise apply in one transaction into an empty store or after "Replace everything"; adopt the backup's account ID; create a new store key and recovery key; for a rescue backup, restore readable records only, load the unverified archive, start a fresh chain with `restored_from_rescue_backup`.
- **Rationale**: it is the owner's own rule set; a dry-run validation is what makes "refuse whole, list every problem" possible.
- **Alternatives**: validating while writing and rolling back (cannot list every problem); partial restore (rejected by the spec).

### D12. Address checker, identifiers and time (PLAN-24, refines PLAN-11)

- **Decision**: a small hand-written address parser in `meep-core` following R16; UUID v4 for the store id, account ID and record ids (`uuid` 1.29.3, `rand` 0.10.4); UTC timestamps in RFC 3339 through `time` 0.3.55 with an injectable clock so tests can control time.
- **Rationale**: the grammar is tiny and fully specified; an injectable clock makes the as-of and chain tests deterministic.
- **Alternatives**: a regular-expression crate (extra dependency for one rule); sequential ids (leak ordering, against R9).

### D13. Loopback server and anti-forgery (PLAN-25, implements R5; refines and moves part of PLAN-07)

- **Decision**: `axum` 0.8.9 on `tokio` 1.54.1 with `tower-http` 0.7.1 layers; loopback binding only; Host, Origin and `Sec-Fetch-Site` checks; a per-session random token as an `HttpOnly`, `SameSite=Strict` cookie plus a matching request token (`cookie` 0.18.2); no-store and content-security headers. Contract in contracts/local-access.md.
- **Rationale**: with no sign-in, these checks are the only barrier against other web pages (R5, FR-021).
- **Alternatives**: a random secret in the launch address only (breaks bookmarks and still needs request checks); a sign-in (excluded by R5).
- **Not covered**: other user accounts on the same computer (FOUND-02).

### D14. Owner-facing messages and Fluent (PLAN-26, refines PLAN-08, implements R10)

- **Decision**: every foundation message is a Fluent ID in `crates/meep-web/locales/en/foundation.ftl` with the exact text in contracts/owner-messages.md; typed errors map to IDs; a lint test fails if owner-visible text bypasses the catalog. The two owner-specified texts are asserted exactly.
- **Rationale**: R10 requires all text in message files; fixed wording is a contract, not a style choice.

### D15. Test strategy (PLAN-27, refines PLAN-12)

- **Decision**: unit tests for the master spec's foundation list (envelope on every record, address parser valid and invalid, database will not open without its key, append-only hash-chained log, JSON export and re-import equal, key derivation, theme-independent items out of scope); property tests (`proptest` 1.12.0) for chain verification and backup round trips; failpoint and child-process kill tests for every all-or-nothing claim (save, delete, append, restore, backup, SC-005, SC-015); two-process lock tests (SC-013); raw-byte destruction scans in unencrypted mode (SC-002); before-and-after hash comparison for damaged-store opens (SC-012); Given/When/Then acceptance features with `cucumber` 0.23.0 for the six stories.
- **Rationale**: every spec claim of the form "in 100% of tested cases" needs a test that can fail.

## Design patterns (ENG-01)

Ports and adapters (the three ports, with the store and fakes as adapters); Strategy for fragments (`BackupFragment` implemented per feature); Builder for assembling a backup archive and its read-me; State for the store's modes (closed, open, open read-only, refused); Observer for domain events feeding the log is handled inside the record operations (one transaction), not through a separate bus.

## Remaining open items (none block planning)

1. SQLCipher build route on Windows (system OpenSSL or vendored with a native Perl) and the macOS and Linux builds.
2. `keyring` per-platform backend features, and the owner's preference for the recovery-key presentation (base32 groups or words).
3. `cargo-deny` configuration for BSD-3-Clause, blessing and WTFPL.
4. Whether `sqlcipher-src`'s libtomcrypt option can avoid OpenSSL on Windows (not exposed by `libsqlite3-sys` today).
5. The exact text of the fixed READ-ME template, which the owner should review.
