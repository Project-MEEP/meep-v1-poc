# Meep-Directive-ORC_2026-11-28_0730-CT

| Field | Value |
|---|---|
| Targets | One new PR from branch `feat/002-store-core` off `main`: spec 002 T022 (tests) and T023 (store core), with the owner decisions they need recorded first |
| Expected head | `main` at `15e422d` (#50 merged). If `main` has moved, run `git diff --name-only 15e422d origin/main`. Continue only if none of the files this directive names changed; otherwise stop and report. |
| Supersedes | Nothing |
| Standard | STD-02 (`docs/standards/SOP-DIRECTIVES.md`) |

**Owner confirmation:** sending this records the owner decisions of 2026-11-28 (07:15 to 07:25 CT), confirmed outside the no-change window:

- **D8-A:** raw 256-bit key, SQLCipher 4 settings pinned with `cipher_compatibility = 4`, and `cipher_memory_security = ON`.
- **D9 (all four items):**
  - `format_version` starts at 1.
  - The header table is one row, `STRICT`, with text IDs and times.
  - `last_successful_backup_at` is created now.
  - `busy_timeout` is 5000 ms.
- **D10:** the codes `E-STORE-KEY-REJECTED`, `E-STORE-MISSING` and `E-STORE-EXISTS`.
- **D11-A:** one SEC-05 item 1 exception for the key statement, plus a CI check.
- **D12-A:** the code `E-STORE-OPEN-FAILED` for any other failure. The orchestrator applies it to create as well as open, and has told the owner.

**Instruction IDs:** `Meep-Directive-ORC_2026-11-28_0730-CT#S<step>.<item>`.

## Standing rules

- Read `CLAUDE.md` and `AGENTS.md` at the current head first. No step may contradict them (STD-02 A5.9).
- PRs only. Never merge, never rebase, never force-push. To update the branch, merge `main` into it.
- **Stop point (owner decision PL-01 B, 2026-11-27):** open the PR as a draft and wait for `all-green`. Then **stop and report to the orchestrator** without marking it ready; the orchestrator marks it ready. After that, handle Codex: fix a finding only if the fix is under 20 lines and has a test; otherwise answer the thread and stop. Then send the A6 report.
- Tests first. Each test commit comes before the code that makes it pass, and its message says it fails.
- Before using any new name, test name or ID, check that it isn't already taken.
- **EXACT** blocks land verbatim.
- **Stop and report** on any conflict with a Confirmed record or existing text, whenever text a step expects to find is missing, and on any Stop-if.
- Test-name checks pair `grep` with `cargo test`, because `cargo test` passes when no test matches.
- Report any change you make to an Accept-when command under Deviations (PL-11).
- No `Co-Authored-By` lines and no "Generated with" lines (owner D2-A). Synthetic data only.
- Don't install anything. If `scripts/check-windows-prereqs.ps1` reports a missing prerequisite, stop and report.

## Verified facts (orchestrator, `main` at `15e422d`)

These were checked by the orchestrator. If your work shows one is false, stop and report.

- F1. `crates/meep-storage` holds only `Cargo.toml` and `src/lib.rs` (`#![forbid(unsafe_code)]`). Its dependencies already include `meep-core`, `rusqlite` 0.43.0 (`bundled-sqlcipher-vendored-openssl` on Windows, `bundled-sqlcipher` elsewhere), `zeroize`, `data-encoding` and `tempfile`. Its features are `plaintext-test-store` and `in-memory-key-store`.
- F2. `crates/meep-core/src/errors.rs`:
  - `Error` has no store-open variants.
  - `message()` (line 78) and `diagnostic_code()` (line 112) are exhaustive `match`es.
  - `detail_key()` (wildcard at line 143) and `cause()` (line 149, wildcard at line 152) end in wildcard arms; `cause()` returns `Cause::None` for everything except `StoreFailed`.
  - `MessageId` has `StoreWrongKey`, `StoreUnreadable` and `InternalError` (`messages.rs`, around lines 30-32).
- F3. `crates/meep-core/tests/errors.rs` uses exhaustive no-wildcard helpers (`ordinal`, `reason_ordinal`, `expected_code`, `every_error`, `declared`). A new `Error` variant fails to compile there until every helper covers it.
- F4. `crates/meep-core/src/clock.rs`: `trait Clock` (line 9), `to_rfc3339` (line 14), `FixedClock::new` (line 35).
- F5. `crates/meep-core/src/ids.rs`: `StoreId` and `AccountId` come from the same macro, which provides `new()` (line 22), a hyphenated `Display`, and a canonical-v4-only `FromStr`.
- F6. `.github/scripts/check-release-features.sh` line 7 scans artifacts for `MEEP_TEST_FEATURE_PLAINTEXT_STORE`.
- F7. `.github/scripts/check-sec05.sh` has four checks and no static-SQL check yet. CI's `guard` job runs it (`ci.yml` line 27).
- F8. CI's `unit` job runs `cargo test --workspace` with no features (`ci.yml` line 67).
- F9. Placement:
  - `data-model.md` lines 25-27 are the last three `store_header` rows, followed by a blank line and `### key_slots`.
  - `SEC-05.md` lines 7-8 are its two `Update` lines.
  - `diagnostic-codes.md` line 27 is the last table row (`E-HISTORY-UNVERIFIED`).
  - `tasks.md` lines 63-64 are T022 and T023.
- F10. ERR-01 item 1 (`docs/decisions/ERR-01.md` line 14) says storage code never uses `PoisonError::into_inner`. This PR is the phase-gate re-audit point E4 of the 2026-11-22 audit.
- F11. None of the test names, type names or codes this directive introduces exist on `main`.

**Hints to check, not facts:**

- H1. The orchestrator believes SQLCipher reports a wrong key, no key and a non-database file all as `SQLITE_NOTADB` (extended code 26) on the first read after keying. Confirm it in S3.
- H2. Whether rusqlite 0.43's `execute_batch` copies the SQL text internally is unknown (the owner's D11 hypothesis). Find out in S4.4 and report.

