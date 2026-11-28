# Meep-Directive-ORC_2026-11-22_1315-CT

| Field | Value |
|---|---|
| Targets | One new docs PR (S1–S8), then one new test PR (S9). Both branch from `main`. |
| Preconditions | #44 and #45 are merged into `main` |
| Supersedes | Nothing. This follows `Meep-Directive-ORC_2026-11-22_1215-CT`. |
| Standard | STD-02 (`docs/standards/SOP-DIRECTIVES.md`) |
| Attachments | `Meep-Review-Conformance-Code_2026-11-22_1250-CT.md` |

**Owner confirmation:** sending this confirms the owner decisions of 2026-11-22 13:09 CT:

- **STD-01:** Confirmed.
- **Test gaps:** fixed in a separate PR after #44.
- **UI-01:** gap fill.
- **LOG-01 canary task:** check that it exists; add it if not.
- **Re-audit at phase gates:** recorded.
- **FOUND-10:** option B, the anchor in owner backups and on screen.
- **Owner-backup encryption:** option B, landing in v1.1.
- **History message wording:** option A.

**Instruction IDs:** `Meep-Directive-ORC_2026-11-22_1315-CT#S<step>.<item>`.

## Standing rules

- Drafts only. Never merge. Mark a PR ready only where a step says to.
- Tests first.
- Before using any new ID, check that it isn't already taken.
- **Stop and report** on any conflict with a Confirmed record or with existing text, and whenever text a step expects to find is missing.
- **EXACT** blocks land verbatim. Wherever an EXACT block contains `NN`, replace it with the number you took.
- To update a branch with `main`, merge `main` in. Never rebase a pushed branch.
- Test-name checks pair a `grep` for the function with `cargo test`, because `cargo test` passes when no test matches.

## Verified facts (reviewer, 2026-11-22 13:10 CT)

- F1. `docs/decisions/STD-01.md` has the status line `- **Status:** Proposed`. `docs/standards/NAMING.md` begins with `Status: Proposed (record as STD-01).`
- F2. `docs/decisions/FOUND-10.md` is Proposed. It is an open question with no options, and its notes say it is not in v1.0 scope until the owner decides.
- F3. `docs/decisions/` has UI-02.md and UI-03.md, but no UI-01.md.
- F4. At 13:10, #44 was ready at `90d2e80` and #45 (STD-02) was a draft at `a7893f2`.

---

## S0. Preconditions

- **S0.1** Run `gh pr view 44 --json state -q .state` and `gh pr view 45 --json state -q .state`.
  - **Stop if:** either result is not `MERGED`.
- **S0.2** Create branch `docs/owner-decisions-2026-11-22` from `main`. Its first commit adds two files, both unchanged:
  - `docs/directives/Meep-Directive-ORC_2026-11-22_1315-CT.md`
  - `docs/reviews/Meep-Review-Conformance-Code_2026-11-22_1250-CT.md` (the attachment)
  - **Accept when:** `test -f docs/directives/Meep-Directive-ORC_2026-11-22_1315-CT.md && test -f docs/reviews/Meep-Review-Conformance-Code_2026-11-22_1250-CT.md`

## S1. STD-01 Confirmed

- **S1.1** In `docs/decisions/STD-01.md`, replace the status line with the EXACT line below. Make the matching change to the status column of the decisions README index.

  EXACT:
  ```
  - **Status:** Confirmed (owner, 2026-11-22)
  ```
- **S1.2** In `docs/standards/NAMING.md`, replace the first status line with the EXACT line below.

  EXACT:
  ```
  Status: Confirmed (STD-01, owner 2026-11-22).
  ```
- **Accept when:** `grep -q 'Confirmed (owner, 2026-11-22)' docs/decisions/STD-01.md && grep -q 'Confirmed (STD-01' docs/standards/NAMING.md`

## S2. UI-01 gap fill

