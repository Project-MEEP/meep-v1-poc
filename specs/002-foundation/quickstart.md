# Quickstart: Validating the Foundation (v1.0)

A validation guide, not implementation. The workspace does not exist yet, so these describe what will run once it does. Details live in [data-model.md](data-model.md) and [contracts/](contracts/). Synthetic fixtures only; no real personal data and no email addresses.

## Prerequisites

- Rust stable (installed: rustc 1.100.0 with rustfmt and clippy) and the Microsoft C++ Build Tools (installed).
- For the SQLCipher build: **either** a system OpenSSL (`OPENSSL_DIR`) **or** a native Windows Perl for the vendored OpenSSL build (see research.md, Spike status). This is the one prerequisite not yet settled on this PC.
- No network is needed to run anything (FR-027).

## Commands (once the workspace exists)

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                          # unit and property tests
cargo test -p meep-acceptance --test acceptance # Given/When/Then for the six stories
cargo test -p meep-storage --test crash         # failpoint and child-process kill tests
cargo test -p meep-backup --test format_docs    # in-backup read-me equals docs/format
cargo run -p meep-web                           # loopback server shell
```

CI runs the same checks on every pull request; `main` merges only when `all-green` passes (ENG-02, ENG-03).

## End-to-end validation scenarios

Run each against a fresh store directory unless stated.

| # | Scenario (fixture) | Expected outcome | Spec ref |
|---|---|---|---|
| 1 | First run | Store created; recovery key shown once; no typing needed on the next start | Story 1, FR-002, FR-003 |
| 2 | Open with no key, a wrong key, the recovery key, the normal key | First two refused; last two open | SC-001, FR-004 |
| 3 | Copy the store file to a temporary folder and open it with no key | Cannot be read | FR-001 |
| 4 | Lose both keys | Stays closed; plain message; no backdoor | FR-004 |
| 5 | Save record A | Full envelope present; account ID equals the store's | FR-006, FR-007 |
| 6 | Check the five fixture addresses and the invalid ones | Valid accepted; each invalid refused with a reason; case-insensitive comparison | SC-008, FR-008 |
| 7 | Delete record A (sentinel content) | Unreadable; log shows only "a record was deleted" and the date | FR-010, FR-011 |
| 8 | Raw-byte scan after delete (unencrypted-mode run) | No sentinel and no key in the file, its log or free space | SC-002, FR-010 |
| 9 | Ask for a purpose value, then delete the record | Value works before; meaningless after | FR-012 |
| 10 | Append entries, verify; then edit, remove, reorder one | Verify passes with a count; each tamper fails naming the first broken entry | SC-003, FR-013 |
| 11 | Try to change an earlier entry or put content or a key in one | Refused | FR-014 |
| 12 | Make an owner backup | One zip with READ-ME-FIRST.txt, data, schema, manifest; read-me complete on its own | Story 5, FR-036, FR-037 |
| 13 | Restore into an empty store, re-export | Records equal; backup equal | SC-004, FR-018 |
| 14 | Restore into a non-empty store | Warning; needs "Replace everything" | FR-017 |
| 15 | Restore a backup that breaks a rule | Whole restore refused, every problem listed, nothing written | FR-017 |
| 16 | Crash during save, delete, append, restore, backup | Store unchanged, or a verified backup, or no backup-named file; changes saved during a backup survive | SC-005, SC-015 |
| 17 | Backup while a record is saved | Save not delayed; backup as of its start; "Changes you made after <time> aren't in this backup" | SC-016, FR-041, FR-042 |
| 18 | Backup with a newer major version | Refused with a plain message; nothing guessed | FR-039 |
| 19 | Start a second Meep program | Refused with "Meep may already be open..."; the store untouched | FR-024, SC-013 |
| 20 | Hard-kill Meep, start again | Opens normally after the integrity and chain check; no marker to clear | FR-034, SC-013 |
| 21 | Damaged copy of a fixture store | Opens read-only with a warning; file byte-identical before and after | FR-028, FR-029, SC-012 |
| 22 | Rescue backup from that store, restore it | Restores cleanly; restored store verifies; old log only in the unverified archive; new chain starts with `restored_from_rescue_backup` | SC-010, FR-030, FR-031 |
| 23 | After scenario 22, use the old recovery key | Fails; the new recovery key opens it | SC-011, FR-032 |
| 24 | Chain break versus unreadable data | Two different messages; the break message names a software fault or outside editing, never an intruder | FR-033 |
| 25 | Request from another machine, from another web page, with a wrong Host or no token | Refused; the owner's own request works with no sign-in | SC-006, FR-020, FR-021 |
| 26 | Check the published format description against the backup read-me | They match; editing one side fails CI | FR-040, SC-014 |

## Done when

- The master spec's foundation unit-test list passes (envelope on every record, address parser, database will not open without its key, append-only hash-chained log, export and re-import equal).
- Scenarios 1 to 26 behave as stated.
- The crash, lock and destruction tests (scenarios 8, 16, 19, 20) are in CI and pass.
- `all-green` passes.