---

## S0. Record

- **S0.1** Create branch `feat/002-store-core` from `origin/main`. The first commit adds this file, unchanged, as `docs/directives/Meep-Directive-ORC_2026-11-28_0730-CT.md`.
  - **Accept when:** `test -f docs/directives/Meep-Directive-ORC_2026-11-28_0730-CT.md`

## S1. Owner decisions into the records (before any code)

- **S1.1** Append the EXACT lines to the end of `docs/decisions/PLAN-14.md`.

  EXACT:
  ```
  - **Update 2026-11-28 (owner, D8-A, D9 item d):** the store is opened with a raw 256-bit key, `PRAGMA key = "x'<64 lowercase hex characters>'"`, so SQLCipher does no key derivation of its own (the key is already derived from the store key). The SQLCipher 4 settings are pinned with `PRAGMA cipher_compatibility = 4`, so a library default change cannot silently change the file format. `PRAGMA cipher_memory_security = ON` wipes SQLCipher's memory after use. `busy_timeout` is 5000 ms.
  - **Implementation note 2026-11-28 (orchestrator, reversible):** `foreign_keys = ON` on every open (research.md); every setting that SQLite or SQLCipher can report back is read back after it is set, and a value that did not take refuses the open.
  ```
- **S1.2** In `specs/002-foundation/data-model.md`, insert the EXACT paragraph right after the `last_successful_backup_at` row of the `store_header` table, with one blank line before it.

  EXACT:
  ```
  **Layout (owner, 2026-11-28, D9):** `format_version` starts at 1. The table is `STRICT` and holds exactly one row (`id INTEGER PRIMARY KEY CHECK (id = 1)`). `store_id`, `account_id` and `created_at` are `TEXT NOT NULL` (UUID v4 in hyphenated form; UTC RFC 3339) and `format_version` is `INTEGER NOT NULL`. `last_successful_backup_at` is `TEXT`, NULL until the first verified backup, and is created with the table so the backup task needs no layout change.
  ```
