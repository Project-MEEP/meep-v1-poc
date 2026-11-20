# Implementation Plan: Foundation (v1.0)

**Branch**: `002-foundation` (spec) / `002-plan-foundation` (this plan) | **Date**: 2026-11-19 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-foundation/spec.md`, the constitution v1.1.0, and the decision records in `docs/decisions/` (R1 to R17, FOUND-01 to FOUND-06, NAMES-01 to NAMES-06, BUILD-01, CONST-27, COMMIT-ID).

## Summary

The foundation gives every other feature a safe place to keep the owner's records: an encrypted local store that opens with a key held on the owner's own computer, a one-time recovery key, per-record keys so a deleted record is destroyed for good, a tamper-evident change history that never holds content, an owner backup and restore, and a local server reachable only from the owner's own machine (R2, R5, R7, R8, R9, R16, principle 27, FOUND-01 to FOUND-06).

Technical approach, all **Proposed** until the owner confirms each decision record (PLAN-13 to PLAN-27), drawn from Confirmed decisions at product level:

- A Rust workspace of four crates (core, storage, backup, web) with the store reached through three ports, `RecordStore`, `ChangeLog` and `KeyCustody` (FOUND-01), which names and the shared UI use.
- SQLCipher through `rusqlite` for the encrypted file (R2). A throwaway build on this PC verified the route on Windows: `bundled-sqlcipher-vendored-openssl` with Strawberry Perl and a short build-folder path (Windows long paths do not help); plain `bundled-sqlcipher` needs an OpenSSL the machine does not have (see research.md). CI gets a `windows-latest` build job so Windows-only failures are caught.
- A key hierarchy: a random store key in the operating-system keychain, a one-time recovery key that unwraps a stored copy of it, per-record keys wrapped under a subkey of the store key, and purpose-specific subkeys derived from each record key.
- Crypto-shredding by destroying a record's wrapped key inside one transaction, with `secure_delete`, a log checkpoint and a test that proves nothing is left (R8).
- A hash-chained change log with a canonical entry encoding, enforced append-only inside the store.
- One running copy enforced by an operating-system file lock from the Rust standard library, verified on this PC to be released when the holder is killed (FOUND-04).
- A damaged store is opened by working on a byte-for-byte copy, never the original, with a before-and-after byte comparison (FOUND-03).
- An owner backup as one zip archive written under a temporary name, verified, then renamed, with a self-contained read-me and a check that the published description matches (FOUND-05, FOUND-06).
- A loopback-only server shell with Host, Origin and per-session token checks (R5).

## Technical Context

**Language/Version**: Rust, stable channel (LANG-01). Installed toolchain: rustc 1.100.0. The standard-library file lock used for FOUND-04 needs a Rust version that includes `File::try_lock` (present in 1.100.0, verified).

**Primary Dependencies** (versions and licenses verified on crates.io 2026-11-19; see [research.md](research.md)): `rusqlite` 0.43.0 (feature `bundled-sqlcipher-vendored-openssl` on Windows) (via `libsqlite3-sys` 0.40.0 and `sqlcipher-src` 420.0.0, which is SQLCipher 4.20.0 on SQLite 3.54.1), `keyring` 4.2.0, `chacha20poly1305` 0.11.0, `hkdf` 0.13.0, `sha2` 0.11.1, `zeroize` 1.9.2, `rand` 0.10.4, `subtle` 2.6.1, `uuid` 1.29.3, `serde` and `serde_json` (see names plan), `serde_json_canonicalizer` 0.4.1, `time` 0.3.55, `data-encoding` 2.11.1, `zip` 9.8.0, `schemars` 1.2.3, `jsonschema` 0.61.2 (tests), `axum` 0.8.9 with `tokio` 1.54.1, `tower-http` 0.7.1, `cookie` 0.18.2, `tempfile` 3.27.0. Test-only: `proptest` 1.12.0, `cucumber` 0.23.0. The one-copy lock uses only the standard library.

**Storage**: One SQLCipher-encrypted database file plus its write-ahead log and a separate lock file, in a folder owned by the owner's operating-system account. Layout in [data-model.md](data-model.md).

**Testing**: `cargo test` unit and property tests; crash and fault injection through failpoint hooks and child-process kills; two-process lock tests; raw-byte destruction scans; golden-file checks for the published format description; `cucumber` for the Given/When/Then scenarios in spec.md.

**Target Platform**: Windows first on this PC, plus macOS and Linux through the same crates; the web client first, with an Android wrapper later (R1, R3), so `meep-core`, `meep-storage` and `meep-backup` stay free of web types.

**Project Type**: Rust workspace, three libraries plus one web application crate.

**Performance Goals**: Single owner and small data (names are capped at 50). Opening the store, saving, deleting, verifying the log and making a backup complete without a perceptible delay on an ordinary computer. No throughput targets.

**Constraints**: Offline only; no outside network calls (FR-027); loopback binding only; deleted records unrecoverable from anything Meep manages (FR-010); the damaged-store file never modified (FR-029); no real personal data in the repository (constitution).

**Scale/Scope**: One owner, hundreds of records at most in v1.0, a change log that grows with use and is never shortened (DATA-01 is Open and is a planning matter only for size), one backup format version.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design: see the end of this section.*

| Principle | Gate | Status |
|---|---|---|
| 1, 2 (one human, owner owns data) | Single owner, no accounts; the account ID is random and the same on every record | Pass |
| 3 to 5, 14 (roles, delegation, peer-to-peer) | None built; entry conditions recorded for later (NAMES-01, NAMES-02) | Pass (out of scope) |
| 6, 7 (domain addressing, repositories) | Shared address checker (R16); repositories are data-agnostic behind `RecordStore` | Pass |
| 10 to 12 (AI skin, vendor-neutral, self-hosted) | No AI, no vendor assumed; no outside services | Pass |
| 13 (replaceable workers, durable state) | Durable state lives in one store; the lock is released automatically on a crash | Pass |
| 15 (secrets isolated, Proposed) | Keys never logged or shown; the recovery key shown once and never stored | Pass |
| 16 (encryption at rest and in transit, audit logs, secure SDLC) | Encrypted store, hash-chained log, loopback only so no transit exposure; no certification claimed | Pass |
| 18 (free and open source, no mandatory paid cloud) | All dependencies are MIT, Apache-2.0 or BSD-3-Clause family; `cargo-deny` allow-list must add BSD-3-Clause and the SQLite blessing and WTFPL notices carried by `sqlcipher-src` (see research.md) | Pass, with a deny.toml action |
| 19, 20 (no clinical claims, import and attribute) | The envelope says `authority: none` on every record | Pass |
| 22 (BYOx) | Standard-library lock, existing crates for crypto, zip and canonical JSON; nothing rebuilt | Pass |
| 23 (specs, requirements, engineering separate) | Spec stays technology-agnostic; technology lives here | Pass |
| 25 (Spec Kit order) | constitution, specify, clarify done; plan now; checklist, tasks, analyze, implement follow | Pass |
| 26 (warn, never adjudicate) | Foundation blocks only for data integrity and authorization-by-location (loopback) | Pass |
| 27 (no lock-in, Confirmed) | Owner backup is a plain, open, documented zip with its own read-me (FOUND-05) | Pass |
| Engineering (ENG-01 to ENG-03, LANG-01, EGRESS-01) | Rust; GoF patterns mapped in research; tests first; pull requests only; no egress | Pass |
| No personal data in repo | Synthetic fixtures only | Pass |
| Build order (BUILD-01) | This feature is first; names and shared UI consume it through the ports | Pass |

No violations, so **Complexity Tracking** is empty.

**Re-check after Phase 1 design**: Pass. The design adds no network code, keeps keys out of logs and messages, and adds one crate (`meep-backup`) beyond the names plan only because backup logic reads the store through ports and is easiest to test on its own.

## Project Structure

### Documentation (this feature)

```text
specs/002-foundation/
├── plan.md                         # This file
├── research.md                     # Phase 0: decisions, rationale, verification log, spike results
├── data-model.md                   # Phase 1: stored entities, key hierarchy, state
├── quickstart.md                   # Phase 1: how to validate end to end
├── contracts/                      # Phase 1
│   ├── ports.md                    # RecordStore, ChangeLog, KeyCustody
│   ├── change-log-entry.md         # entry format, chain, kinds
│   ├── backup-archive-format.md    # zip layout, manifest, version, read-me sources
│   ├── owner-messages.md           # every owner-facing message with Fluent IDs
│   └── local-access.md             # loopback binding, Host/Origin, token
└── tasks.md                        # Phase 2 (/speckit-tasks, not created here)
```

### Source Code (repository root)

```text
Cargo.toml                          # workspace
crates/
├── meep-core/                      # pure domain, no I/O
│   └── src/                        #   ports.rs, envelope.rs, address.rs, ids.rs,
│                                   #   change_log.rs (entry model, chain, verify), fragment.rs
│                                   #   (BackupFragment trait), messages.rs (message IDs), testing.rs
├── meep-storage/                   # encrypted store, key custody, lock, damaged-store open
│   └── src/                        #   store.rs, keys.rs, keychain.rs, recovery.rs, shred.rs,
│                                   #   lock.rs, damaged.rs, restore.rs
├── meep-backup/                    # owner backup writer/reader, read-me, schemas, verification
│   ├── src/
│   └── templates/                  #   READ-ME-FIRST.txt (golden, placeholders)
└── meep-web/                       # server shell (foundation) and, later, screens (specs 010, 001)
    ├── src/                        #   server.rs, middleware.rs, token.rs, i18n.rs
    └── locales/en/foundation.ftl   #   all foundation messages (R10)
