# Meep-Directive-ORC_2026-11-22_1215-CT

| Field | Value |
|---|---|
| Target | PR #44, branch `feat/002-t019-ports`, plus one new PR for STD-02 |
| Expected head of #44 | `c67e2d0` |
| Supersedes | `Meep-Directive-ORC_2026-11-22_1119-CT`, and every chat answer about #44 after 2026-11-22 02:00 CT |
| Standard | STD-02 (SOP-DIRECTIVES.md), confirmed by the owner 2026-11-22 |

**Owner confirmation:** sending this confirms every item marked **Confirmed**. Items marked **Pending owner** are not built.

**Instruction IDs:** each item below is cited as `Meep-Directive-ORC_2026-11-22_1215-CT#S<step>.<item>`. Cite these IDs in commits, PR bodies and reports.

## Standing rules

- Drafts only. Never merge. Mark a PR ready only where a step says to.
- Tests first: write the test, run it, see it fail, then implement.
- Before using any new ID (decision, diagnostic code, Fluent ID, task), check that it isn't already taken.
- **Stop and report** if a step conflicts with a Confirmed record, with existing code, or with another step. Never choose between readings.
- Text inside an **EXACT** block lands verbatim. Everything outside EXACT blocks is instruction, not content.
- To bring a branch up to date with main, merge main into it. Never rebase a pushed branch.
- **Test-name checks:** `cargo test <filter>` passes even when no test matches. Every test check below therefore pairs a `grep` for the test function with the `cargo test` run. Both must succeed.

## Verified facts (checked by the reviewer; don't re-derive)

- F1. #40, #42, #43 and #41 are merged. #44 is open at `c67e2d0`.
- F2. `deny.toml` already bans reqwest, ureq, hyper-tls, isahc, curl and surf (FR-027).
- F3. Deletion mechanics are already planned: research D4, and tasks T022, T023, T028, T032, T065 and T069, using the `plaintext-test-store` feature. Add no deletion code or tasks.
- F4. `DetailValue::from_stored(key, text) -> Option<DetailValue>` returns None for both unknown and malformed text. `DetailKey::from_code` and `NoticeId::from_code` return `Option`.
- F5. `NoticeId::as_str()` is the Fluent message ID. Don't add another method.
- F6. The Fluent catalog is in `crates/meep-web/locales/en/`.
- F7. proptest 1.12.0 is the planned version (T006).
- F8. `Address::parse` checks syntax only.

---

## S0. Preconditions and the directive record

- **S0.1** Run `git fetch && git rev-parse --short origin/feat/002-t019-ports`. **Stop if** the result isn't `c67e2d0`. Report what changed instead.
- **S0.2** On #44's branch, make this file's first new commit: `docs/directives/Meep-Directive-ORC_2026-11-22_1215-CT.md` (the attached file, unchanged).
  - **Accept when:** `test -f docs/directives/Meep-Directive-ORC_2026-11-22_1215-CT.md`

## S1. STD-02 adoption (Confirmed). Separate draft PR from `main`

- **S1.1** Create branch `docs/std-02-sop`. Add the attached `SOP-DIRECTIVES.md` unchanged as `docs/standards/SOP-DIRECTIVES.md`.
- **S1.2** Create `docs/decisions/STD-02.md`. Its status line is the exact text below, and its body points to `docs/standards/SOP-DIRECTIVES.md`. Add a row to the decisions README index.

  EXACT:
  ```
  Status: Confirmed (owner, 2026-11-22)
  ```
- **S1.3** Open it as a draft PR titled `Standards: directive and conformance SOP (STD-02)`.
  - **Accept when:** `test -f docs/standards/SOP-DIRECTIVES.md && grep -q 'Confirmed' docs/decisions/STD-02.md`
  - **Stop if:** STD-02 is already taken.

## S2. Registered addresses (Confirmed), on #44

- **S2.1** Take the next free FOUND number (call it FOUND-NN) and create `docs/decisions/FOUND-NN.md`, status Confirmed, title "Registered addresses". It must cover:
  - An address names one repository (`INSTANCE.NAME.Domain.Namespace.Repository`). Features register whole addresses, not segment vocabularies.
  - v1.0 fixes the instance and codename segments in code as `Alpha.Meep`. Names register `Alpha.Meep.Personal.Identity.Names`.
  - Registry comparison ignores ASCII case (R16). The original case is kept for display.
  - `Address::parse` stays a syntax-only check. The registry check runs at the port boundary: every `save` and `update`, and every Address-type event detail passed in with them. Later, backup import too. `from_stored` stays pure and has no registry access.