- **S1.3** Append the four EXACT rows to the end of the table in `specs/002-foundation/contracts/diagnostic-codes.md`, right after the `E-HISTORY-UNVERIFIED` row.

  EXACT:
  ```
  | `E-STORE-KEY-REJECTED` | `StoreKeyRejected` | No key, a wrong key, or a file that is not a Meep store; SQLCipher cannot tell these apart, and telling them apart would reveal something about the file (FR-003) | `store-wrong-key` |
  | `E-STORE-MISSING` | `StoreMissing` | The store file to open does not exist | `store-unreadable` |
  | `E-STORE-EXISTS` | `StoreExists` | Create was asked to write where a store already exists, and refused so nothing is overwritten (programmer error) | `internal-error` |
  | `E-STORE-OPEN-FAILED` | `StoreOpenFailed` | Creating or opening the store failed for another reason (permission, disk I/O, locked); carries a content-free `Cause` | `store-unreadable` |
  ```
- **S1.4** In `docs/decisions/SEC-05.md`, insert the EXACT line as a new line right after line 8 (the second `Update` line).

  EXACT:
  ```
  - **Update 2026-11-28 (owner, D11-A):** one exception to item 1. SQLCipher cannot take its key as a bound parameter, so the key statement `PRAGMA key = "x'…'"` is the only SQL not written as static text. It is built in one function in `crates/meep-storage/src/store.rs`, from the key's 64 lowercase hex characters only, in a buffer that is wiped after use; CI fails on any other SQL that is not static text.
  ```
- **Accept when:**
  ```
  grep -q 'Update 2026-11-28 (owner, D8-A, D9 item d)' docs/decisions/PLAN-14.md \
  && grep -q 'Layout (owner, 2026-11-28, D9)' specs/002-foundation/data-model.md \
  && grep -q 'Update 2026-11-28 (owner, D11-A)' docs/decisions/SEC-05.md \
  && for c in E-STORE-KEY-REJECTED E-STORE-MISSING E-STORE-EXISTS E-STORE-OPEN-FAILED; do [ "$(grep -c "\`$c\`" specs/002-foundation/contracts/diagnostic-codes.md)" -eq 1 ] || exit 1; done
  ```

## S2. Error variants (meep-core)

- **S2.1** Tests first. In `crates/meep-core/tests/errors.rs`, extend every exhaustive helper (`ordinal`, `expected_code`, `every_error`, and any other that fails to compile) with four new variants:
  - `StoreKeyRejected`
  - `StoreMissing`
  - `StoreExists`
  - `StoreOpenFailed { cause }`

  Use the codes from S1.3. Add the test `store_open_failures_map_to_their_messages`. It asserts these mappings:

  | Variant | `message()` id |
  |---|---|
  | `StoreKeyRejected` | `StoreWrongKey` |
  | `StoreMissing` | `StoreUnreadable` |
  | `StoreExists` | `InternalError`, with the `code` argument |
  | `StoreOpenFailed { cause: Cause::Sqlite(14) }` | `StoreUnreadable` |

  It also asserts that `StoreOpenFailed`'s `cause()` returns its cause. Commit; the tests fail to compile.
- **S2.2** In `crates/meep-core/src/errors.rs`, add the four variants with one-line doc comments: unit variants for the first three, `StoreOpenFailed { cause: Cause }` for the fourth. Add their arms to `message()`, `diagnostic_code()` and `cause()`.
- **S2.3** Replace the two existing wildcard arms in `errors.rs` with explicit arms for every variant, so the compiler catches a new variant there too:
  - `_ => None` at line 143, in `detail_key()`
  - `_ => Cause::None` at line 152, in `cause()`

  Behavior must not change for any existing variant (the existing tests prove it). Don't add a wildcard arm anywhere. This is the orchestrator's reversible decision (H-01, H-41), in the spirit of #50.
- **Accept when:**
  ```
  grep -q 'fn store_open_failures_map_to_their_messages' crates/meep-core/tests/errors.rs \
  && cargo test -p meep-core --test errors \
  && cargo test -p meep-core \
  && ! grep -nE '^\s*_ =>' crates/meep-core/src/errors.rs
  ```

## S3. Store core tests (T022)