- **S2.1** Find the UI-01 row or entry in the newest master spec copy in `docs/spec/` (newest by the timestamp in its file name). Create `docs/decisions/UI-01.md` using that text only, with the same status the master spec gives it. Add an index row.
  - **Stop if:** the master spec has no UI-01. Report what #41 meant by UI-01 instead.
- **Accept when:** `test -f docs/decisions/UI-01.md`

## S3. FOUND-10 Confirmed, option B

- **S3.1** Rewrite `docs/decisions/FOUND-10.md` so it contains the EXACT text below. Keep its ID and its original date line, and update the index row.

  EXACT:
  ```
  - **Status:** Confirmed (owner, 2026-11-22)
  - **Decision:** The change-log head (seq and hash) is anchored outside the live store in two places in v1.0: every owner backup records it in its manifest, and the History view shows it. Restoring a backup compares the backup's recorded head with the live store's head and tells the owner how far back or ahead the restore goes before they confirm. This makes replacement of the whole store with an older copy visible at backup time, at restore time, and on screen.
  - **Deferred:** a continuous anchor in the OS keychain (checked on every open) is deferred to the R7 key-custody review before v1.1, so the keychain design changes once.
  - **Why:** detects accidental rollback (OS "previous versions", sync tools restoring an old file) with no new runtime failure modes; a same-account attacker can rewrite a keychain anchor too, so the keychain option mainly protects against accidents.
  ```
- **S3.2** Append the EXACT line to `docs/decisions/R7.md`.

  EXACT:
  ```
  Input for the v1.1 review: FOUND-10 defers a keychain-held change-log anchor to this review.
  ```
- **S3.3** Add the three EXACT tasks, unchecked, to `specs/002-foundation/tasks.md`, each with the next free T number.

  EXACT:
  ```
  Each owner backup records the change-log head (seq and full hash) in its manifest (FOUND-10).
  The History view shows the current change-log head seq and the first 12 hex characters of its hash (FOUND-10).
  Restoring a backup compares its recorded head with the live store's head and, before the owner confirms, says how many entries back or ahead the restore goes (FOUND-10).
  ```
- **S3.4** In spec 002, append the EXACT sentence to FR-013.

  EXACT:
  ```
  Replacement of the whole store with an older copy is made visible at backup and restore time and on the History view (FOUND-10); continuous detection is deferred to the R7 v1.1 review.
  ```
- **Accept when:** `grep -q 'Confirmed (owner, 2026-11-22)' docs/decisions/FOUND-10.md && grep -c 'FOUND-10' specs/002-foundation/tasks.md` prints 3 or more, and `grep -q 'FOUND-10' specs/002-foundation/spec.md`

## S4. Owner-backup encryption, option B, v1.1

- **S4.1** Take the next free FOUND number (FOUND-NN) and create `docs/decisions/FOUND-NN.md` with the EXACT text below. Add an index row.

  EXACT:
  ```
  # FOUND-NN: Owner-backup encryption (v1.1)
  - **Status:** Confirmed (vNext) (owner, 2026-11-22)
  - **Decision:** From v1.1 (insurance intake), owner backups offer encryption, pre-selected; the owner may untick it. An encrypted backup opens with the owner's existing recovery key, so no new secret is introduced. v1.0 backups stay plaintext as decided in FOUND-02.
  - **Consequence:** losing the recovery key also makes encrypted backups unreadable; the backup screen says so in plain words.
  - **Why:** v1.1 adds member IDs and insurance details; a plaintext backup of those is a ready-made medical identity theft kit.
  ```
- **S4.2** Append the EXACT line to `docs/decisions/FOUND-02.md`.

  EXACT:
  ```
  See FOUND-NN for owner-backup encryption from v1.1.
  ```
- **S4.3** In the newest master spec copy, find the open question "Revisit owner-backup encryption before v1.1 insurance intake (FOUND-02, SEC-02)." Append the EXACT text to it.

  EXACT:
  ```
   Resolved: FOUND-NN (owner, 2026-11-22).
  ```
  - **Stop if:** that question isn't there. Report where #44 put it.