- **S2.2** Add `register_address(address)` on the same port that has `register_kind`, so one registry serves both checks. Registering the same address twice is an error. Implement it in the fakes.
  - **Stop if:** `register_kind` isn't on a port that `save`/`update` can reach. Report where it lives before restructuring anything.
- **S2.3** Add `AddressReason::Unregistered`, with diagnostic code `E-ADDRESS-UNREGISTERED`. Add a row to `specs/002-foundation/contracts/diagnostic-codes.md`. Its owner message is the generic `internal-error`; the syntax reasons keep their `address-*` messages. The existing all-variants Display test and the registry test must include it.
- **S2.4** Tests, written first, in meep-core:

  | Test function | Asserts |
  |---|---|
  | `parse_accepts_syntactically_valid_unregistered_address` | `Address::parse("Alpha.Meep.Personal.Okafor.Names")` is Ok |
  | `save_refuses_unregistered_address` | `save()` with that address fails with `E-ADDRESS-UNREGISTERED`, and nothing is written |
  | `save_accepts_registered_address_any_ascii_case` | a registered address saves in its original case and in different case |
  | `register_address_twice_is_refused` | the second registration fails |
  | `event_with_unregistered_address_detail_refuses_whole_call` | no record and no log entry are written |

  Delete the old `#[ignore]`d Okafor test; the first two rows replace it.
  - **Accept when:** for each test name N, `grep -rq "fn N" crates/meep-core && cargo test -p meep-core N` succeeds; and `! grep -rn '#\[ignore\]' crates/meep-core | grep -i okafor`
- **S2.5** Docs:
  - Update spec 002 FR-008 and `specs/002-foundation/contracts/ports.md` to describe `register_address` and the Unregistered refusal.
  - Append the EXACT line below to `docs/decisions/R6.md` and to `docs/decisions/R16.md`, with NN replaced by the number you took.

    EXACT:
    ```
    See FOUND-NN (registered addresses).
    ```
  - Add this unchecked task to `specs/002-foundation/tasks.md`, using the next free T number:

    EXACT:
    ```
    Backup import refuses records whose address is not registered; the record is listed as not imported with its diagnostic code.
    ```
  - **Accept when:** `grep -q 'register_address' specs/002-foundation/contracts/ports.md && grep -q 'FOUND-' docs/decisions/R6.md docs/decisions/R16.md`

## S3. Spec 001 FR-025 and the ARCH-03 note (Confirmed), on #44

- **S3.1** Rewrite spec 001 FR-025 so it says all names use the one fixed address `Alpha.Meep.Personal.Identity.Names`, and records are told apart by id, never by address. Remove the phrase "each name's address segment".
  - **Accept when:** `! grep -q "each name's address segment" specs/001-identity-names/spec.md && grep -q 'Alpha.Meep.Personal.Identity.Names' specs/001-identity-names/spec.md`
- **S3.2** If `docs/decisions/ARCH-03.md` exists, append the EXACT text there. Otherwise append it to the ARCH-03 row of the newest master spec copy in `docs/spec/` (newest by the timestamp in the file name). Do not create ARCH-03.md.

  EXACT:
  ```
  If multiple instances are ever named, the instance segment must be a code-assigned value, never an owner-chosen name. A friendly instance name is display-only and never appears in an address, because addresses are written to the permanent change log.
  ```

## S4. Unknown versus malformed stored codes (Confirmed), on #44

- **S4.1** Change `from_stored` to return `Result<DetailValue, StoredValueError>`, where `StoredValueError` has two variants:
  - `UnknownCode`: the text matches the code grammar but names no known variant.
  - `Malformed`: anything else.

  Apply the same split to `DetailKey::from_code` (grammar `^[a-z][a-z0-9_]*$`) and `NoticeId::from_code` (grammar `^[a-z][a-z0-9-]*$`). `accepts()` stays a bool and is true only for Ok.
- **S4.2** Tests, written first: `from_stored_unknown_notice_code_is_unknown`, `from_stored_garbage_is_malformed`, `detail_key_unknown_code_is_unknown`, `detail_key_garbage_is_malformed`.
  - **Accept when:** each passes the grep-plus-`cargo test` check from the standing rules.
- **S4.3** Edit task text only; no code. Append the EXACT sentence below to tasks T077 and T080 in `specs/002-foundation/tasks.md`.

  EXACT:
  ```
  Chain verification checks integrity only (seq continuity, prev_hash, hash over the canonical bytes); it must not fail because an event kind, detail key or NoticeId is unknown.
  ```

  Then add this new unchecked task with the next free T number:

  EXACT:
  ```
  Entries with well-formed but unknown kinds, keys or notice codes are kept unchanged and reported once as written by a newer Meep: diagnostic code E-HISTORY-NEWER, owner message ID history-from-newer-version (wording pending owner review). E-HISTORY-UNVERIFIED stays reserved for integrity failures.
  ```

  - **Stop if:** `E-HISTORY-NEWER` or `history-from-newer-version` already exists.