- **S3.1** Create `crates/meep-storage/tests/store_core.rs` with the tests below, using `tempfile` directories and `FixedClock`. Each one names the FR it covers in a comment. The API they call is fixed in S4.1.

  | Test function | Asserts |
  |---|---|
  | `a_store_is_created_and_reopened_with_its_key` | `Store::create` then `Store::open` with the same key succeeds |
  | `the_header_holds_format_version_one_and_fresh_ids` | `header()` returns `format_version == 1`, canonical v4 `store_id` and `account_id` that differ from each other, and `created_at` equal to the `FixedClock` time as RFC 3339; `last_successful_backup_at` is `None` |
  | `opening_with_a_wrong_key_is_refused_as_key_rejected` | `Err(Error::StoreKeyRejected)` |
  | `a_file_that_is_not_a_store_is_refused_as_key_rejected` | a `store.db` of synthetic random bytes gives `Err(Error::StoreKeyRejected)`, and the file is byte-identical afterwards |
  | `the_store_cannot_be_read_without_a_key` | a plain `rusqlite::Connection` on `store.db`, with no key, fails `SELECT count(*) FROM sqlite_master` (FR-003, SC-001) |
  | `opening_a_missing_store_is_refused_and_creates_nothing` | `Err(Error::StoreMissing)`, and the directory is still empty afterwards |
  | `create_refuses_an_existing_store_and_leaves_it_unchanged` | a second `create` in the same directory gives `Err(Error::StoreExists)`, `store.db` is byte-identical, and the first key still opens it |
  | `the_settings_are_in_effect_after_open` | `settings()` reports journal mode `wal`, synchronous `FULL` (2), secure delete on, foreign keys on, busy timeout 5000 ms, and cipher memory security on |
  | `the_store_file_holds_no_plaintext` | the raw bytes of `store.db` and of `store.db-wal` (if present) contain neither the `account_id` nor the `store_id` text, nor `SQLite format 3` |
  | `a_database_key_never_shows_in_debug_output` | for `DbKey::from_bytes([0x42; 32])`, `format!("{key:?}")` equals exactly `DbKey(<redacted>)` |
  | `the_store_file_is_owner_only_on_unix` | `#[cfg(unix)]`: `store.db` has mode `0o600` |
  | `the_plaintext_test_store_is_readable_without_a_key` | `#[cfg(feature = "plaintext-test-store")]`: a store from `Store::create_plaintext` starts with `SQLite format 3\0`, and a plain `rusqlite::Connection` reads `store_header` |

  Commit; the tests fail to compile.
- **Accept when:** each name passes the grep-plus-`cargo test` check (after S4):
  ```
  for t in a_store_is_created_and_reopened_with_its_key the_header_holds_format_version_one_and_fresh_ids opening_with_a_wrong_key_is_refused_as_key_rejected a_file_that_is_not_a_store_is_refused_as_key_rejected the_store_cannot_be_read_without_a_key opening_a_missing_store_is_refused_and_creates_nothing create_refuses_an_existing_store_and_leaves_it_unchanged the_settings_are_in_effect_after_open the_store_file_holds_no_plaintext a_database_key_never_shows_in_debug_output the_store_file_is_owner_only_on_unix the_plaintext_test_store_is_readable_without_a_key; do grep -q "fn $t" crates/meep-storage/tests/store_core.rs || { echo "missing $t"; exit 1; }; done \
  && cargo test -p meep-storage --test store_core \
  && cargo test -p meep-storage --test store_core --features plaintext-test-store
  ```

## S4. Store core (T023)