- **Accept when:** `grep -q 'Owner-backup encryption (v1.1)' docs/decisions/FOUND-*.md && grep -q 'Resolved: FOUND-' docs/spec/*.md`

## S5. History message wording, option A

- **S5.1** In `specs/002-foundation/tasks.md`, find the task containing `history-from-newer-version`. Replace `(wording pending owner review)` with the EXACT text.

  EXACT:
  ```
  (approved wording: "Some history was written by a newer version of Meep. It is kept unchanged. Update Meep to see it.")
  ```
  - **Stop if:** no task contains `history-from-newer-version`.
- **S5.2** Under a "Planned messages" heading in `specs/002-foundation/contracts/owner-messages.md`, adding the heading if it's absent, add the EXACT line. Do not add it to `foundation.ftl` yet.

  EXACT:
  ```
  history-from-newer-version = Some history was written by a newer version of Meep. It is kept unchanged. Update Meep to see it.
  ```
- **Accept when:** `grep -q 'approved wording' specs/002-foundation/tasks.md && grep -q 'history-from-newer-version' specs/002-foundation/contracts/owner-messages.md`

## S6. LOG-01 canary task

- **S6.1** Run `grep -n -i 'canary' specs/002-foundation/tasks.md`.
  - If a task covers LOG-01 item 9 (canary strings checked against all log output), report its T number and change nothing.
  - Otherwise, add this unchecked task with the next free T number:

    EXACT:
    ```
    LOG-01 item 9: fixtures seed unique canary strings; a test fails if any canary appears in the History log, the Diagnostic log or the boot log output.
    ```
- **Accept when:** `grep -qi 'canary' specs/002-foundation/tasks.md`

## S7. (No action) Re-audit points

These are already recorded in the committed review (S0.2). Nothing to do.

## S8. Finish the docs PR

- **S8.1** Open it as a draft PR titled `Docs: owner decisions 2026-11-22 (STD-01, UI-01, FOUND-10, backup encryption, history wording)`. Wait for `all-green`.
- **S8.2** Mark it ready. Wait for Codex. Resolve every thread.
- **S8.3** Report using the SOP A6 template, citing every instruction ID with the output of each Accept-when command. Do not merge.

## S9. Test-gap PR (after S8.1 is open; separate branch `test/err-01-gaps` from `main`)

- **S9.1** Panic hook tests, written so they fail before any fix is made:

  | Test function | Input | Expected |
  |---|---|---|
  | `an_absolute_unix_workspace_path_is_never_shown` | `/home/bob/meep-v1-poc/crates/meep-core/src/clock.rs` | `E-PANIC <dep>`, no `bob` |
  | `a_windows_toolchain_path_is_never_shown` | `C:\Users\bob\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\option.rs` | `E-PANIC <dep>`, no `bob` |
  | `a_panic_without_location_is_dep` | no location | `E-PANIC <dep>` |

  - **Stop if:** the hook has no function that accepts a missing location. Report how a missing location could be tested, and don't refactor.
- **S9.2** Replace the hand-maintained `every_error()` in `crates/meep-core/tests/errors.rs` with a helper whose `match` over `Error` has no wildcard arm, so adding a variant fails to compile until the helper covers it. Do the same for any other hand-maintained variant list in `crates/meep-core/tests/` (report which ones).
- **S9.3** Fix the stale comment "address reasons share one code" in `diagnostic_codes_are_uppercase_area_condition_and_unique`. It should say that the two address reasons have separate codes.
- **Accept when:**
  - each S9.1 test passes the grep-plus-`cargo test` check
  - `! grep -q 'share one code' crates/meep-core/tests/errors.rs`
  - `cargo test -p meep-core` passes
- **S9.4** Open it as a draft PR titled `Tests: ERR-01 coverage gaps from conformance audit (Q2)`. Then wait for `all-green`, mark it ready, wait for Codex, and report. Do not merge.
