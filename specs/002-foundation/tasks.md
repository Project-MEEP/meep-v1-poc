---

description: "Task list for feature 002: Foundation (v1.0)"
---

# Tasks: Foundation (v1.0)

**Input**: Design documents from `specs/002-foundation/` (spec.md, plan.md, research.md, data-model.md, contracts/, quickstart.md) and `docs/decisions/`

**Prerequisites**: plan.md and spec.md (present); research.md, data-model.md, contracts/ and quickstart.md (present)

**Tests**: REQUIRED. ENG-02 and the spec's Verification section make tests mandatory. In every story the test tasks (unit, property, failpoint crash, two-process, raw-byte) come first and must fail before the code they cover is written (Red, then Green).

**Organization**: grouped by the six user stories in spec.md. The one-running-copy rule is in User Story 1; the start-up check after an unexpected stop is in User Story 4 (it needs log verification); damaged-store handling and rescue backups are in User Story 5.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: US1 to US6; none for Setup, Foundational, Open items and Polish
- Every task names its file path and cites the spec requirement (FR, SC) or decision (R, FOUND, PLAN, NAMES, BUILD) it serves
- **[UNBLOCKS 001/010]**: later specs wait for this task (BUILD-01 order: this feature, then spec 010, then spec 001 screens)
- **[OWNER QUESTION]**: needs the owner's decision
- Field citations: `data-model.md entity.field` points at the canonical constraint; tasks do not repeat it

## Path Conventions

Cargo workspace from plan.md: `crates/meep-core/`, `crates/meep-storage/`, `crates/meep-backup/`, `crates/meep-web/`, `tests/acceptance/`, `tests/fixtures/`, `docs/format/`, `docs/decisions/`. All test data is synthetic; no real personal data and no email addresses anywhere (constitution). **Windows**: keep the repository and `target` folder path at 121 characters or fewer, Strawberry Perl on PATH, vendored OpenSSL feature (research.md).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: workspace, tooling, CI wiring, fixtures, published-format skeleton. No behavior yet.

- [ ] T001 Create the workspace manifest `Cargo.toml` at the repository root with members `crates/meep-core`, `crates/meep-storage`, `crates/meep-backup`, `crates/meep-web`, `tests/acceptance`; pin edition and `rust-version`; set `[workspace.lints]` so clippy warnings are errors; keep the repository and its `target` folder path at 121 characters or fewer (research.md, Spike status) (PLAN-13, ENG-01)
- [ ] T002 [P] Create `crates/meep-core/Cargo.toml` and `src/lib.rs`: dependencies `serde`, `serde_json`, `serde_json_canonicalizer`, `sha2`, `thiserror`, `uuid`, `rand`, `time`, `zeroize`; no I/O, web or database crates; a `testing` cargo feature for the fakes and a `failpoints` feature for the crash hooks (PLAN-13, PLAN-17, PLAN-18, PLAN-24)
- [ ] T003 [P] Create `crates/meep-storage/Cargo.toml` and `src/lib.rs`: `rusqlite` 0.43.0 with `bundled-sqlcipher-vendored-openssl` on Windows (target-specific dependency) and `bundled-sqlcipher` elsewhere pending the macOS and Linux check, `keyring` 4.2.0, `chacha20poly1305`, `hkdf`, `sha2`, `zeroize`, `rand`, `subtle`, `data-encoding`, `tempfile`, path dependency on `meep-core`; a `plaintext-test-store` feature (tests only) (PLAN-14, PLAN-15, PLAN-16)
- [ ] T004 [P] Create `crates/meep-backup/Cargo.toml` and `src/lib.rs`: `zip` 9.8.0, `schemars` 1.2.3, `jsonschema` 0.61.2 (dev), `serde_json`, `sha2`, `time`, path dependency on `meep-core` only (PLAN-13, PLAN-21)
- [ ] T005 [P] Create `crates/meep-web/Cargo.toml` and `src/main.rs`: `axum` 0.8.9, `tokio` 1.54.1, `tower-http` 0.7.1, `cookie` 0.18.2, `fluent-templates`, path dependencies on core and storage; no screens (specs 010 and 001 add them) (PLAN-13, PLAN-25)
- [ ] T006 [P] Create `tests/acceptance/Cargo.toml` with `cucumber` 0.23.0 and `proptest` 1.12.0, a `[[test]] name = "acceptance"` entry with `harness = false`, and path dependencies on core, storage and backup (PLAN-27, ENG-02)
- [ ] T007 [P] Create `deny.toml` for `cargo-deny`: allow MIT, Apache-2.0, Unicode-3.0 and BSD-3-Clause; leave the SQLite "blessing" and WTFPL terms from `sqlcipher-src` as flagged findings for the owner question task; deny known vulnerabilities; **ban network-client crates** (for example `reqwest`, `hyper-tls`, `ureq`) so FR-027 (no outside network request) is enforced by the dependency check (principle 18, FR-027, research.md)
- [ ] T008 [P] Create `rustfmt.toml` and commit `Cargo.lock` (application workspace) (ENG-01)
- [ ] T009 [P] Create `tests/fixtures/records.json` (synthetic record A at `Alpha.Meep.Personal.Identity.Names` with content "synthetic sample A" and record B at `Alpha.Meep.Personal.Notes.Sample` with content "synthetic sample B"), `tests/fixtures/addresses.json` (valid and invalid addresses from data-model.md: four and six segments, a leading digit, an underscore, a space, mixed case) and `tests/fixtures/README.md` stating synthetic-only; confirm `.github/scripts/guard.sh` passes (R16, FR-008, constitution)
- [ ] T010 [P] Create the published format description skeleton: `docs/format/READ-ME-FIRST.txt` (fixed text with placeholders for format version, as-of time, feature sections, and rescue-only sections) and `docs/format/fragments/README.md`; this is the golden copy the backup writer renders and the format-docs check compares against (FR-037, FR-040, PLAN-22)
- [ ] T011 [P] Create `scripts/check-windows-prereqs.ps1`: reports whether Strawberry Perl is on PATH, the C++ tools are present, and the `target` folder path length is 121 characters or fewer, with the fixed explanation from research.md for each failure (PLAN-14)
- [ ] T012 Verify and adjust `.github/workflows/ci.yml` for the real workspace: `lint`, `deps`, `unit` and `build-windows` run for real, the acceptance job runs `cargo test -p meep-acceptance --test acceptance --no-fail-fast`, the crash and format-docs tests run inside `cargo test --workspace`, every third-party action stays pinned to a full commit hash, and `all-green` still requires every job (ENG-02, ENG-03, PLAN-22, PLAN-27)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: the pieces every story and the later specs need. Ports and fakes come first so specs 001 and 010 can proceed (BUILD-01). Tasks marked **[UNBLOCKS 001/010]** are what those specs wait for. No story starts until this phase is done.