- **S4.1** Create `crates/meep-storage/src/store.rs` and declare it in `lib.rs`. These names and signatures are fixed:
  - `pub struct DbKey`:
    - holds `zeroize::Zeroizing<[u8; 32]>`
    - built only by `DbKey::from_bytes(bytes: [u8; 32]) -> DbKey`
    - no `Clone`, `Copy`, `Display` or `Serialize`
    - a manual `Debug` that prints `DbKey(<redacted>)`
  - `pub struct StoreHeader { pub store_id: StoreId, pub account_id: AccountId, pub format_version: u32, pub created_at: String, pub last_successful_backup_at: Option<String> }`
  - `pub struct StoreSettings { pub journal_mode: String, pub synchronous: i64, pub secure_delete: bool, pub foreign_keys: bool, pub busy_timeout_ms: u64, pub cipher_memory_security: bool }`
  - `pub struct Store`, holding the `rusqlite::Connection`, with:
    - `pub fn create(dir: &Path, key: &DbKey, clock: &dyn Clock) -> Result<Store, Error>`
    - `pub fn open(dir: &Path, key: &DbKey) -> Result<Store, Error>`
    - `pub fn header(&self) -> Result<StoreHeader, Error>`
    - `pub fn settings(&self) -> Result<StoreSettings, Error>`
  - Behind `#[cfg(feature = "plaintext-test-store")]` only: `pub fn create_plaintext(dir: &Path, clock: &dyn Clock) -> Result<Store, Error>` and `pub fn open_plaintext(dir: &Path) -> Result<Store, Error>`. The feature adds these two functions and changes nothing in `create` or `open`.
  - The store file is `dir/store.db` (data-model.md file names).
