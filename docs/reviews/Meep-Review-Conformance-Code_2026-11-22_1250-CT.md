# Conformance audit: decisions that already have code

Drafted 2026-11-22 12:50 CT. Auditor: the reviewer (independent of the executor), per STD-02 Part B. Scope 2B: the Confirmed decisions with code or specs merged on `main` (#38–#43). PR #44 is excluded because it is still open.

Method:
- The intent ledger is built from the owner's directives and answers between 2026-11-21 20:50 and 2026-11-22 00:51.
- Evidence was read from `main` through the GitHub API on 2026-11-22.
- Most of these instructions were sent as chat pastes, before the SOP required directive files. Rows marked "chat-only" are a process defect in themselves (P2).

## Summary

| Status | Count |
|---|---|
| Conforms | 24 |
| Partial | 4 |
| Deviated, reported | 1 |
| Deviated, silent | 0 |
| Not done | 0 |
| N/A (code not built yet) | 3 |

**Headline:** the code and specs match what was asked. There are no silent deviations. The gaps are thin test coverage in four places and three process defects. The largest process defect: the naming standard is still Proposed, yet everything has been built to it.

## Ledger

### ERR-01: errors and panics (directive 2026-11-21 2050 Part C; answers 23:53, 00:51)

| ID | Instruction | Status | Evidence | Root cause / note |
|---|---|---|---|---|
| E1 | Deny unwrap_used, expect_used, panic, todo, unimplemented in product code; allow them in tests | Conforms | Root `Cargo.toml` `[workspace.lints.clippy]`; `clippy.toml` allow-*-in-tests | |
| E2 | Release `panic = "abort"` | Conforms | `[profile.release] panic = "abort"` | |
| E3 | Panic hook: code plus repo path and line, never the payload; `<dep>` for non-repo paths | Conforms | `src/panic_hook.rs`; tests in `tests/panic_hook.rs` (canary, both slashes, absolute Windows, registry Windows and Unix, toolchain) | |
| E3t | Panic hook test coverage | Partial | No test for a panic with no location, an absolute Unix workspace path, or a Windows toolchain path | Directive asked for "tests for both slash directions + registry + canary", which are met. The gaps are beyond the ask. |
| E4 | Storage never uses `PoisonError::into_inner` | N/A | No storage code yet | Re-audit at T022+ |
| E5 | `Display` equals the diagnostic code, with a test over every variant | Conforms | `errors.rs` manual Display; test `display_is_the_diagnostic_code_for_every_variant` | Risk: `every_error()` is hand-maintained, so a new variant can escape the test (Q2) |
| E6 | `Cause`: Copy, with None, Io and Sqlite; carried by StoreFailed; never in Display | Conforms | `errors.rs`; test `a_storage_failure_carries_a_content_free_cause` | |
| E7 | `DetailRefused.key` is a typed `DetailKey` | Conforms | `errors.rs` field `key: DetailKey` | |
| E8 | Seven code renames and the address split | Conforms | Codes in `errors.rs` and `diagnostic-codes.md` match exactly | |
| E9 | Registry at `contracts/diagnostic-codes.md`; a test fails on a missing or duplicate code | Conforms | Test `the_code_registry_lists_every_code` | Stale comment ("address reasons share one code") in `diagnostic_codes_are_uppercase_area_condition_and_unique` |
| E10 | Message wording (record-changed, record-not-found, record-deleted; record-unreadable unchanged) | Conforms | `foundation.ftl` lines match the 00:51 approved text | |
| E11 | internal-error: keep "nothing was changed" only if proven, else the alternate; `{ $code }` | Deviated, reported | `foundation.ftl` uses the alternate text with `{ $code }` | The orchestrator took the safe branch. It didn't report the proof attempt explicitly, but the outcome is allowed by the directive. |
| E12 | CI fails a release build that enables `failpoints` or `testing` | Conforms | `check-release-features.sh` (feature-tree and default checks) | Low: the feature-tree grep isn't anchored, so `testing` could match `testing-extra` (a false positive, which is the safe direction) |
| E13 | NAMING §2: hyphenate every multi-word condition; point to the registry | Conforms | `docs/standards/NAMING.md` §2 | See P1: the file is still Proposed |

### SEC-05: input and injection safety (00:46 paste; 08:4x corrections)

| ID | Instruction | Status | Evidence | Note |
|---|---|---|---|---|
| S1 | CI fails on Box::leak, String::leak, Vec::leak | Conforms | `check-sec05.sh` check 1 | |
| S2 | CI fails on askama `\|safe` | Conforms | `check-sec05.sh` check 4 (also `escape = none`) | |
| S3 | rusqlite only inside meep-storage | Conforms | `check-sec05.sh` check 3 | It doesn't match a bare `.execute(`. The `use rusqlite` and `rusqlite::` patterns cover realistic cases. |
| S4 | Fluent bidi isolation stays on, with a test | Partial | `check-sec05.sh` check 2 is a grep guard | A runtime test needs meep-web; add it with the first Fluent rendering task |
| S5 | NFC first, then the caps | N/A | Names code not built | Re-audit at spec 001 implementation |

### LOG-01: logging (08:46 paste)

| ID | Instruction | Status | Evidence | Note |
|---|---|---|---|---|
| L1 | Logging only through the typed API; no print or log macros elsewhere | Conforms | `check-logging.sh` | The panic hook writes through a sink function, which is allowed |
| L2 | Canary tests (item 9) | Partial | None yet; the log module doesn't exist | It must be tracked as a task, and that hasn't been checked (Q4) |
| L3 | Boot log for pre-unlock failures | N/A | T133 not built; the panic hook takes a sink until then | |

### VAL-01 and spec 001 reconciliation (01:08 answers)

| ID | Instruction | Status | Evidence |
|---|---|---|---|
| V1 | Names in one FOUND-09 batch aren't independent; a refused Legal swap saves nothing | Conforms | Spec 001 FR-001c; US2 scenario 10 |
| V2 | "A save" means one item or one batch | Conforms | Spec 001 FR-030 |
| V3 | A mixed-result message names what saved and what didn't, with rule IDs | Conforms | Spec 001 FR-030; US2 scenario 11 |
| V4 | Error summary plus inline `aria-describedby` | Conforms | Spec 010 FR-029, FR-024 |

### Spec 010, LANG-02, ERR-01 item 7 (01:04–01:08 answers)

| ID | Instruction | Status | Evidence |
|---|---|---|---|
| U1 | Info and success toasts: 8 s, never under 5, pause on hover or focus | Conforms | Spec 010 FR-010 |
| U2 | Error toasts never auto-dismiss; every toast goes to the tray | Conforms | Spec 010 FR-010, FR-012 |
| U3 | Typed word is the action's own verb from Fluent, ignoring case and surrounding spaces | Conforms | Spec 010 Q2 and edge cases |
| U4 | Language setting from v1.0; never changes names; English fallback; en-XA test-only; logged as a code | Conforms | Spec 010 FR-034 to FR-038 |
| U5 | Surfaces (inline, toast plus tray, blocking) and four severities | Conforms | Spec 010 FR-032, FR-033 |
| U6 | LANG-01 gap fill; LANG-02 record | Conforms | `docs/decisions/LANG-01.md`, `LANG-02.md` |

## Process findings

- **P1. STD-01 is Proposed but enforced.** `NAMING.md` still reads "Status: Proposed". The E- codes, file names and registry row all follow it. CLAUDE.md says only Confirmed decisions bind, so in principle the code was built to a non-binding rule. The owner never explicitly confirmed STD-01.
- **P2. The directives that drove these changes were chat pastes.** They were never committed, so today's ledger had to be rebuilt from conversation. STD-02 (A1) fixes this going forward. No retroactive fix is proposed.
- **P3. A missing record.** PR #41's title cites UI-01, but `docs/decisions/` has UI-02 and UI-03 and no UI-01.md.
- **P4. Hand-maintained variant lists in tests.** `every_error()` (and likely its peers) can drift from the enum. A test helper that does an exhaustive `match` on every variant makes adding a variant a compile error until the test covers it.

## Owner decisions

**Answered 2026-11-22 13:09 CT:** Q1 A, Q2 B, Q3 A, Q4 A, Q5 A. Re-audit E4 when storage code lands (T022 onward) and S5 when the spec 001 names code lands.

| # | Decision | Options | Recommendation |
|---|---|---|---|
| Q1 | STD-01 (naming standard) | **A** Confirm now · **B** Keep it Proposed and treat the existing usage as accepted deviations | **A.** The code already follows it, and it was reviewed |
| Q2 | Test gaps (E3t no-location / Unix absolute / Windows toolchain; P4 exhaustive match; the stale E9 comment) | **A** Add them to the #44 directive as one small step · **B** A separate small PR after #44 · **C** Accept | **B.** Keeps #44's scope fixed; about 30 minutes of work |
| Q3 | UI-01 missing record | **A** Gap-fill from the master spec text, no new content · **B** Rename the PR reference | **A.** Same pattern as R5, ENG-03, ARCH-02 |
| Q4 | LOG-01 canary tests (item 9) | **A** Check that tasks.md has a canary task; add one if not · **B** Defer until the log module exists | **A.** It's a one-line check now, versus a forgotten requirement later |
| Q5 | Re-audit points | **A** Re-audit E4 and S5 when storage (T022+) and names code land, as phase-gate audits under STD-02 B1 · **B** Ad hoc | **A** |
