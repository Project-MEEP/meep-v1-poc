# ORC directive, 2026-11-21 20:50 CT

From: owner (Samantha Schroff), drafted with her review assistant.
For: the orchestrator Claude Code session.

**Owner confirmation:** sending this directive confirms ERR-01, LANG-01, VAL-01 and the spec 010 defaults in Part F, as written here. Everything else keeps its current status.

Read the whole directive before you act. Do the parts in order. Each part says when to stop and report.

---

## Part A. Standing rules (apply to everything, every time)

These are working agreements that live in no single file. Follow them even when a task doesn't mention them.

1. **No real personal data**, ever, in the repo, tests, fixtures, logs, commit messages or PR text. Use only the synthetic fixtures. Real data lives only in the git-ignored `data/` folder.
2. **Public name is "Samantha Schroff"** on anything public. Never use any other name.
3. **Pull requests only.** Open every PR as a draft. Never merge or mark ready unless the owner says so in this session for that PR number.
4. **Decision status is binding.** Only Confirmed records bind. If an instruction here conflicts with a Confirmed record, **stop and report the conflict.** Do not pick a side.
5. **Check every new ID against `docs/decisions/` and all `specs/*/tasks.md`** before using it. Report collisions.
6. **Tick your own tasks.** Each implementation PR ticks the `tasks.md` tasks it completes. Never tick anything under `specs/*/checklists/`; the owner ticks those.
7. **Logs are content-free** (LOG-01): no record content, credentials, keys, tokens, file paths or usernames. Identify data by FieldRef, record alias and rule ID only.
8. **Identity edits warn, never block** (NAMES-07): a confirmation is an extra step, never a refusal.
9. **Nothing disposed:** readable owner data is never silently dropped, hidden or deleted.
10. **All owner-visible text** goes through Fluent message IDs (R10). Owner text never comes from `Display`/`Debug` or a hard-coded string.
11. **Provider neutrality** (principle 11): don't propose any vendor, including Anthropic, as a default.
12. **Don't change system settings** on the owner's machine without asking first.
13. **Tests first:** write the test, see it fail, then implement. CI (`all-green`) must pass before you report "done".
14. **Report format** for every part:
    - what changed (files, PR number)
    - what you verified, with file path and line or test name
    - what you could not verify
    - conflicts found
    - questions for the owner

---

## Part B. Bookkeeping fix (small PR or include in Part C's PR)

**Fact:** on `main`, `specs/002-foundation/tasks.md` shows Phase 1 at 11/13 with T012 and T013 unchecked, and Phase 2 at 0/21, even though #39 (merged) covered T012–T018.

1. Find out why #39's tasks.md edits didn't land.
2. Tick T012–T018 if CI confirms them. If any are only partly done, leave them unticked and say what is left.
3. Report the cause.

---

## Part C. ERR-01: error and panic handling (Confirmed)

**Order:** keep #40 (foundation messages) as a draft. Apply ERR-01 items 2–4 on #40's branch, or in a follow-up PR based on it, whichever is cleaner. Report which you chose.