- **S4.2** Opening order, for both create and open:
  1. Open the connection without the create flag on `open`, and with it on `create`.
  2. Apply the key (S4.4).
  3. `PRAGMA cipher_compatibility = 4`.
  4. `PRAGMA cipher_memory_security = ON`.
  5. A first read, `SELECT count(*) FROM sqlite_master`. On `open`, `SQLITE_NOTADB` here becomes `StoreKeyRejected`.
  6. `journal_mode = WAL`, `synchronous = FULL`, `secure_delete = ON`, `foreign_keys = ON`, and `busy_timeout` 5000 ms (rusqlite's `busy_timeout`).
  7. Read back every setting in `StoreSettings`. If any value didn't take, return `StoreOpenFailed { cause: Cause::None }`.

  `create` then creates `store_header` exactly as S1.2 says, and inserts the one row with `StoreId::new()`, `AccountId::new()`, `format_version` 1 and `to_rfc3339(clock.now())`, inside one transaction. On Unix it then sets `store.db` to mode `0o600` with `std::fs::set_permissions`. On `open`, a missing `store_header` table is `StoreKeyRejected` (the file is not a Meep store).
  - **Stop if:** `cipher_compatibility` or `cipher_memory_security` is rejected by the bundled SQLCipher, or a read-back shows a different value than was set.
- **S4.3** Error mapping:

  | Case | Result |
  |---|---|
  | `open` where `dir/store.db` doesn't exist (checked with `try_exists` before connecting) | `StoreMissing` |
  | `create` where `dir/store.db` or `dir/store.db-wal` already exists | `StoreExists`, with nothing written |
  | `SQLITE_NOTADB` at the first read | `StoreKeyRejected` |
  | any other SQLite failure during create or open | `StoreOpenFailed { cause: Cause::Sqlite(extended code) }` |
  | any other I/O failure during create or open | `StoreOpenFailed { cause: Cause::Io(kind) }` |
  | a failed header read after a successful open | `StoreFailed { cause }` |

  No `From` impls. No error text from rusqlite or std is kept, logged or put into `Error`.
- **S4.4** The key statement (D11), in one private function `apply_key`. It encodes the 32 key bytes with `data_encoding::HEXLOWER` into a `zeroize::Zeroizing<String>`, builds the full statement in that same buffer, and runs it with `execute_batch`. The line that runs it ends with the comment `// SEC-05 exception (D11)`. It is the only line in the crate with that marker, and the only SQL in the crate not written as a string literal or a `const`.
  - Find out from the rusqlite 0.43.0 source whether `execute_batch` copies the SQL text into a new allocation (for example a `CString`). Report the file and line, and whether the copy is wiped.
  - Don't work around a copy yourself; report it.
- **S4.5** All other SQL is a string literal or a `const &str` passed straight to rusqlite, with bound parameters for values. No `PoisonError::into_inner`. No logging, printing or `dbg!`.
- **S4.6** Test-feature marker: under `#[cfg(feature = "plaintext-test-store")]`, add `#[used] static PLAINTEXT_TEST_STORE_MARKER: &str = "MEEP_TEST_FEATURE_PLAINTEXT_STORE";` in `store.rs`.
- **S4.7** In `specs/002-foundation/tasks.md`, tick T022 and T023 (`- [ ]` becomes `- [x]`). Change no other text.
- **Accept when:**
  ```
  cargo clippy -p meep-storage --all-targets -- -D warnings \
  && cargo clippy -p meep-storage --all-targets --features plaintext-test-store -- -D warnings \
  && cargo test -p meep-storage \
  && cargo test -p meep-storage --features plaintext-test-store \
  && ! git grep -n 'PoisonError::into_inner' -- crates/meep-storage \
  && [ "$(git grep -c 'SEC-05 exception (D11)' -- crates/meep-storage/src | awk -F: '{s+=$2} END {print s+0}')" -eq 1 ] \
  && grep -qE '^- \[x\] T022 ' specs/002-foundation/tasks.md && grep -qE '^- \[x\] T023 ' specs/002-foundation/tasks.md
  ```
  Marker present only with the feature:
  ```
  cargo build --release -p meep-storage --features plaintext-test-store && grep -a -c MEEP_TEST_FEATURE_PLAINTEXT_STORE target/release/libmeep_storage.rlib
  cargo build --release -p meep-storage && ! grep -a -q MEEP_TEST_FEATURE_PLAINTEXT_STORE target/release/libmeep_storage.rlib
  ```
  The first must print 1 or more; the second must exit 0.

## S5. CI gates

- **S5.1** In `.github/workflows/ci.yml`, add one step to the `unit` job, after `cargo test --workspace`, that runs `cargo test -p meep-storage --features plaintext-test-store`, with the same `if:` condition as the step before it. Change nothing else in the workflow.
- **S5.2** Add check 5 to `.github/scripts/check-sec05.sh` and its header comment: static SQL in `crates/meep-storage/src` (SEC-05 item 1, D11). It fails when either of these is true:
  - (a) a line in `crates/meep-storage/src/**/*.rs` contains `format!`, `format_args!`, `concat!`, `write!`, `writeln!` or `.push_str(`, and the same line doesn't end with `// SEC-05 exception (D11)`;
  - (b) after joining each file into one line (as check 2 does), a call to `.execute(`, `.execute_batch(`, `.prepare(`, `.prepare_cached(`, `.query_row(` or `.query_map(` has a first argument that is neither a string literal (`"`, `r"`, `r#"`) nor an `UPPER_SNAKE_CASE` identifier, unless the call is followed by the `// SEC-05 exception (D11)` marker before the next `;`.

  It prints each hit with its file and line. When there are no hits it passes and prints `check-sec05: clean` as today.
  - **Stop if:** (a) blocks a legitimate non-SQL use in `store.rs`. Report the line rather than adding a marker to it.
- **Accept when:**
  - `bash .github/scripts/check-sec05.sh` passes on the final tree.
  - `grep -q 'features plaintext-test-store' .github/workflows/ci.yml`
  - Negative dry-run, with output recorded in the PR body. Stage a throwaway file `crates/meep-storage/src/zz_sec05_probe.rs` containing:
    - `fn a(c: &rusqlite::Connection, t: &str) { let _ = c.execute_batch(&format!("DROP TABLE {t}")); }`
    - `fn b(c: &rusqlite::Connection, s: &str) { let _ = c.execute(s, []); }`

    Run `git add` on it, run the script, and confirm it exits non-zero and names both lines. Then run `git rm --cached` and delete the file. It must not be committed.

## S6. Finish

- **S6.1** Open the PR as a draft, titled `Foundation: encrypted store core (002:T022, T023; D8-D12)`. Its body lists:
  - every instruction ID with its Accept-when command and raw output
  - the S4.4 rusqlite finding
  - every Deviation
- **S6.2** Wait for `all-green`. Then **stop and report to the orchestrator** (stop point in the standing rules). Don't mark the PR ready.
- **S6.3** When the orchestrator says the PR is ready, wait for Codex and handle each thread under the standing rules. Then send the A6 report. Don't merge.