- [ ] T013 Define the error types in `crates/meep-core/src/errors.rs` for every failure listed in contracts/ports.md (`AddressInvalid{reason}`, `StoreReadOnly`, `StoreFailed`, `Stale`, `NotFound`, `Unreadable{id}`, `KindExists`, `UnknownKind`, `DetailRefused{key}`, `ChainBroken{first_seq}`, `RecordKeyDestroyed`) with a one-to-one mapping to message IDs (FR-025, PLAN-26)
- [ ] T014 [P] Write a failing test then implement the injectable clock in `crates/meep-core/src/clock.rs` (UTC, RFC 3339, a fixed test clock); used for data-model.md store_header.created_at, records.created_at, records.updated_at, change_log.ts (PLAN-24)
- [ ] T015 [P] Write a failing test then implement the feature-gated failpoint macro in `crates/meep-core/src/failpoint.rs` (a named point that can return an error or terminate the process, compiled out without the `failpoints` feature) (PLAN-18, PLAN-27)
- [ ] T016 [P] Create the message IDs in `crates/meep-core/src/messages.rs` and the English catalog `crates/meep-web/locales/en/foundation.ftl` with every ID and exact text from contracts/owner-messages.md; add tests that assert the two owner-specified texts (`store-already-open`, `backup-changes-after`) exactly and a lint test that fails if foundation code produces owner-visible text outside the catalog (R10, FR-035, FR-042, PLAN-26)
- [ ] T017 [P] Write failing tests then implement `crates/meep-core/src/ids.rs`: random UUID v4 helpers for the values in data-model.md store_header.store_id, store_header.account_id and records.id, using a cryptographically secure random source; tests for format and uniqueness (R9, FR-007, PLAN-24)
- [ ] T018 Define the ports in `crates/meep-core/src/ports.rs` exactly as contracts/ports.md: `RecordStore`, `ChangeLog` (including `register_kind`), `KeyCustody` and the `BackupFragment` trait in `fragment.rs`; no operation returns key material (FOUND-01, FR-025, principle 15) **[UNBLOCKS 001/010]**
- [ ] T019 Write failing tests in `crates/meep-core/tests/fakes.rs` that hold the in-memory fakes to the same rules as the real store (envelope on every record, address checked, stale update refused, delete makes content unreadable, log append-only with registered kinds and no content, `StoreReadOnly` on writes in read-only mode) (BUILD-01, FR-006, FR-008, FR-014)
- [ ] T020 Implement the in-memory fakes of the three ports in `crates/meep-core/src/testing.rs` behind the `testing` cargo feature so specs 001 and 010 can be built and tested before the real store lands (BUILD-01, FOUND-01) **[UNBLOCKS 001/010]**
- [ ] T021 [P] Write failing tests in `crates/meep-core/tests/change_log_entry.rs` for the entry body and its canonical encoding: RFC 8785 canonical JSON gives identical bytes for reordered members, `hash` is SHA-256 of those bytes, `prev_hash` is inside the body, the genesis entry has seq 0 and 64 zeros, `hash` is lowercase hex; cite data-model.md change_log.seq, change_log.prev_hash, change_log.hash (FR-013, PLAN-17)
- [ ] T022 Implement the entry model, canonical encoding (`serde_json_canonicalizer`) and hash computation in `crates/meep-core/src/change_log.rs`; cite data-model.md change_log.seq, change_log.ts, change_log.record_id, change_log.kind, change_log.details, change_log.prev_hash, change_log.hash (FR-013, FR-014, PLAN-17)
- [ ] T023 [P] Create the test harness in `crates/meep-storage/tests/support/`: a temporary store directory helper, a child-process crash harness (a small binary `crates/meep-storage/src/bin/crash_child.rs` built with the `failpoints` feature that runs one operation and is terminated at a named failpoint), and a two-process helper for the lock tests (PLAN-27, FR-015, FR-024)
- [ ] T024 [P] Create the raw-byte scan helper in `crates/meep-storage/tests/support/scan.rs`: builds the same code path against an unencrypted database (feature `plaintext-test-store`, tests only), plants sentinel content and a known fake key, and scans the database file, its write-ahead log and its free pages; used by the destruction tests (SC-002, FR-010, PLAN-16)
- [ ] T025 Write failing tests in `crates/meep-storage/tests/log_store.rs` for the stored log: `change_log` rows hold exactly the entry body, triggers refuse any update or delete of a row, append happens inside the caller's transaction and a rolled-back transaction leaves no entry; cite data-model.md change_log.seq, change_log.prev_hash, change_log.hash (FR-013, FR-014, FR-015)
- [ ] T026 Implement the stored log in `crates/meep-storage/src/log_store.rs`: the `change_log` table, the refusing triggers, append inside a transaction with the previous hash read in the same transaction, and the genesis entry written at store creation; cite data-model.md change_log.seq, change_log.kind, change_log.prev_hash, change_log.hash (FR-013, FR-014, PLAN-17, PLAN-18)