**Verified facts you can rely on (don't re-derive):**
- Workspace lints are only `unsafe_code = forbid` and `clippy::all = warn`. There are no `[profile.*]` sections.
- `failpoint.rs` is correctly gated: `#[cfg(any(test, feature = "failpoints"))]`, a no-op otherwise. `meep-core` has no default features; `testing` and `failpoints` are opt-in.
- `Error` derives `Clone, PartialEq, Eq, thiserror::Error` and has no `From` impls. Keep it that way: no `?`-conversion from library errors.
- `DetailRefused { key: String }` exists.
- #40 adds `Error::diagnostic_code()`.

**Record `docs/decisions/ERR-01.md` (Confirmed, owner 2026-11-21) with these items, then implement 1–5 and 9:**

1. **Panics.**
   - Workspace clippy lints: deny `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented` in non-test code. Allow them in tests via `clippy.toml` (`allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests`).
   - Release profile: `panic = "abort"`.
   - Install a panic hook that writes only a fixed boot-log code plus crate/module location. **Never the panic payload or message.**
   - Storage code never uses `PoisonError::into_inner`; a poisoned lock in storage is fatal. Clock and failpoint uses may stay. Add a one-line comment at each saying why.
2. **`Display`** for every `Error` variant is its stable diagnostic code (the same string `diagnostic_code()` returns, e.g. `E-STORE-READONLY`). Never prose. Add a test asserting `to_string() == diagnostic_code()` for every variant.
3. **Content-free cause.** Add a `Copy` cause type: `None`, `Io(std::io::ErrorKind)`, `Sqlite(i32 extended result code)`. Variants that wrap a storage or IO failure carry it. It goes to the diagnostic log only. **Never store or log the underlying error's message text.**
4. **`DetailRefused.key`:** replace `String` with a typed field reference (FieldRef or a closed enum). If the right type doesn't exist yet, stop and report what you'd add.
5. **Messages:** #40 already gives dedicated messages to Stale, NotFound, Unreadable and RecordKeyDestroyed, and the generic message plus code to the rest. Keep that. Present the full English wording to the owner for review in the PR body.
6. **Web boundary** (record now, implement with meep-web): handlers map `Error` to an HTTP status and a message ID, and never send `Display` or `Debug` text to the browser. Refusals stay 403 `request-refused`.
7. **Where errors show up** (hand to spec 010, Part F):
   - validation: inline per field, all failures at once, with rule IDs
   - transient save or backup failure: toast plus tray entry
   - store-level (wrong key, damaged, read-only): blocking screen

   Severity becomes part of the message catalog.
8. **Retries:** no automatic write retries beyond SQLite `busy_timeout`. Retrying is the owner's choice.
9. **Release safety:** add a CI check that a release build of every product crate enables neither `failpoints` nor `testing` (for example `cargo tree -e features --release` with a grep that fails on either).

**Stop and report** after the PR is open and CI has run.

---

## Part D. LANG-01: UI language (Confirmed)

Record `docs/decisions/LANG-01.md`:

1. Settings has a **Language** choice from v1.0. Only English ships until the first stable long-term release (LTS) is accepted. More languages come after that, from the DOD-16 list (Tsalagi, English, Spanish). Update DOD-16's timing in the master spec copy.
2. **Correction from the spec check:** there is **no** profile-level "naming convention" setting. Spec 001 has the owner choose the convention per name, in the Last field menu (FR-012, FR-016). Keep it per name. The rule: **UI language never changes how names are entered, split, stored or displayed.** Format locale (dates, numbers) is a separate future setting. None of these three implies another.
3. **Fallback:** if a message is missing in the chosen language, show English, never a raw ID. CI fails if any `en` ID is missing.
4. **Pseudo-locale:** test-only, never offered in release builds. Accented characters plus about 40% text expansion. Run the Playwright + axe suite once under it to catch hard-coded text and layout breakage. This is part of spec 010's test plan, not v1.0 shipping scope.
5. The language choice is stored per user and logged only as a fixed code.

Implementation goes in spec 010 (Part F). The record ships in the same PR as Part E.

---

## Part E. VAL-01: shared field validation (Confirmed)

Record `docs/decisions/VAL-01.md`:

1. **Server-side validation is the authority.** Any browser-side check is a hint only.
2. **Every rule** has a stable rule ID and a Fluent message (R10, ERR-01).
3. **All failures come back at once:** an error summary at the top that links to each field, plus an inline message per field linked with `aria-describedby` (WCAG 3.3.1, 3.3.3).
4. **The owner's input is kept on error,** never cleared.
5. **Validation runs on save,** not on draft (access review D11). NFC normalization first, then the caps (SEC-05).
6. **Future field types** (dates, phone numbers, member IDs in v1.1) register their rules in this pattern. No feature invents its own error display.

**Reconcile spec 001 before recording, and report what you find:**
- Quote FR-001a ("refuses the whole save", error on the field) and FR-001c ("a rejected box must not stop the other valid boxes from saving") exactly.
- Proposed reconciliation, pending owner review:
  - **One name saves atomically:** all of that name's boxes, or none. A half-saved name would break R12 and FOUND-09.
  - **Independent items save independently:** a different name, or another setting.
  - All failures in the form are shown at once, per item 3.
- Amend FR-001a/FR-001c wording to match. If the wording means something else, stop and report.

---

## Part F. Spec 010 additions (#41, draft, spec only)

Add to #41:

1. **Language setting and pseudo-locale** (LANG-01 items 1, 3, 4, 5).
2. **The VAL-01 pattern** (error summary plus inline, `aria-describedby`, input kept on error) as the shared form-error component. The NAMEVAL rule IDs use it.
3. **Error surfaces and severity** (ERR-01 item 7): validation inline, transient failures as toast plus tray, store-level as a blocking screen. Severity levels are info, warning, error, blocking.
4. **Defaults for #41's open questions** (owner-confirmed by sending this):
   - **Q1, toast display time:**
     - Error toasts never auto-dismiss; the owner closes them.
     - Info and success toasts stay at least 6 seconds and pause while hovered or focused.
     - Every toast is also recorded in the tray (UI-03), so nothing is only transient. WCAG 2.2.1 applies.
   - **Q2, typed confirmation word:** the action's own verb (e.g. "delete", "replace"), taken from the Fluent catalog in the UI language. Matching ignores case, and leading and trailing spaces are trimmed.
   - **Q3, tray storage:** inside the encrypted store, under the same rules as the History log (LOG-01). Dismissals are recorded as `notice_dismissed`.
5. **Out of scope, with pointers to `docs/reviews/` (the access review):** keep auto-lock and the resume/discard diff out.

**Stop and report** with the updated spec diff summary.

---

## Part G. Dependency version report (report only, no changes)

1. Compare every crate version in every `Cargo.toml` with `specs/002-foundation/research.md` and the PLAN records. Report each mismatch and any newer major version.
2. Priority order:
   1. `rand` (now 0.10.4). Confirm that every key, token and ID comes from the OS RNG (`OsRng`/`getrandom`), never a seeded or thread RNG, and name the call sites.
   2. Crypto crates.
   3. `rusqlite` with bundled SQLCipher.
3. Do not bump anything.

---

## Order and PR plan

1. Part B: bookkeeping. Small PR, or fold into Part C.
2. Part C: ERR-01 (on or after #40).
3. Parts D and E: one PR with the three records LANG-01, VAL-01 and ERR-01's record (if not already in Part C), plus the FR-001a/c reconciliation.
4. Part F: update #41.
5. Part G: report in your final message.

Everything stays a draft. Report after each part. If anything here is ambiguous, ask. Don't guess.
