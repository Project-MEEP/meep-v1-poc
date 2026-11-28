# Meep-Directive-ORC_2026-11-27_1415-CT

| Field | Value |
|---|---|
| Targets | The S9 test-gap PR of `Meep-Directive-ORC_2026-11-22_1315-CT` (branch `test/err-01-gaps` from `main`) |
| Expected head | `main` at `fe0da5a` when the branch is cut (stop if `main` has moved and the panic hook changed) |
| Supersedes | Only the S9.1 Stop-if of `Meep-Directive-ORC_2026-11-22_1315-CT`, and only for the test `a_panic_without_location_is_dep`. Everything else in S9 stands. |
| Standard | STD-02 (`docs/standards/SOP-DIRECTIVES.md`) |

**Owner confirmation:** sending this confirms owner decision D6-B (2026-11-27 14:14 CT): a refactor of at most 10 lines in the panic hook is allowed so that the missing-location case can be tested.

**Instruction IDs:** `Meep-Directive-ORC_2026-11-27_1415-CT#S<step>.<item>`.

## Standing rules

The standing rules of `Meep-Directive-ORC_2026-11-22_1315-CT` apply unchanged. Read `CLAUDE.md` and `AGENTS.md` at the current head first; no step here may contradict them (STD-02 A5.9). Never merge.

## Verified facts (orchestrator, 2026-11-27 14:15 CT, `main` at `fe0da5a`)

- F1. `crates/meep-core/src/panic_hook.rs` lines 18-26: `install()` handles `info.location() == None` inline in the hook closure with `format!("{PANIC_CODE} {DEPENDENCY}")`. No public function accepts a missing location.
- F2. `pub fn describe(file: &str, line: u32) -> String` is at line 30 of the same file.
- F3. No function named `describe_location` exists anywhere in the repository.
- F4. Panic hook tests live in `crates/meep-core/tests/panic_hook.rs` and import `meep_core::panic_hook::{PANIC_CODE, describe, install}`.

## S0. Record

- **S0.1** The first commit of the S9 PR adds this file, unchanged, as `docs/directives/Meep-Directive-ORC_2026-11-27_1415-CT.md`.
  - **Accept when:** `test -f docs/directives/Meep-Directive-ORC_2026-11-27_1415-CT.md`

## S1. Missing-location test and the extraction it needs

- **S1.1** Tests first. In `crates/meep-core/tests/panic_hook.rs`, add the test `a_panic_without_location_is_dep`. It calls `describe_location(None)` and asserts the result equals `format!("{PANIC_CODE} <dep>")`. Commit it before the fix (it fails to compile until S1.2 lands; say so in the commit message).
- **S1.2** In `crates/meep-core/src/panic_hook.rs`, add `pub fn describe_location(location: Option<(&str, u32)>) -> String` with a one-line doc comment. `Some((file, line))` returns `describe(file, line)`. `None` returns `format!("{PANIC_CODE} {DEPENDENCY}")`. Change the `install()` closure to build its line with `describe_location(info.location().map(|loc| (loc.file(), loc.line())))`, removing the inline `match`.
  - The diff to `panic_hook.rs` is at most 10 added and 10 removed lines, doc comment included.
  - No other file under `crates/*/src` changes.
  - **Stop if:** the change needs more than 10 lines, or the output of any existing panic-hook test changes.
- **Accept when:**
  - `grep -q 'pub fn describe_location' crates/meep-core/src/panic_hook.rs`
  - `grep -q 'fn a_panic_without_location_is_dep' crates/meep-core/tests/panic_hook.rs && cargo test -p meep-core --test panic_hook a_panic_without_location_is_dep`
  - `git diff --numstat origin/main -- crates/meep-core/src/panic_hook.rs` shows 10 or fewer in each column
  - `git diff --name-only origin/main -- 'crates/*/src'` lists only `crates/meep-core/src/panic_hook.rs`
  - `cargo test -p meep-core` passes

## S2. Report

- **S2.1** The S9 report (STD-02 A6) cites these IDs alongside the 1315 S9 IDs, with each Accept-when output.