## S5. NoticeId catalog check (Confirmed), on #44

- **S5.1** Add the test `every_notice_id_has_fluent_message` in meep-core. It reads every `*.ftl` file under `concat!(env!("CARGO_MANIFEST_DIR"), "/../meep-web/locales/en")` and asserts that each `NoticeId` variant's `as_str()` is a message ID in them. This is test-only file access, so PLAN-13 (no I/O in product code) is not affected.
  - **Stop if:** `backup-made`, `backup-failed` or `save-failed` is missing from the catalog. Don't write wording.
- **S5.2** Append this rule to `specs/002-foundation/contracts/owner-messages.md`:

  EXACT:
  ```
  Any new tray notice adds a NoticeId variant in the same PR; a notice without a variant cannot be dismissed (notice_dismissed logs the NoticeId).
  ```

  - **Accept when:** the grep-plus-`cargo test` check passes for `every_notice_id_has_fluent_message`, and `grep -q 'NoticeId variant in the same PR' specs/002-foundation/contracts/owner-messages.md`

## S6. Property tests (Confirmed), on #44

- **S6.1** Add `proptest = "1.12.0"` to meep-core's `[dev-dependencies]`.
  - **Accept when:** `cargo deny check` passes.
- **S6.2** Create `crates/meep-core/tests/detail_props.rs` with two properties:
  - `prop_from_stored_round_trips`: for every DetailKey and arbitrary strings `s`, `from_stored(key, s)` is Ok only if `to_stored()` of the result equals `s`.
  - `prop_non_role_word_never_changed_parts`: a lowercase word that is not one of the six PartRole codes, shaped as `word[n]`, is never Ok for ChangedParts.

  Keep the existing surname canary tests.
  - **Accept when:** `test -f crates/meep-core/tests/detail_props.rs && cargo test -p meep-core --test detail_props`

## S7. Log content and erasure policy record, on #44

Take the next free LOG number (call it LOG-NN) and create `docs/decisions/LOG-NN.md`, titled "Log content and erasure policy", with these items:

- **B, Confirmed:** integrity checks and schema checks are separate. An unknown but well-formed code means newer data, never tampering (S4).
- **A, Confirmed (documentation rule):** deleting a record destroys its content and key, and the deletion entry holds only the fact and the date (FR-011).
  - Earlier change-log entries for that record survive by design, because the hash chain can't be rewritten. What survives: its alias, address, event kinds, changed part roles and positions, and dates. The log stores the alias, never the record id.
  - Owner docs must list this. Part roles such as `particle` can hint at a naming tradition; disclose that.
  - Append this as a sentence to T070's task text.
- **G, Confirmed (design constraint):** v1.0 makes no outbound network requests (FR-027, enforced by the deny.toml bans).
  - Every future outbound request (updates in spec 015, SSO in AUTH-01, sync, AI) must be designed against the FTC position that using a health app can itself be health information: minimal identifiers, documented, opt-in where it reveals use.
  - Add this as a note to the REG-01 open question in the newest master spec copy.
  - Add this unchecked task with the next free T number:

    EXACT:
    ```
    When meep-web first adds hyper or axum: add cargo-deny bans.features entries denying hyper's client feature and hyper-util's client features; verify the syntax against the installed cargo-deny version.
    ```
- **P, Confirmed:** the property tests in S6.
- **D, Pending owner:** chain-head anchoring is FOUND-10 (Proposed). Cross-reference it only.
- **E, Pending owner:** owner backups are plaintext (FOUND-02). Add this open question to the newest master spec copy:

  EXACT:
  ```
  Revisit owner-backup encryption before v1.1 insurance intake (FOUND-02, SEC-02).
  ```
- **F, Met:** ERR-01 covers diagnostic output (CWE-209, CWE-532).
- **H, Already planned:** see F3. The behavior spike must prove that secure_delete zeroes freed content.
- **Accept when:** `grep -q 'Log content and erasure policy' docs/decisions/LOG-*.md`

## S8. Finish

- **S8.1** Push #44. Wait for `all-green` on the new head.
- **S8.2** Mark #44 ready (this starts Codex). Wait for Codex. Answer and resolve every thread.
- **S8.3** Report using the SOP A6 template. Cite every instruction ID with its status and evidence, include each Accept-when command with its output, and name the FOUND and LOG numbers you took. Include the STD-02 PR number.
- **S8.4** Do not merge either PR. Stop.

After the owner gives the merge go-ahead and #44 is merged, start T022 on a new branch from `main`.