docs/format/                        # published backup format description (golden copy)
tests/
├── acceptance/                     # cucumber features for spec 002
├── crash/                          # child-process kill and failpoint tests
└── fixtures/                       # synthetic records, damaged-store samples
docs/decisions/                     # PLAN-13 to PLAN-27 (Proposed)
```

**Structure Decision**: four crates. `meep-core` holds the ports and the pure rules so names and the shared UI can be tested with fakes (BUILD-01 lets domain work proceed before the foundation lands). `meep-storage` owns everything that touches the file, the keychain and the lock. `meep-backup` is separate because it only needs the ports and the zip and schema crates, which keeps the store crate small and lets backup tests run without a database. `meep-web` carries the foundation's server shell; spec 010 and spec 001 add screens to it later. The workspace is created at implementation time, not by this plan.

## Impacts on other specs (not edited here)

Spec 001's task list should be adjusted when it is next revised, because ownership moved to the foundation (FOUND-01, FOUND-02, BUILD-01):

- **T022, T023** (loopback server and anti-forgery) become foundation work (PLAN-25); names only consumes the shell.
- **T049, T050, T054, T055** keep their names-side logic but run against the foundation's real key custody, store and shredding once they land; the primitives and the raw-byte destruction test are foundation tasks (PLAN-15, PLAN-16).
- **T026, T027** (keyring backends and SQLCipher build) are answered by this plan's spikes and research; they can be closed after the owner reviews PLAN-14 and PLAN-15.
- **T012** (fakes) can reuse the in-memory test doubles this plan places in `meep-core` (`testing.rs`).

## Complexity Tracking

No constitution violations. Nothing to justify.