---

## Phase 2b: Open items and owner questions

**Purpose**: the open items from research.md and plan.md. Items marked **[OWNER QUESTION]** need your decision; they do not block the foundational work, but the recovery-key presentation blocks the first-run screen text in User Story 1.

- [ ] T027 [P] Spike: confirm the `keyring` 4.2.0 backend features needed on Windows with a throwaway test that stores, reads and deletes a synthetic secret, and document what macOS and Linux need; record the result in `docs/decisions/PLAN-15.md` and the research.md verification log (PLAN-15, FR-005, research.md open items 1 and 2)
- [ ] T028 [P] **[OWNER QUESTION]** Ask the owner how the one-time recovery key is presented: base32 groups (the plan's default, 52 characters in groups of four) or a word list; record the answer as a decision in `docs/decisions/` before the first-run text in User Story 1 is finalized (FR-002, PLAN-15, research.md D3)
- [ ] T029 [P] **[OWNER QUESTION]** Run `cargo deny check licenses` once the workspace builds, list what `sqlcipher-src` carries (BSD-3-Clause, the SQLite blessing notice, WTFPL), and ask the owner to approve any reviewed exceptions in `deny.toml` (principle 18, PLAN-14, research.md open item 3)
- [ ] T030 [P] Add `build-macos` (macos-latest) to `.github/workflows/ci.yml` and the `all-green` gate, confirm the Linux build through the existing `unit` job, and record the macOS and Linux SQLCipher results in `docs/decisions/PLAN-14.md`; keep actions pinned by commit hash (PLAN-14, research.md open item 1)

---

## Phase 3: User Story 1 - My information is kept in an encrypted store only I can open (Priority: P1) 🎯 MVP

**Goal**: create and open the encrypted store with a key held on the owner's computer and a one-time recovery key; allow one running copy at a time.

**Independent Test**: create a store, close it, then try to open it with no key, a wrong key, the recovery key and the normal key; only the last two succeed (quickstart scenarios 1 to 4, 19). The unexpected-stop check is in User Story 4 because it needs log verification.

- [ ] T031 [P] [US1] Write failing tests in `crates/meep-storage/tests/open_keys.rs`: a store refuses to open with no key and with a wrong key and reveals nothing about its contents, and opens with the normal key and with the recovery key (FR-001, FR-003, FR-004, SC-001, R2)
- [ ] T032 [P] [US1] Write a failing test in `crates/meep-storage/tests/file_unreadable.rs`: the raw bytes of the store file contain none of the synthetic content, and a copy of the file in another folder cannot be opened without the key (FR-001, R2)
- [ ] T033 [P] [US1] Write failing tests in `crates/meep-storage/tests/first_run.rs`: first run creates the store and returns a recovery key once; the recovery key is absent from every file Meep manages and from the keychain after display; the next start opens with no typing (FR-002, FR-003, R5, R7, FOUND-02)
- [ ] T034 [P] [US1] Write failing tests in `crates/meep-storage/tests/lost_keys.rs`: with the normal key missing the recovery key opens the store; with both missing the store stays closed with the `store-lost-both-keys` message and no bypass of any kind exists (FR-004, R7)
- [ ] T035 [P] [US1] Write failing tests in `crates/meep-storage/tests/key_hierarchy.rs`: HKDF labels (`meep/db-key/v1`, `meep/record-wrap/v1`, `meep/recovery-wrap/v1`) give distinct keys, derivation is deterministic, secrets are zeroized on drop and never appear in `Debug`, logs or messages (FR-005, principle 15, PLAN-15)
- [ ] T036 [P] [US1] Write failing tests in `crates/meep-storage/tests/store_header.rs` for data-model.md store_header.store_id, store_header.account_id, store_header.format_version, store_header.recovery_wrapped_store_key, store_header.recovery_nonce, store_header.created_at: formats, a fresh nonce on every wrap, a format version newer than supported is refused (FR-007, R7, R9)
- [ ] T037 [P] [US1] Write failing two-process tests in `crates/meep-storage/tests/lock.rs`: a second program is refused while the first holds the store, a hard-killed first program releases it, and the first program is never asked to prove ownership; also an unreadable lock state refuses with `store-already-open` and never asks the owner to confirm (FR-024, FR-035, SC-013, FOUND-04)
- [ ] T038 [US1] Implement the one-running-copy lock in `crates/meep-storage/src/lock.rs` using the standard library's `File::try_lock` on `store.lock`, holding the handle for the life of the process and returning held, acquired or cannot-tell (refuse) (FR-024, FR-035, PLAN-19, FOUND-04)
- [ ] T039 [US1] Implement the keychain wrapper in `crates/meep-storage/src/keychain.rs` over `keyring` 4.2.0 for the store key, entry named by data-model.md store_header.store_id (FR-005, R7, PLAN-15)
- [ ] T040 [US1] Implement the HKDF subkey derivation and secret handling in `crates/meep-storage/src/keys.rs` (`zeroize`, `subtle`): store key to database key, record-wrapping key and recovery wrap key per data-model.md key hierarchy (FR-005, FR-009, PLAN-15)
- [ ] T041 [US1] Implement the recovery key in `crates/meep-storage/src/recovery.rs`: generate 256 random bits, encode per the owner's answer to the presentation question, derive the wrap key from the key and the store id, wrap and unwrap a copy of the store key; cite data-model.md store_header.recovery_wrapped_store_key and store_header.recovery_nonce; no replacement operation exists (FR-002, FR-004, FOUND-02, PLAN-15)
- [ ] T042 [US1] Implement store create and open in `crates/meep-storage/src/store.rs`: SQLCipher with a raw key derived from the store key, WAL mode, `synchronous=FULL`, `secure_delete=ON`, the `store_header` table written at creation; cite data-model.md store_header.store_id, store_header.account_id, store_header.format_version, store_header.created_at (FR-001, FR-003, PLAN-14, PLAN-18)
- [ ] T043 [US1] Wire the first-run, recovery, wrong-key, lost-keys and already-open outcomes to the message IDs in `crates/meep-storage/src/startup.rs`, including the one-time display element for the recovery key (FR-002, FR-004, FR-035, PLAN-26)
- [ ] T044 [US1] Run the acceptance feature `tests/acceptance/features/store_and_keys.feature` (Given/When/Then for Story 1 scenarios 1 to 6 and 11 to 12) and quickstart scenarios 1 to 4 and 19; fix until green (ENG-02)

---

## Phase 4: User Story 2 - Every record carries the same standard envelope (Priority: P1)

**Goal**: features save, read, update and list records that all carry the same envelope, at addresses the shared checker accepts.

**Independent Test**: save a synthetic record and confirm every envelope field is correct; save one at an invalid address and confirm it is refused (quickstart scenarios 5 and 6). Depends on the foundational log core, because every save appends a `record_created` entry.

- [ ] T045 [P] [US2] Write failing tests in `crates/meep-core/tests/address.rs` from `tests/fixtures/addresses.json`: exactly five segments, each starting with a letter and holding only letters, digits and hyphens; each invalid case refused with the reason; two addresses differing only in case compare equal while the written case is kept (FR-008, R16, SC-008)
- [ ] T046 [P] [US2] Write failing tests in `crates/meep-storage/tests/envelope.rs` (a property test with `proptest`): every saved record carries id, account ID, address, provenance, `authority` equal to `none` and created and last-changed timestamps; the account ID is the same on every record; provenance is one of entered, imported, restored; cite data-model.md records.id, records.address, records.provenance, records.authority, records.created_at, records.updated_at (FR-006, FR-007, R9)
- [ ] T047 [P] [US2] Write failing tests in `crates/meep-storage/tests/record_crypto.rs`: content is encrypted with the record's own key; moving the ciphertext to another record id or address makes it unreadable; each record has a different key; cite data-model.md records.content_ct, records.content_nonce, record_keys.record_id, record_keys.wrapped_key, record_keys.wrap_nonce (FR-009, PLAN-15)
- [ ] T048 [P] [US2] Write failing tests in `crates/meep-storage/tests/record_ops.rs`: save, read, list (with an address prefix), update with `expected_updated_at` (a changed record returns `Stale`), `NotFound`, `StoreReadOnly`; a failure at any failpoint during a save leaves no record, no key and no log entry (FR-015, SC-005, ports contract)
- [ ] T049 [US2] Implement the address checker in `crates/meep-core/src/address.rs` per R16 with the refusal reasons mapped to `address-invalid-segments` and `address-invalid-segment`; used for data-model.md records.address (FR-008, R6, R16, PLAN-24) **[UNBLOCKS 001/010]**
- [ ] T050 [US2] Implement the envelope in `crates/meep-core/src/envelope.rs`: all fields and the rule that `authority` is always `none`; cite data-model.md records.id, records.provenance, records.authority, records.created_at, records.updated_at (FR-006, FR-007)
- [ ] T051 [US2] Implement per-record keys in `crates/meep-storage/src/records.rs`: a random 256-bit key per record wrapped under the record-wrapping subkey with the record id as associated data; cite data-model.md record_keys.record_id, record_keys.wrapped_key, record_keys.wrap_nonce (FR-009, PLAN-15)
- [ ] T052 [US2] Implement `RecordStore` in `crates/meep-storage/src/records.rs`: save, update, read, list, each one transaction holding the record, its key and its log entry; content encrypted with XChaCha20-Poly1305 binding the record id and address; cite data-model.md records.address, records.content_ct, records.content_nonce, records.created_at, records.updated_at (FR-006, FR-009, FR-015, FOUND-01, PLAN-18) **[UNBLOCKS 001]**
- [ ] T053 [US2] Run the acceptance feature `tests/acceptance/features/envelope.feature` (Story 2 scenarios 1 to 6) and quickstart scenarios 5 and 6; fix until green (ENG-02)

---

## Phase 5: User Story 3 - Deleting a record destroys it for good (Priority: P1)

**Goal**: deleting a record destroys its content and its key everywhere Meep manages, while the history keeps only "a record was deleted" and the date.

**Independent Test**: save a record with sentinel content, delete it, then scan all of Meep's storage for the content and the key (quickstart scenarios 7 to 9).

- [ ] T054 [P] [US3] Write failing tests in `crates/meep-storage/tests/delete.rs`: after delete the content cannot be read by any feature; the log shows only `record_deleted`, the record id and the date; deleting an unknown id returns `NotFound`; a read-only store refuses (FR-010, FR-011, R8)
- [ ] T055 [P] [US3] Write failing raw-byte tests in `crates/meep-storage/tests/destruction_scan.rs` using the scan helper: after delete, neither the sentinel content nor the fake record key appears in the database file, its write-ahead log or its free pages; the encrypted store has no key row for the record; the log is checkpointed and truncated (SC-002, FR-010, PLAN-16)
- [ ] T056 [P] [US3] Write failing tests in `crates/meep-storage/tests/purpose_value.rs`: a purpose value is stable for the same record, purpose and input, differs by purpose, is non-reversible, and after the record key is destroyed the call returns `RecordKeyDestroyed` and no earlier value can be recomputed (FR-012, NAMES-02, PLAN-15)
- [ ] T057 [P] [US3] Write failing crash tests in `crates/meep-storage/tests/crash_delete.rs`: terminating the process at each failpoint during a delete leaves either the full record or nothing, never a half-deleted state, and never a log entry without the deletion (FR-015, SC-005)
- [ ] T058 [US3] Implement `KeyCustody::purpose_value` and `destroy_record_key` in `crates/meep-storage/src/keys.rs`: HKDF of the record key with `meep/purpose/<purpose>`; `destroy_record_key` is used only by delete and restore; cite data-model.md record_keys.wrapped_key (FR-012, FR-025, PLAN-15)
- [ ] T059 [US3] Implement delete with crypto-shredding in `crates/meep-storage/src/shred.rs`: one transaction that overwrites the wrapped-key row with zeros, deletes it, deletes the record and appends `record_deleted`, then checkpoints and truncates the write-ahead log, with `secure_delete` on; cite data-model.md record_keys.wrapped_key, record_keys.wrap_nonce, records.content_ct, change_log.kind (FR-009, FR-010, FR-011, R8, PLAN-16) **[UNBLOCKS 001]**
- [ ] T060 [US3] Run the acceptance feature `tests/acceptance/features/deletion.feature` (Story 3 scenarios 1 to 5) and quickstart scenarios 7 to 9; document in `docs/` the limits stated in FR-010 (copies outside Meep and earlier backups cannot be recalled); fix until green (SC-002)

---

## Phase 6: User Story 4 - Every change is recorded in a tamper-evident history (Priority: P2)

**Goal**: verification, the registry of kinds and allowed details, refusal of content, correct ordering, and the start-up check after an unexpected stop.

**Independent Test**: append entries, verify, then alter, remove or reorder one and confirm verification fails and names the first broken entry (quickstart scenarios 10, 11, 20).

- [ ] T061 [P] [US4] Write failing tests in `crates/meep-storage/tests/log_verify.rs`: verification passes and reports the count; editing a body, deleting an entry, reordering entries or leaving a gap in `seq` each fail naming the first broken `seq`; cite data-model.md change_log.seq, change_log.prev_hash, change_log.hash (FR-013, SC-003)
- [ ] T062 [P] [US4] Write property tests in `tests/acceptance/tests/chain_property.rs` (`proptest`): any generated sequence verifies, and any single mutation is caught at the first affected entry (FR-013, PLAN-27)
- [ ] T063 [P] [US4] Write failing tests in `crates/meep-core/tests/log_registry.rs`: a feature registers a kind with its allowed detail keys; an unknown kind or an unregistered detail key is refused; values that look like content or key material are refused with `DetailRefused`; a later change or deletion of an entry is refused (FR-014, FOUND-01, R8)
- [ ] T064 [P] [US4] Write failing concurrency tests in `crates/meep-storage/tests/log_order.rs`: simultaneous appends from two threads give one consistent `seq` order and a valid chain (FR-013, spec edge cases)
- [ ] T065 [US4] Write failing crash-and-restart tests in `crates/meep-storage/tests/unexpected_stop.rs`: a process killed while the store is open leaves no marker to clear, and the next start checks that the store is intact and the chain verifies before opening normally; if either fails the damaged path in User Story 5 applies (FR-034, SC-013, FOUND-04) **[needs log verification]**
- [ ] T066 [US4] Implement `ChangeLog::verify` and `read` in `crates/meep-storage/src/log_store.rs`, returning the count or `ChainBroken{first_seq}`; cite data-model.md change_log.seq, change_log.prev_hash, change_log.hash (FR-013, PLAN-17)
- [ ] T067 [US4] Implement the kind registry and detail validation in `crates/meep-core/src/change_log.rs` with the v1.0 foundation kinds from contracts/change-log-entry.md; cite data-model.md change_log.kind, change_log.details (FR-014, FOUND-01) **[UNBLOCKS 001]**
- [ ] T068 [US4] Implement the start-up sequence in `crates/meep-storage/src/startup.rs`: take the lock, open, run the integrity check and chain verification, then open normally or hand over to damaged-store handling (FR-034, FR-024, FOUND-04, FOUND-03)
- [ ] T069 [US4] Map `ChainBroken` to `history-does-not-verify` (naming the first broken entry and the likely causes without implying an intruder) and unreadable data to `store-damaged-readonly` in `crates/meep-storage/src/startup.rs` (FR-033, FOUND-03, PLAN-26)
- [ ] T070 [US4] Run the acceptance feature `tests/acceptance/features/change_log.feature` (Story 4 scenarios 1 to 8) and quickstart scenarios 10, 11, 20 and 24; fix until green (ENG-02)

---

## Phase 7: User Story 5 - I can take a full backup, restore it, and rescue a damaged store (Priority: P2)

**Goal**: the owner backup and restore, the rescue backup and read-only handling of a damaged store, and the published format description.

**Independent Test**: save records from two features, make an owner backup, restore it into an empty store, and confirm every record matches; then try a non-empty store, a damaged backup and a damaged store (quickstart scenarios 12 to 18, 21 to 24, 26). Parts 5a to 5d can be built in order inside this phase.

- [ ] T071 [P] [US5] Write failing tests in `crates/meep-backup/tests/archive.rs` (5a, backup): the archive is one zip containing `READ-ME-FIRST.txt`, `manifest.json` (format version, as-of, kind, account ID, features with counts, SHA-256 of every other entry), `data/<fragment>.json` and `schema/<fragment>.schema.json` plus `schema/manifest.schema.json`; data validates against its schema (FR-016, FR-036, PLAN-21)
- [ ] T072 [P] [US5] Write failing tests in `crates/meep-backup/tests/readme.rs`: the rendered read-me contains every required topic (what Meep is, what each part holds, how fields map to the standard name formats from each feature's section, how to restore, that the file is not encrypted, the as-of moment, the format version), is plain text with no jargon list violations, and a rescue backup's read-me also lists what could not be read and explains the unverified history archive (FR-037, FR-038, FOUND-02, FOUND-05)
- [ ] T073 [P] [US5] Write failing tests in `crates/meep-backup/tests/write_verify.rs`: a backup is written under a `.partial` name, verified (archive readable, every file's SHA-256 matches, every record present, read-me present), flushed and renamed; a failure at any step leaves no file with a final name and returns `backup-failed`; `data-model.md store_header.last_successful_backup_at` is written only after the rename (FR-043, FR-044, SC-015, FOUND-06)
- [ ] T074 [P] [US5] Write failing tests in `crates/meep-storage/tests/backup_snapshot.rs`: saves made while a backup runs are not delayed and are not in the backup, which shows the store as of its start; after the backup, the "changes after <time>" information is available exactly when a later change exists; a multi-record change is either wholly in or wholly out (FR-018, FR-041, FR-042, SC-016)
- [ ] T075 [P] [US5] Write failing crash tests in `crates/meep-storage/tests/crash_backup.rs`: terminating the process at every failpoint during a backup leaves either a verified backup or no backup-named file, and saves made during the backup survive (SC-015, FR-043)
- [ ] T076 [P] [US5] Write failing tests in `crates/meep-backup/tests/version.rs`: the manifest carries the format version; a newer major version is refused with `restore-version-too-new` and nothing is guessed; a newer minor version with the same major is read and unknown members are ignored (FR-039, FOUND-05)
- [ ] T077 [US5] Implement the fragment collection and the manifest in `crates/meep-backup/src/manifest.rs` and `fragment.rs` over the `BackupFragment` trait: export within one read snapshot, schema per fragment (`schemars`), counts and checksums (FR-016, FR-036, FOUND-01, PLAN-21) **[UNBLOCKS 001]**
- [ ] T078 [US5] Create the READ-ME template `crates/meep-backup/templates/READ-ME-FIRST.txt` and the renderer `crates/meep-backup/src/readme.rs` that fill placeholders and insert each feature's section; the fixed text equals `docs/format/READ-ME-FIRST.txt` (FR-037, FR-040, PLAN-22)
- [ ] T079 [US5] **[OWNER QUESTION]** Ask the owner to review the exact text of `crates/meep-backup/templates/READ-ME-FIRST.txt` (plain language, no jargon, what each part holds, how to restore, that the file is not encrypted) before it is treated as final (FR-037, FOUND-05, research.md open item 5)
- [ ] T080 [US5] Implement the backup writer in `crates/meep-backup/src/writer.rs` and `verify.rs`: temporary name, full verification, flush, rename, then record data-model.md store_header.last_successful_backup_at and append `backup_made`; the `backup-not-encrypted-warning` and `backup-made` messages are produced here (FR-023, FR-043, FR-044, PLAN-21, FOUND-06)
- [ ] T081 [US5] Implement the as-of snapshot in `crates/meep-storage/src/records.rs` (one read transaction for the whole export) and the post-backup check that returns the time for `backup-changes-after` when a change is newer than as-of (FR-018, FR-041, FR-042, FOUND-06)
- [ ] T082 [P] [US5] Write failing tests in `crates/meep-storage/tests/restore.rs` (5b, restore): restore into an empty store reproduces every record and re-exporting gives an equal backup; a non-empty store warns and requires "Replace everything"; "Replace everything" destroys the old keys and records and its outcome is stated in the owner-facing message (FR-017, FR-018, SC-004, NAMES-05)
- [ ] T083 [P] [US5] Write failing tests in `crates/meep-storage/tests/restore_refuse.rs`: a backup that breaks an address rule, a limit or a feature's own rule is refused whole with every problem listed and nothing written; a fragment for a feature that is not available is reported by name, not dropped (FR-017, FR-019, NAMES-05)
- [ ] T084 [P] [US5] Write failing crash tests in `crates/meep-storage/tests/crash_restore.rs`: terminating the process at every failpoint during a restore leaves the store exactly as it was before (FR-015, SC-005)
- [ ] T085 [P] [US5] Write failing tests in `crates/meep-storage/tests/restore_identity.rs`: a restore adopts the backup's account ID, creates a new store key and a new recovery key shown once, and the recovery key from before does not open the new store; cite data-model.md store_header.account_id, store_header.recovery_wrapped_store_key (FR-032, SC-011, R9, FOUND-03)
- [ ] T086 [US5] Implement restore in `crates/meep-storage/src/restore.rs`: read the manifest and refuse a newer major version, check for an empty store or the "Replace everything" choice, validate every fragment and address first and collect every problem, then apply everything in one transaction; cite data-model.md store_header.format_version, store_header.account_id (FR-017, FR-019, FR-032, FR-039, PLAN-23)
- [ ] T087 [P] [US5] Create the damaged-store fixture generator in `crates/meep-storage/tests/support/damaged.rs` (and generated files under `tests/fixtures/damaged/`, synthetic only): a store with a corrupted record, a store with a broken chain, a store with pending write-ahead-log content, and an unreadable file (SC-010, SC-012, FOUND-03)
- [ ] T088 [P] [US5] Write failing tests in `crates/meep-storage/tests/damaged_open.rs` (5c, damaged store): a partly damaged or chain-failed store opens read-only with `store-damaged-readonly` and every write returns `StoreReadOnly`; an unreadable store is refused with `store-unreadable` and points to the latest owner backup; the original files are byte-identical before and after opening, including with pending write-ahead-log content, using before-and-after SHA-256 (FR-028, FR-029, SC-012, FOUND-03)
- [ ] T089 [P] [US5] Write failing tests in `crates/meep-storage/tests/damaged_classify.rs`: the rule for "partly damaged" versus "cannot be read at all" gives one outcome per fixture; chain break and unreadable data produce the two different messages (FR-028, FR-033, FOUND-03)
- [ ] T090 [P] [US5] Write failing tests in `crates/meep-backup/tests/rescue.rs` and `crates/meep-storage/tests/rescue_restore.rs`: a rescue backup contains only records that read and validate fully, is labeled as from a damaged store, lists what could not be read, carries the old log only under `unverified-history/`, and always restores cleanly; importing it repeats the warning; the restored store verifies, starts a fresh chain whose first normal entry is `restored_from_rescue_backup` with the date, and keeps the old log read-only in the archive; cite data-model.md unverified_log_archive (FR-030, FR-031, SC-010, FOUND-03)
- [ ] T091 [US5] Implement damaged-store handling in `crates/meep-storage/src/damaged.rs`: copy `store.db` and `store.db-wal` byte-for-byte into a private temporary folder, hash the originals before and after, work only on the copy, run the key check, integrity checks, chain verification and a read-and-validate of every record, classify as normal, read-only damaged or refused, and delete the copy afterwards; never modify, repair or delete the original (FR-028, FR-029, SC-012, PLAN-20, FOUND-03)
- [ ] T092 [US5] Implement the read-only mode in `crates/meep-storage/src/store.rs`: every write operation returns `StoreReadOnly` and the only offered action is the rescue backup (FR-028, FOUND-03)
- [ ] T093 [US5] Implement the rescue backup in `crates/meep-backup/src/rescue.rs` and its import in `crates/meep-storage/src/restore.rs`: only fully valid records, the unread list, the old log as an unverified archive written to the table described by data-model.md unverified_log_archive, a fresh chain, and the new recovery key message (FR-030, FR-031, FR-032, PLAN-20, PLAN-23, FOUND-03)
- [ ] T094 [P] [US5] Write the format-docs test in `crates/meep-backup/tests/format_docs.rs` (5d, published description): the fixed text of the template equals `docs/format/READ-ME-FIRST.txt` and each feature section equals its file under `docs/format/fragments/`; editing either side makes the test fail, and it runs inside `cargo test --workspace` in the existing `unit` job (FR-040, SC-014, PLAN-22)
- [ ] T095 [US5] Fill `docs/format/READ-ME-FIRST.txt` and `docs/format/owner-backup-format.md` with the full format description (layout, manifest members, versioning, how to read the data without Meep) so the published copy and the in-backup copy are the same text once the owner has reviewed it (FR-036, FR-037, FR-040, FOUND-05)
- [ ] T096 [US5] Run the acceptance features `tests/acceptance/features/backup_restore.feature` and `tests/acceptance/features/damaged_store.feature` (Story 5 scenarios 1 to 19) and quickstart scenarios 12 to 18, 21 to 24 and 26; fix until green (ENG-02)

---

## Phase 8: User Story 6 - Meep is reachable only from my own machine (Priority: P2)

**Goal**: a loopback-only server shell with Host, Origin and per-session token checks; no sign-in.

**Independent Test**: try to reach Meep from another machine and from a web page on another site; both fail, while the owner's own use works (quickstart scenario 25).

- [ ] T097 [P] [US6] Write failing tests in `crates/meep-web/tests/binding.rs`: the server listens on loopback only (127.0.0.1 and ::1), a connection to the machine's network address is refused, binding any other address is a startup error, and the port is ephemeral (FR-020, R5, SC-006, PLAN-25)
- [ ] T098 [P] [US6] Write failing tests in `crates/meep-web/tests/request_checks.rs`: a wrong `Host`, a foreign `Origin`, `Sec-Fetch-Site: cross-site` and a state-changing request without the token each get 403 with the `request-refused` message; a normal same-origin request with the token succeeds with no sign-in; several browser tabs on one running Meep all work (FR-020, FR-021, FR-024, SC-006, FOUND-04)
- [ ] T099 [P] [US6] Write failing tests in `crates/meep-web/tests/token.rs`: the session secret is random and in memory only, the cookie is `HttpOnly` and `SameSite=Strict` with no `Domain`, the request token must match the cookie, responses carry `Cache-Control: no-store`, `X-Content-Type-Options: nosniff` and a content security policy limited to the server's own origin (FR-021, PLAN-25)
- [ ] T100 [P] [US6] Write a failing test in `crates/meep-web/tests/no_egress.rs` that fails if the dependency tree contains a network-client crate, using the `deny.toml` ban list, and a check that Meep makes no outbound connection when started (FR-027, EGRESS-01)
- [ ] T101 [US6] Implement the server shell in `crates/meep-web/src/server.rs`: loopback binding only, ephemeral port, the address printed for the owner, startup refusal on any non-loopback bind (FR-020, R5, PLAN-25)
- [ ] T102 [US6] Implement the request checks in `crates/meep-web/src/middleware.rs` and `token.rs`: Host allow-list, Origin and `Sec-Fetch-Site` checks, per-session token with cookie and request token, response headers (FR-021, PLAN-25) **[UNBLOCKS 010/001]**
- [ ] T103 [US6] Implement the Fluent loader in `crates/meep-web/src/i18n.rs` for `foundation.ftl` so later specs add their own catalogs beside it, and map `request-refused` and the store messages to the catalog (R10, PLAN-26)
- [ ] T104 [US6] Run the acceptance feature `tests/acceptance/features/local_access.feature` (Story 6 scenarios 1 to 5) and quickstart scenario 25; document in the project documentation that v1.0 assumes one person uses the computer and does not guard against other accounts (FR-022, FOUND-02); fix until green (ENG-02)

---

## Phase 9: Polish & Cross-Cutting Concerns

- [ ] T105 Run all 26 quickstart scenarios in `specs/002-foundation/quickstart.md` on this PC and record the results in that file; the SQLCipher prerequisites and the 121-character path rule are checked first with `scripts/check-windows-prereqs.ps1`; also time how long first run takes until the owner has saved the recovery key, as the owner-run check for SC-007 (spec Verification, PLAN-14, SC-007)
- [ ] T106 [P] Run `cargo deny check` and `cargo test --workspace` on Windows, Linux and macOS through CI and confirm `build-windows`, `build-macos` and the Linux jobs are green (principle 18, PLAN-14)
- [ ] T107 [P] Run `.github/scripts/guard.sh` and a manual scan to confirm no personal data, email address or real record exists in code, fixtures, damaged-store samples, tests, backups or docs (SC-009, constitution, ENG-02)
- [ ] T108 [P] Write the owner-facing documentation section covering: the recovery key is shown once and cannot be replaced in v1.0, the backup file is not encrypted, earlier backups and exports cannot be recalled, copies outside Meep (operating-system caches, disk wear) are outside what Meep manages, the store must not be on a network share, and v1.0 assumes one person uses the computer (FR-002, FR-010, FR-022, FR-023, FOUND-02)
- [ ] T109 [P] Record the adjustments spec 001's tasks need now that the foundation owns the server and key primitives (T022, T023 move here; T049, T050, T054, T055 run on the real key custody; T026 and T027 are answered by this work) in `specs/002-foundation/plan.md` under Impacts, without editing spec 001 files (BUILD-01, FOUND-01)
- [ ] T110 **[OWNER QUESTION]** After implementation, ask the owner to promote PLAN-13 to PLAN-27 from Proposed to Confirmed (or reject any), updating each record and `docs/decisions/README.md` (principle 23)
- [ ] T111 Run `/speckit-analyze` for cross-artifact consistency and `/speckit-converge` to append any unbuilt work (principle 25)

---

## Dependencies & Execution Order

### Phase dependencies

- **Phase 1 (Setup)**: none; start immediately.
- **Phase 2 (Foundational)**: needs Phase 1; **blocks every user story**. The ports, fakes, address checker, record operations, key-destruction, kind registry and server request checks marked **[UNBLOCKS 001/010]** are what specs 001 and 010 wait for. The fakes (T020) let those specs start before any real store exists.
- **Phase 2b (Open items)**: can run alongside Phase 2; the recovery-key presentation answer (T028) is needed before the first-run text in User Story 1 is final.
- **Phases 3 to 8 (stories)**: need Phase 2.
- **Phase 9 (Polish)**: needs the stories you intend to ship.

### User story dependencies

- **US1** (encrypted store, keys, one copy): only Phase 2.
- **US2** (envelope, addresses, record operations): needs US1 (an open store) and the foundational log core (every save appends an entry).
- **US3** (deletion): needs US2 (records) and the log core.
- **US4** (verification, registry, start-up check): needs the log core; its start-up task needs US1 and uses damaged-store handling from US5 when a check fails.
- **US5** (backup, restore, damaged store, published format): needs US2 to US4; the restore and damaged-store parts need the store from US1.
- **US6** (loopback server): independent of US2 to US5 beyond the message catalog; needed before any screen exists.

### Within each story

Tests first (they must fail), then models, then services, then wiring, then run the acceptance feature. Do not start the next story's implementation tasks until the previous checkpoint is green.

### Parallel opportunities

- Phase 1: T002 to T011 in parallel after T001.
- Phase 2: T014, T015, T016, T017, T021, T023 and T024 in parallel; T020 follows T019.
- Each story's [P] test tasks are separate files and can be written together.
- US5: parts 5a, 5b, 5c and 5d have independent test tasks that can be written in parallel.

### Example: parallel test authoring for User Story 1

```text
T031 open_keys.rs        T032 file_unreadable.rs    T033 first_run.rs
T034 lost_keys.rs        T035 key_hierarchy.rs     T036 store_header.rs
T037 lock.rs
```

## Implementation Strategy

1. **First, unblock the later specs**: finish Phase 1 and Phase 2, in particular the ports and fakes (T018, T020), so spec 010 and the names domain work can proceed against them (BUILD-01).
2. **MVP**: User Story 1 then User Story 2: an encrypted store that opens only with a key and holds records with a full envelope. Validate with quickstart scenarios 1 to 6.
3. Add User Story 3 (deletion that destroys for good) and User Story 4 (tamper-evident history); these are where R8 and principle 16 are proven.
4. Add User Story 5 (backup, restore, damaged store, published format) and User Story 6 (loopback access).
5. After each story: run the full suite on Windows, open a pull request, merge only when `all-green` passes (ENG-03).
6. The owner questions (T028, T029, T079, T110) are asked when their phase is reached; none changes a Confirmed decision.

## Notes

- Out of scope and not tasked: names (spec 001), shared UI elements and theme (spec 010), distribution, signing and updates and the snapshot before migration (spec 015, AT-35 and AT-36), login, delegates, caregivers, insurance, AI, cloud, v1.1 and later (spec Out of Scope).
- Open items covered: macOS and Linux SQLCipher builds (T030), `keyring` backends (T027), recovery-key presentation (T028), `cargo-deny` license handling (T007, T029), the READ-ME text for owner review (T079).
- Test-only unencrypted mode: the `plaintext-test-store` feature exists only so the destruction scan can see page bytes; it must never be enabled in a release build (PLAN-16).
