---

description: "Task list for feature 001: Identity and Names (v1.0)"
---

# Tasks: Identity and Names (v1.0)

**Input**: Design documents from `specs/001-identity-names/` (spec.md, plan.md, research.md, data-model.md, contracts/, quickstart.md) and `docs/decisions/`

**Prerequisites**: plan.md and spec.md (both present); research.md, data-model.md, contracts/ and quickstart.md (all present)

**Tests**: REQUIRED. ENG-02 and the spec's Verification section make tests mandatory: unit tests plus the star-gated Given/When/Then acceptance tests (AT-23, AT-26 to AT-31). In every story, the test tasks come first and must fail before the code they cover is written (Red, then Green).

**Organization**: Tasks are grouped by user story (US1 to US5, from spec.md) so each can be implemented and tested independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: US1 to US5; none for Setup, Foundational and Polish
- Every task names its file path and cites the spec ID (FR, ID, AT, R, NAMES, PRON) or decision (PLAN-nn) it serves
- **[DEP-002]** = needs the foundation (spec 002) delivered before the real wiring can be done; **[DEP-010]** = needs the shared UI elements (spec 010). Build order for v1.0 is spec 002 (foundation), then spec 010 (shared UI elements), then the names screens on the real 010 elements (BUILD-01). Until then names domain and core tasks proceed against the ports and in-memory fakes; there are no throwaway placeholders. **This plan does not design 002 or 010.**
- Field citations: `data-model.md Entity.field` points at the canonical constraint; tasks do not repeat it

## Path Conventions

Cargo workspace from plan.md: `crates/meep-core/`, `crates/meep-storage/`, `crates/meep-web/`, `tests/acceptance/`, `tests/fixtures/`, `docs/decisions/`. All test data is synthetic (spec fixtures only); no real personal data and no email addresses anywhere (constitution).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: workspace, tooling, CI wiring, fixtures. No behavior yet.

- [ ] T001 Create the workspace manifest `Cargo.toml` at the repository root with members `crates/meep-core`, `crates/meep-storage`, `crates/meep-web`, `tests/acceptance`; pin edition and `rust-version`; set `[workspace.lints]` so clippy warnings are errors (PLAN-01, ENG-01)
- [ ] T002 [P] Create `crates/meep-core/Cargo.toml` and `src/lib.rs` with dependencies `unicode-segmentation`, `unicode-normalization`, `serde`, `serde_json`, `thiserror`, `uuid`, `rand`; no web, database or I/O crates (PLAN-01, PLAN-06, PLAN-11)
- [ ] T003 [P] Create `crates/meep-storage/Cargo.toml` and `src/lib.rs` with `rusqlite` (feature `bundled-sqlcipher`), `keyring`, `chacha20poly1305`, `hmac`, `sha2`, `hkdf`, `zeroize`, `rand` and a path dependency on `meep-core` (PLAN-02, PLAN-03, PLAN-04, PLAN-05)
- [ ] T004 [P] Create `crates/meep-web/Cargo.toml` and `src/main.rs` with `axum`, `tokio`, `askama`, `fluent-templates` and path dependencies on core and storage (PLAN-07, PLAN-08)
- [ ] T005 [P] Create `tests/acceptance/Cargo.toml` with `cucumber` and `proptest`, a `[[test]] name = "acceptance"` entry with `harness = false`, and a path dependency on `meep-core` (PLAN-12, ENG-02)
- [ ] T006 [P] Create `deny.toml` for `cargo-deny`: allow MIT, Apache-2.0 and Unicode-3.0, deny known vulnerabilities, require MIT-compatible licenses (constitution principle 18, research.md verification log)
- [ ] T007 [P] Create `rustfmt.toml` and commit `Cargo.lock` (application workspace) (ENG-01)
- [ ] T008 Verify and adjust `.github/workflows/ci.yml` against the new workspace: the lint, deps, unit and acceptance-domain jobs must run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo-deny`, `cargo test --workspace` and the acceptance suite as `cargo test -p meep-acceptance --test acceptance --no-fail-fast` (a workspace-wide `--test acceptance` fails for packages without that test target, so `.github/workflows/ci.yml` is changed to the package form); keep every third-party action pinned to a full commit hash (repository setting `sha_pinning_required` is on) and keep the `all-green` gate (ENG-02, ENG-03, PLAN-12)
- [ ] T009 [P] Create `tests/fixtures/names.json` holding only the five synthetic fixtures from data-model.md (Jordan Lee Rivera Okafor, Jordan Rivera, Jordan L. Rivera, Jordan Lee Pak, María José García López) plus `tests/fixtures/README.md` stating synthetic-only; confirm `.github/scripts/guard.sh` passes (constitution, ENG-02)
- [ ] T010 [P] Create `crates/meep-web/locales/en/main.ftl` (empty catalog with header comment) and the Fluent loader configuration file `crates/meep-web/src/i18n.rs` skeleton (R10, PLAN-08) (FR citations: FR-026)

**Checkpoint**: workspace builds empty; CI jobs run for real instead of skipping.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: only what names needs from specs 002 and 010, expressed as ports and thin interfaces, plus shared rules every story uses. No user story work starts until this phase is done.

**Foundation interfaces (ports and fakes; do not design spec 002 or 010 here)**

- [ ] T011 [P] **[DEP-002]** Depend on the ports the foundation defines in `crates/meep-core/src/ports.rs` (foundation T019: `RecordStore`, `ChangeLog` with `register_kind`, `KeyCustody` with `purpose_value` and `destroy_record_key`, and the `BackupFragment` trait; no operation returns key material) and add the names-side typed layer over `RecordStore` (a Name serialized to and from record content at the address `Alpha.Meep.Personal.Identity.Names`) in `crates/meep-core/src/name/store.rs`; names defines no port of its own (FR-019, R8, R9, FOUND-01)
- [ ] T012 [P] **[DEP-002]** Use the foundation's in-memory fakes of the three ports (foundation T021, the `testing` feature of `meep-core`); names writes no fakes of its own: enable the feature in names' dev-dependencies and add names-specific test helpers (the synthetic fixtures) in `crates/meep-core/tests/support/names.rs` (BUILD-01, ENG-02, FOUND-01)
- [ ] T013 [P] Write unit tests in `crates/meep-core/tests/events_no_text.rs` proving no domain event payload contains a name's words, parts, short form or pronoun text (contracts/domain-events.md rules)
- [ ] T014 Implement the domain event types in `crates/meep-core/src/events.rs` per contracts/domain-events.md (NameAdded, NameEdited, NameRetired, NameReactivated, NamePromoted, PreferredReplaced, NameDeleted, PronounSettingChanged) and register each kind and its allowed detail keys with `ChangeLog::register_kind` (foundation T078) so the log refuses anything else and never holds name text (NAMES-02, R8, PLAN-10, FOUND-01)
- [ ] T015 [P] **[DEP-002]** Switch names' tests from the fakes to the real foundation (store core foundation T023, `RecordStore` foundation T062, delete foundation T069, kind registry foundation T078) once the foundation is delivered, and re-run the T049 and T050 tests against it; keep the ports unchanged (R2, R7, R8, FR-019, BUILD-01)
- [ ] T016 [P] **[DEP-010]** Build the names screens only on the real shared elements from spec 010 (text field, menu, button, toggle, list row, status label, Settings page, theme toggle), after spec 002 and then spec 010 are delivered (BUILD-01); there are no throwaway placeholders (APPROACH: nothing is disposed), and any need 010 cannot meet is raised as a change request to 010 (UI-01, AT-41, AT-42, FR-032)

**Shared rules (tests first, then code)**

- [ ] T017 [P] Write tests in `crates/meep-core/tests/address.rs` for the five-segment address syntax: valid and invalid cases, case-insensitive comparison, the names address `Alpha.Meep.Personal.Identity.Names` (R6, R16) (FR citations: FR-025)
- [ ] T018 Implement the address parser in `crates/meep-core/src/address.rs`; used for data-model.md Name.address (R6, R16, PLAN-11) (FR citations: FR-025)
- [ ] T019 [P] Write tests in `crates/meep-core/tests/text_rules.rs` for NFC normalization, trim and collapse of spaces with apostrophes and hyphens kept (ID-09), the processing order NFC, trim, collapse spaces, then count (FR-004), and grapheme counting (UAX #29) where "María" with a combining accent counts as 5 and with a precomposed accent also 5 (NAMES-03, PLAN-06) (FR citations: FR-004)
- [ ] T020 Implement `crates/meep-core/src/text.rs` (normalize to NFC, trim, collapse, count user-perceived characters, enforce minimum and maximum lengths via a limits type); used for data-model.md Name.text and data-model.md Name part.value (ID-09, NAMES-03, PLAN-06) (FR citations: FR-001a, FR-004)
- [ ] T021 [P] Write tests and implement `crates/meep-core/src/ids.rs` for the random account ID (UUID v4) carried on every record; used for data-model.md Name.account_id (R9, PLAN-11)
- [ ] T022 [P] Write tests in `crates/meep-web/tests/names_routes.rs` that the names routes are mounted on the foundation's server shell and inherit its checks: a state-changing names route without the per-session token is rejected and a non-local `Host` or `Origin` is rejected; the shell itself and its tests are the foundation's (foundation T113, T114 and their tests, FR-020, FR-021) (R5, FOUND-01) **[DEP-010]**
- [ ] T023 Mount the names routes on the foundation's server shell in `crates/meep-web/src/routes/mod.rs`; build no server, anti-forgery or Host and Origin middleware of its own (those are foundation T113 and T114) (R5, FOUND-01) **[DEP-010]**
- [ ] T024 [P] Spike: prove a template can call a Fluent `t("message-id")` helper using `askama` and `fluent-templates` in `crates/meep-web/src/i18n.rs` with a test template; if it does not fit cleanly, switch the template layer to `minijinja` and update `docs/decisions/PLAN-07.md` and `PLAN-08.md` with the outcome (R10, PLAN-07, PLAN-08, research.md open item 5) (FR citations: FR-026) **[DEP-010]**
- [ ] T025 [P] Write a test in `crates/meep-web/tests/no_literal_text.rs` that fails the build if any template under `crates/meep-web/templates/` contains literal user-facing text instead of a message ID (R10, PLAN-08) (FR citations: FR-026) **[DEP-010]**
- [x] T026 [P] Closed: the `keyring` backends and the recovery-key format are the foundation's (foundation spike and research.md, PLAN-15); names builds no custody (R7, PLAN-03, FOUND-01) (closed by the foundation plan)
- [x] T027 [P] Closed: the SQLCipher build is verified on this PC with the vendored OpenSSL feature and a short build path (foundation research.md Spike status, PLAN-14); names builds no storage (R2, PLAN-02, FOUND-01) (closed by the foundation plan)

**Checkpoint**: ports, fakes, events, address, text rules, ids, secure web skeleton and the spike results are in place; user stories can start.

---

## Phase 3: User Story 1 - Add my name in three familiar boxes (Priority: P1) 🎯 MVP

**Goal**: add a Legal name in First, Middle and Last boxes, stored as labeled parts that rebuild the text exactly.

**Independent Test**: add "Jordan Lee Rivera Okafor" with no menu changes; it is listed exactly as entered, the Last name is one compound surname, and it exports First, Middle, Last unchanged (quickstart scenarios 1 to 3, 13, 14).

### Tests for User Story 1 (write first; they must fail)

- [ ] T028 [P] [US1] Unit tests in `crates/meep-core/tests/name_builder.rs`: parts joined in `seq` order with one space and no inserted punctuation rebuild `text` exactly; a mismatching assembly is refused and nothing is written; cite data-model.md Name.parts and Name.text (ID-04, R14, AT-28) (FR citations: FR-002, FR-003)
- [ ] T029 [P] [US1] Unit tests in `crates/meep-core/tests/name_required_parts.rs`: only one of First or Last required; Middle never required and never stored empty, no "NMN" filler; a single-character part is accepted; cite data-model.md Name part.role and Name part.value (ID-05, NAMES-03)
- [ ] T030 [P] [US1] Unit tests in `crates/meep-core/tests/compound_surname.rs`: several words in Last default to one family part ("Rivera Okafor"); the owner may split; cite data-model.md Name part.role (ID-08, FR-012)
- [ ] T031 [P] [US1] Unit tests in `crates/meep-core/tests/given_names.rs`: "José Luis" in First stays one given part unless split (AT-31, ID-05)
- [ ] T032 [P] [US1] Unit tests in `crates/meep-core/tests/spacing.rs`: leading and trailing spaces trimmed, runs of spaces collapsed, apostrophes and hyphens kept (ID-09) (FR citations: FR-004)
- [ ] T033 [P] [US1] Unit tests in `crates/meep-core/tests/limits.rs`: 128-character box limit, 384 full-name limit, 50-name cap, the whole save refused with the message on the offending field and everything typed kept, no separate combined limit for the three boxes, retired names counting toward the 50, and a stale-save warning ("Keep mine" or "Reload") tested in `crates/meep-core/tests/stale_save.rs`, counting user-perceived characters; cite data-model.md Name.text and Name part.value (NAMES-03, FR-001a) (FR citations: FR-031)
- [ ] T034 [P] [US1] Property test in `tests/acceptance/tests/rebuild_property.rs` using `proptest`: for any valid assembly the parts always rebuild the text exactly (AT-28, PLAN-12)
- [ ] T035 [P] [US1] Acceptance feature `tests/acceptance/features/add_name.feature` with step definitions in `tests/acceptance/tests/acceptance.rs`: Given/When/Then for AT-28, AT-30, AT-31 using the fixtures (PLAN-12)

### Implementation for User Story 1

- [ ] T036 [US1] Implement the Name and NamePart model in `crates/meep-core/src/name/model.rs`; cite data-model.md Name.text, Name.use, Name.status, Name.parts, Name part.seq, Name part.value, Name part.role, Name part.qualifier, and include data-model.md Name.share_outbound (default share, no control in v1.0) and Name.short_form as plain fields (ID-04, ID-01, ID-02, NAMES-01, NAMES-04) (FR citations: FR-002, FR-005, FR-018b)
- [ ] T037 [US1] Implement `NameBuilder` with the rebuild-equals-text check at `build()` in `crates/meep-core/src/name/builder.rs` (Builder pattern, ID-04, R14, PLAN-10) (FR citations: FR-002, FR-003)
- [ ] T038 [US1] Implement entry validation in `crates/meep-core/src/name/validation.rs`: required parts, rebuild check against the normalized text, limits via `text.rs`; the whole save is refused with the message on the offending field and each save is all-or-nothing (FR-030); blocking only for integrity and limits (ID-05, NAMES-03, principle 26) (FR citations: FR-001a)
- [ ] T039 [US1] Implement the names repository against the foundation's `RecordStore` (foundation T062) in `crates/meep-core/src/name/repository.rs`: a Name is saved as record content at the address `Alpha.Meep.Personal.Identity.Names` checked by the shared address checker; the encrypted storage is the foundation's **[DEP-002]**, so this runs on the fakes until T015; cite data-model.md Name.address, Name.account_id (R2, R8, R9, R6, FOUND-01)
- [ ] T040 [US1] Implement the add-name application service in `crates/meep-core/src/name/service.rs` emitting `NameAdded`, enforcing the 50-name cap (all non-deleted names count, retired included) with the NAMES-03 message before saving, detecting a name that changed since it was opened (FR-031), and keeping the save all-or-nothing (FR-030) (FR-001, FR-001a, ID-05, NAMES-03)
- [ ] T041 [US1] Build the Add or edit a name screen (three boxes, Use selector) in `crates/meep-web/templates/name_form.html` and `crates/meep-web/src/routes/name_form.rs`, with errors tied to their field, announced to screen readers, focus moved to the first error, the "Fix a typo" or "Add a new name" choice on edit, and the "Keep mine" or "Reload" stale-save warning, and message IDs added to `crates/meep-web/locales/en/main.ftl` (contracts/ui-screens.md Screen 2, R10) (FR citations: FR-001) (FR citations: FR-031) **[DEP-010]**
- [ ] T042 [US1] Build the Names list screen with its empty state in `crates/meep-web/templates/names_list.html` and `crates/meep-web/src/routes/names_list.rs` (contracts/ui-screens.md Screen 1) **[DEP-010]**
- [ ] T043 [US1] Add plain-language limit and integrity error messages (short sentences, no jargon, each saying what to do next, FR-034) to `crates/meep-web/locales/en/main.ftl` and map them in `crates/meep-web/src/routes/name_form.rs` (NAMES-03, ID-04) (FR citations: FR-001a) (FR citations: FR-032) **[DEP-010]**
- [ ] T044 [US1] Run the acceptance suite and quickstart scenarios 1 to 3, 13 and 14; fix until green (ENG-02) Scenarios that need a screen wait for the screens (BUILD-01); domain-level acceptance runs now against the fakes

**Checkpoint**: a name can be added and listed; US1 works on its own.

---

## Phase 4: User Story 2 - Keep several names and say which are in use (Priority: P1)

**Goal**: Legal, Preferred, Also known as and Nickname names, Active or No longer used, with corrections and deletion that respect R8.

**Independent Test**: create the five fixtures, retire one, replace the Preferred, promote one to Legal, correct a typo and delete a name; lists, statuses and the log behave as stated (quickstart scenarios 10 to 12, 15, 16).

### Tests for User Story 2 (write first)

- [ ] T045 [P] [US2] Unit tests in `crates/meep-core/tests/uniqueness.rs`: exactly one Legal, at most one Preferred, any number of Also known as and Nickname, a second Legal refused with an explanation; cite data-model.md Name.use (R12) (FR citations: FR-007)
- [ ] T046 [P] [US2] Unit tests in `crates/meep-core/tests/preferred_fallback.rs`: a blank Preferred falls back to Legal in lists and exports (ID-01) (FR citations: FR-008)
- [ ] T047 [P] [US2] Unit tests in `crates/meep-core/tests/status_rules.rs`: Legal and Preferred cannot be retired; several names may be Active; retiring keeps the name listed and greyed; an optional end date is accepted; "Mark as in use again" returns the name to Active, clears the end date and emits `NameReactivated` (FR-028); cite data-model.md Name.status and Name.period (ID-01, ID-02) (FR citations: FR-006, FR-011)
- [ ] T048 [P] [US2] Unit tests in `crates/meep-core/tests/replace_and_promote.rs`: replacing Preferred turns the old one into an Active Also known as (R13); "Make this my legal name" promotes and turns the old Legal into an Active Also known as (ID-01) (FR citations: FR-009, FR-010)
- [ ] T049 [P] [US2] Tests in `crates/meep-core/tests/name_edit_fingerprint.rs` (against the fakes now; against the real foundation once T015 lands, **[DEP-002]**): an in-place correction keeps only the new wording; the `NameEdited` entry has the date, the role and position of each changed part and the value returned by `KeyCustody::purpose_value` for the prior wording, never the old text; the value is stable for the same input and differs by purpose; names holds no key material and computes no HMAC (NAMES-02, FOUND-01, quickstart scenario 15) (FR citations: FR-002a)
- [ ] T050 [P] [US2] Tests in `crates/meep-core/tests/name_delete.rs` (against the fakes now; against the real foundation once T015 lands, **[DEP-002]**): deleting a name makes it unreadable, a later read returns `NotFound`, and the log shows only the deletion and the date; the raw-byte proof that content and key are gone is the foundation's destruction scan (foundation T065), which names relies on rather than repeating; backups made before deletion are documented as still containing the name (R8, FOUND-01, quickstart scenario 16) (FR citations: FR-033)
- [ ] T051 [P] [US2] Acceptance feature `tests/acceptance/features/names_lifecycle.feature`: Given/When/Then for R12, R13, ID-01, ID-02 and promotion (PLAN-12)

### Implementation for User Story 2

- [ ] T052 [US2] Implement name status as an enum with guarded transitions, including reactivation that clears the end date, in `crates/meep-core/src/name/status.rs`; cite data-model.md Name.status and Name.period (State pattern, ID-02, PLAN-10) (FR citations: FR-006, FR-011) (FR citations: FR-028)
- [ ] T053 [US2] Implement uniqueness, fallback, replace and promote rules in `crates/meep-core/src/name/rules.rs`, emitting `PreferredReplaced`, `NamePromoted`, `NameRetired` and `NameReactivated` (R12, R13, ID-01, PLAN-10) (FR citations: FR-007, FR-008, FR-009, FR-010) (FR citations: FR-028)
- [ ] T054 [US2] Implement the names side of the correction in `crates/meep-core/src/name/fingerprint.rs` and the correction path: call `KeyCustody::purpose_value(record_id, "name-edit", prior wording)` to get the non-reversible fingerprint, keep only the new wording, and emit `NameEdited`; the keyed computation belongs to the foundation (foundation T068) **[DEP-002]**; names never receives key material and computes no HMAC (NAMES-02, FOUND-01, FR-012) (FR citations: FR-002a) (FR citations: FR-030)
- [ ] T055 [US2] Implement the names side of deletion in `crates/meep-core/src/name/service.rs`: call `RecordStore::delete`, which destroys the record's key and content inside one transaction (foundation T069), and emit `NameDeleted` with only the date; names does not call `destroy_record_key` itself and does not touch the shredding primitives **[DEP-002]**; the foundation's destruction scan proves nothing is recoverable (R8, FOUND-01) (FR citations: FR-030, FR-033)
- [ ] T056 [US2] Add the list actions (retire, make legal, replace Preferred, edit, delete with confirmation), "Mark as in use again" on a retired name's menu, greyed retired rows showing the text "No longer used" with muted styling that meets 4.5:1 contrast to `crates/meep-web/templates/names_list.html` and routes (contracts/ui-screens.md Screen 1, ID-01) (FR citations: FR-006, FR-010) (FR citations: FR-028, FR-032) **[DEP-010]**
- [ ] T057 [US2] Wire all names events through the foundation so each lands in the same transaction as its record change: register the kinds (T014) with `ChangeLog::register_kind` and pass them as the `events` argument of `RecordStore::save`, `update` and `delete` in `crates/meep-core/src/name/service.rs`; the foundation validates every event (no content, no keys, FR-014) and fails the whole call if one is invalid (FOUND-08, contracts/domain-events.md) **[DEP-002]**
- [ ] T058 [US2] Reactivation is a feature (owner decision, `docs/decisions/NAMES-06.md`): confirm T047, T052, T053 and T056 cover "Mark as in use again", the cleared end date and the `NameReactivated` event, and that no other path reactivates a name (ID-02, data-model.md State transitions) (FR citations: FR-028)
- [ ] T059 [US2] Run the acceptance suite and quickstart scenarios 10 to 12, 15 and 16; fix until green (ENG-02) Scenarios that need a screen wait for the screens (BUILD-01); domain-level acceptance runs now against the fakes

**Checkpoint**: US1 and US2 together give a usable names list. Recommended first demonstrable increment.

---

## Phase 5: User Story 3 - Handle names that do not fit three boxes (Priority: P2)

**Goal**: two surnames, particles, prefixes, suffixes, initials, origins, the full-name escape hatch and a custom short name.

**Independent Test**: enter "María José García López" as two surnames and "Jordan Lee Pak" with a birth origin; each stores the right labeled parts (quickstart scenarios 4 and 17).

### Tests for User Story 3 (write first)

- [ ] T060 [P] [US3] Unit tests in `crates/meep-core/tests/two_surnames.rs`: two separate last names with optional father's and mother's labels store two ordered family parts; cite data-model.md Name part.qualifier (ID-07, FR-012)
- [ ] T061 [P] [US3] Unit tests in `crates/meep-core/tests/qualifiers.rs`: initial-only keeps its period; origin birth or spouse is owner-set; "the name I go by" is recorded; prefix, suffix and particle ("de", "del", "van") sit in their own roles and rebuild in order; cite data-model.md Name part.role and Name part.qualifier (ID-05) (FR citations: FR-013, FR-015)
- [ ] T062 [P] [US3] Unit tests in `crates/meep-core/tests/labeler.rs`: the full-name box with tap-to-label produces the same labeled parts as three-field entry for every fixture (AT-27) (FR citations: FR-014)
- [ ] T063 [P] [US3] Unit tests in `crates/meep-core/tests/conventions.rs`: only the US and Spanish two-surname conventions are offered (ID-07) (FR citations: FR-016)
- [ ] T064 [P] [US3] Unit tests in `crates/meep-core/tests/short_form.rs`: default is first given name plus first-surname initial and period ("Jordan R.", "María G.") from Preferred, else Legal; an owner override wins; display only, never used for matching or in outbound exports; cite data-model.md Name.short_form (R15, NAMES-04) (FR citations: FR-021)
- [ ] T065 [P] [US3] Acceptance feature `tests/acceptance/features/names_options.feature` for AT-27 and the short-name behaviour (PLAN-12)

### Implementation for User Story 3

- [ ] T066 [US3] Extend roles and qualifiers in `crates/meep-core/src/name/model.rs`; cite data-model.md Name part.role and Name part.qualifier (ID-04, ID-05, ID-07) (FR citations: FR-012)
- [ ] T067 [US3] Implement the tap-to-label full-name parser in `crates/meep-core/src/name/labeler.rs` (AT-27, FR-014)
- [ ] T068 [US3] Implement the default and override short form in `crates/meep-core/src/name/short_form.rs`; cite data-model.md Name.short_form (R15, NAMES-04) (FR citations: FR-021)
- [ ] T069 [US3] Build the per-field ⋯ menus (labeled for screen readers) labeled "More options for <field name>" with targets of at least 44 by 44 px, and "More name options" in `crates/meep-web/templates/name_form.html`, offering only the US and Spanish two-surname conventions; use the shared menu element once **[DEP-010]** is delivered (FR-016, ID-05, ID-07, WCAG 2.2 SC 2.5.8) (FR citations: FR-013) (FR citations: FR-032)
- [ ] T070 [US3] Make tap-to-label keyboard-operable (each word a button reached with Tab, toggled with Space or Enter, FR-014); add the Short name box labeled by purpose with the computed default as placeholder, and the full-name box (384 limit), to `crates/meep-web/templates/name_form.html`; cite data-model.md Name.short_form (NAMES-04, NAMES-03) (FR citations: FR-001a, FR-014) (FR citations: FR-032) **[DEP-010]**
- [ ] T071 [US3] Run the acceptance suite and quickstart scenarios 4 and 17; fix until green (ENG-02) Scenarios that need a screen wait for the screens (BUILD-01); domain-level acceptance runs now against the fakes

**Checkpoint**: unusual names can be recorded without forcing a wrong shape.

---

## Phase 6: User Story 4 - Names survive accents, saving and export (Priority: P2)

**Goal**: accents preserved, matching key, FHIR and USCDI outbound exports, and the owner backup.

**Independent Test**: save "María José García López", export in each format, re-import, and compare character by character (quickstart scenarios 5, 6, 9, 10).

### Tests for User Story 4 (write first)

- [ ] T072 [P] [US4] Unit tests in `crates/meep-core/tests/accents_roundtrip.rs`: a name with accents survives save, display, export and re-import unchanged (AT-26)
- [ ] T073 [P] [US4] Unit tests in `crates/meep-core/tests/match_key.rs`: accent-folded key cases (precomposed and combining forms, mixed case), matching only and never altering stored text; cite data-model.md Name.match_key (ID-04, research.md open item 2) (FR citations: FR-020)
- [ ] T074 [P] [US4] Unit tests in `crates/meep-core/tests/export_from_parts.rs`: every export is built from parts; anything a target cannot hold is logged as an Also known as rendering (AT-29, FR-017)
- [ ] T075 [P] [US4] Unit tests in `crates/meep-core/tests/export_uscdi.rs`: three-field names with no menu changes export First, Middle, Last unchanged; prefix and suffix in their own elements; a retired name's end date goes into the FHIR period and is omitted and logged where the format cannot hold it (AT-30, FR-017, FR-018a)
- [ ] T076 [P] [US4] Unit tests in `crates/meep-core/tests/export_fhir.rs`: use mapping Legal official, Preferred usual, Nickname nickname, Also known as usual while Active and old when No longer used (ID-10, FR-018)
- [ ] T077 [P] [US4] Unit tests in `crates/meep-core/tests/outbound_rules.rs`: retired names are included in outbound exports ("old" and Previous Name); the outbound-sharing flag defaults to share and is respected; short forms are omitted; deleted names never appear in later exports; exports are snapshots; cite data-model.md Name.share_outbound and Name.short_form (NAMES-01, NAMES-04) (FR citations: FR-018a, FR-018b) (FR citations: FR-033)
- [ ] T078 [P] [US4] Unit tests in `crates/meep-core/tests/json_backup.rs`: the owner backup always includes every name (including No longer used), short forms, sharing flags and pronoun settings; export then import produces equal records; import restores only into an empty store, a non-empty store warns and needs "Replace everything", an import breaking a limit or the one-Legal rule is refused whole with every problem listed, and an imported sharing flag keeps its value (FR-019, FR-024a, principle 27) (FR citations: FR-029)
- [ ] T079 [P] [US4] Acceptance feature `tests/acceptance/features/export.feature` for AT-26, AT-29 and AT-30 (PLAN-12)

### Open items to settle (decisions, not code)

- [ ] T080 [US4] Obtain the exact USCDI Previous Name element layout (the owner-verified source blocks automated checks) and fix the JSON backup field names; update `specs/001-identity-names/contracts/export-formats.md` and `docs/decisions/PLAN-09.md` (NAMES-01, PLAN-09, research.md open item 3)
- [ ] T081 [P] [US4] Check whether FHIR R6 changes HumanName and which FHIR release the target systems require; record the outcome in `specs/001-identity-names/research.md` and `docs/decisions/PLAN-09.md` (ID-10, PLAN-09, research.md open item 4)

### Implementation for User Story 4

- [ ] T082 [US4] Implement the `NameExporter` trait (Strategy) in `crates/meep-core/src/export/mod.rs` with an outbound entry point that skips names whose data-model.md Name.share_outbound is false and omits data-model.md Name.short_form, and a backup entry point that includes everything (FR-017, FR-018b, NAMES-01, NAMES-04, PLAN-09, PLAN-10) (FR citations: FR-033)
- [ ] T083 [P] [US4] Implement the FHIR R4 HumanName exporter in `crates/meep-core/src/export/fhir.rs` (ID-10, FR-018)
- [ ] T084 [P] [US4] Implement the US health-data exporter with Previous Name for retired names in `crates/meep-core/src/export/uscdi.rs` once T080 settles the layout, honoring data-model.md Name.share_outbound through the trait's outbound entry point (NAMES-01, FR-017, FR-018b) (FR citations: FR-018a)
- [ ] T085 [P] [US4] Implement names' `BackupFragment` in `crates/meep-core/src/export/names_fragment.rs`: export from one snapshot, the machine-readable schema, names' section of the read-me (how fields map to the standard name formats), validation of an imported fragment (limits, one Legal and one Preferred) returning every problem, and apply through `RecordStore`; the zip, manifest, restore flow and rescue handling are the foundation's (foundation T088 and the restore tasks) **[DEP-002]** (FR-019) (FR citations: FR-024a) (FR citations: FR-029)
- [ ] T086 [US4] Implement the accent-folded match key in `crates/meep-core/src/name/match_key.rs`; cite data-model.md Name.match_key (ID-04) (FR citations: FR-020)
- [ ] T087 [US4] Add the outbound export and backup actions to `crates/meep-web/src/routes/export.rs` and the Names screen (downloads only; nothing is sent anywhere) (R5, EGRESS-01) **[DEP-010]**
- [ ] T088 [US4] Run the acceptance suite and quickstart scenarios 5, 6, 9 and 10; fix until green (ENG-02) Scenarios that need a screen wait for the screens (BUILD-01); domain-level acceptance runs now against the fakes

**Checkpoint**: names export correctly and the owner's own backup is always complete.

---

## Phase 7: User Story 5 - Pronouns stay hidden until I opt in (Priority: P3)

**Goal**: pronouns appear nowhere until enabled; visibility "Only me" or "Included in exports"; the owner backup always includes them.

**Independent Test**: on a fresh install no pronoun appears anywhere; enable them and check visibility, exports and the backup (quickstart scenarios 7 to 9).

### Tests for User Story 5 (write first)

- [ ] T089 [P] [US5] Acceptance feature `tests/acceptance/features/pronouns.feature` for AT-23: a fresh install shows no pronoun on any screen or export (PRON-01, R4) (FR citations: FR-022)
- [ ] T090 [P] [US5] Unit tests in `crates/meep-core/tests/pronoun_setting.rs`: default off; visibility defaults to `only_me` on first enable; only two choices offered; the stored value can hold `delegates` but v1.0 never offers or writes it, and adding a `delegates` value changes no existing stored record (tested); cite data-model.md Pronoun setting.enabled, Pronoun setting.choice and Pronoun setting.visibility (PRON-01a) (FR citations: FR-024)
- [ ] T091 [P] [US5] Unit tests in `crates/meep-core/tests/pronoun_exports.rs`: outbound exports include pronouns only if enabled and visibility is `in_exports`; the owner backup always includes them; turning pronouns off hides them everywhere again (PRON-01a, FR-024a)

### Implementation for User Story 5

- [ ] T092 [US5] Implement the pronoun setting in `crates/meep-core/src/pronouns.rs`; cite data-model.md Pronoun setting.enabled, Pronoun setting.choice and Pronoun setting.visibility; emit `PronounSettingChanged` with no pronoun text (PRON-01, PRON-01a) (FR citations: FR-022, FR-023, FR-024)
- [ ] T093 [US5] Add the pronouns section to Settings in `crates/meep-web/templates/settings_pronouns.html` and `crates/meep-web/src/routes/settings.rs` (contracts/ui-screens.md Screen 3); use the shared Settings page once **[DEP-010]** is delivered (FR citations: FR-023, FR-024)
- [ ] T094 [US5] Add a template guard so no template prints a pronoun unless the setting is enabled, using the name or "you" otherwise (PRON-01, AT-23) (FR citations: FR-022) **[DEP-010]**
- [ ] T095 [US5] Run the acceptance suite and quickstart scenarios 7 to 9; fix until green (ENG-02) Scenarios that need a screen wait for the screens (BUILD-01); domain-level acceptance runs now against the fakes

**Checkpoint**: all five stories work independently.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [ ] T096 Run all 18 quickstart scenarios in `specs/001-identity-names/quickstart.md` and record the results in that file; also time how long adding a Legal name takes (SC-001) and ask the owner to run the first-time-user check for SC-008, or to reword it, since it needs ten people (spec Verification)
- [ ] T097 [P] Run the accessibility checks (AT-41, AT-42, WCAG 2.2 AA) on every v1.0 names screen in both themes using axe-core through Playwright, plus a manual keyboard-only pass, focus order and visible focus, and reflow at 320 CSS px width and 400% zoom, once spec 010 delivers the shared elements **[DEP-010]** (SC-009, FR-032)
- [ ] T098 [P] Run `cargo deny check` and review the license report against principle 18 (PLAN-12, constitution)
- [ ] T099 [P] Run `.github/scripts/guard.sh` and a manual scan to confirm no personal data, email address or real record exists in code, fixtures, tests or docs (constitution, ENG-02)
- [ ] T100 Update `docs/decisions/PLAN-01.md` to `PLAN-12.md` for any choice that changed during implementation (for example `minijinja` or AES-GCM) and ask the owner to promote the ones they accept to Confirmed (principle 23)
- [ ] T101 Update `docs/decisions/README.md` and `README.md` with the run and test commands once they exist (ENG-02)
- [ ] T102 Run `/speckit-analyze` for cross-artifact consistency, then `/speckit-converge` to append any unbuilt work (principle 25)

---

## Dependencies & Execution Order

### Phase dependencies

- **Phase 1 (Setup)**: no dependencies; start immediately.
- **Phase 2 (Foundational)**: needs Phase 1; **blocks all user stories**. Build order (BUILD-01): spec 002, then spec 010, then the names screens. Domain and core tasks (ports, fakes, events, address, text rules, ids, the story logic) proceed now against the fakes. Every **[DEP-010]** task (all screens, the web server skeleton T022 and T023, the Fluent helper and no-literal-text checks T024 and T025) waits for specs 002 and 010; T015 and T057's real wiring wait for spec 002.
- **Phases 3 to 7 (stories)**: each needs Phase 2. Stories can then proceed in priority order or in parallel by different people.
- **Phase 8 (Polish)**: needs the stories you intend to ship.

### User story dependencies

- **US1 (P1)**: only Phase 2. The MVP.
- **US2 (P1)**: only Phase 2; uses the US1 model and repository, so it follows US1 in practice.
- **US3 (P2)**: extends the US1 model and screen; independently testable on its own fixtures.
- **US4 (P2)**: needs the US1 model; T080 and T081 gate T084 and the R6 target.
- **US5 (P3)**: only Phase 2; independent of US1 to US4 except for the export interaction tested in T091 (needs the US4 exporters).

### Within each story

Tests first (they must fail), then models, then services, then screens, then run the suite. Do not start the next story's implementation tasks until the previous story's checkpoint is green, per the one-feature-at-a-time approach (APPROACH).

### Parallel opportunities

- Phase 1: T002 to T007, T009 and T010 in parallel after T001.
- Phase 2: T011, T012, T013, T017, T019, T021, T022, T024, T025, T026 and T027 can run together; T014, T018, T020, T023 follow their tests.
- Each story's `[P]` test tasks are independent files and can be written together.
- US4: T083 to T085 in parallel once T082 and T080 are done.

### Example: parallel test authoring for User Story 1

```text
T028 name_builder.rs      T029 name_required_parts.rs   T030 compound_surname.rs
T031 given_names.rs       T032 spacing.rs               T033 limits.rs
T034 rebuild_property.rs  T035 add_name.feature
```

## Implementation Strategy

1. **MVP**: Phase 1, Phase 2, then US1. The domain-level MVP (model, builder, validation, service, acceptance tests against the fakes) can be built now; the screen part of US1 follows specs 002 and 010 (BUILD-01). Validate with quickstart scenarios 1 to 3, 13, 14 once the screens exist.
2. **First useful increment**: add US2 (several names, corrections, deletion). This is where R8 and NAMES-02 are proven.
3. Add US3, then US4 (resolve T080 and T081 first), then US5.
4. After each story: run the full suite, open a pull request, and merge only when `all-green` passes (ENG-03).
5. Build order (BUILD-01): spec 002, then spec 010, then the names screens. While those are built, the names domain and core work proceeds against the ports and fakes; the **[DEP-002]** and **[DEP-010]** tasks are done as each lands. Nothing in this list designs those specs, and no throwaway placeholders are built.

## Notes

## Phase 7b: Input validation (SEC-05, LOG-01)

**Purpose**: the name input rules the owner confirmed on 2026-11-20. Lives in `crates/meep-core/src/text.rs` and `name/validation.rs` with the existing validation tasks; needs the foundation's typed log API (foundation T131) for rule IDs **[DEP-002]**.

- [ ] T103 **[OWNER QUESTION]** Ask the owner whether the full-name box (384 graphemes) has its own byte limit and what it is (SEC-05 gave 512 bytes per name part only); record the answer in SEC-05 and FR-001b; build nothing on a guess
- [ ] T104 [P] Write failing tests in `crates/meep-core/tests/nameval_rules.rs`: a table of every rule `NAMEVAL-01` onward (limits, the 512-byte cap, control characters including null, bidi overrides and isolates U+202A to U+202E and U+2066 to U+2069, noncharacters) where each rule has one ID, one Fluent message ID and one fixture; a rejected box does not block saving other valid boxes; the rejection message cites the rule ID (FR-001b, FR-001c, FR-001d, SEC-05 item 4)
- [ ] T105 [P] Write failing tests in `crates/meep-core/tests/accept_rules.rs`: ZWJ and ZWNJ are accepted; apostrophes, hyphens, spaces, accents, non-Latin scripts, emoji ZWJ sequences and Turkish İ and ı are never rejected (FR-001c, SEC-05 item 4)
- [ ] T106 Implement the rule table in `crates/meep-core/src/name/rules.rs` with stable `NAMEVAL-nn` IDs used by validation, the Fluent messages in `crates/meep-web/locales/en/main.ftl` and the log calls (rule ID and field reference only, never the text); extend `text.rs` and `name/validation.rs` for the byte cap and character rules (FR-001b, FR-001c, FR-001d, LOG-01 item 4) **[DEP-002]**
- [ ] T107 [P] Create the hostile-input fixture corpus `tests/fixtures/names/hostile.json` (synthetic): `Robert'); DROP TABLE Students;--`, `O'Brien`, `<script>alert(1)</script>`, text with U+202E, Zalgo text, a null byte, 10,000 characters, NFD "José", an emoji ZWJ sequence, Turkish İ and ı, each marked valid or invalid with the expected rule ID (SEC-05 item 6)
- [ ] T108 [P] Write acceptance tests in `tests/acceptance/features/name_input_safety.feature` and its steps: every valid fixture round-trips byte-identical after save and read (accepted input is stored as normalized, and the test states the normalized form it expects), and every invalid fixture is rejected with the right rule ID; the hostile text is escaped in the rendered page (SEC-05 item 6, ENG-02)
- [ ] T109 [P] Write property tests with `proptest` in `crates/meep-core/tests/validator_props.rs`: normalization is idempotent, validation never panics on any Unicode string, a string accepted once is accepted after normalization, no accepted string contains a forbidden character, and counts never exceed the limits (SEC-05 item 6)
- [ ] T110 [P] Add `cargo-fuzz` targets `fuzz/fuzz_targets/normalize.rs` and `validate_name.rs` for the normalizer and validator, a short run in CI and a longer scheduled run (SEC-05 item 6, VZ-01 item 5)

- Out of scope and not tasked: ID-06, AT-24, AT-25, AT-32, v1.1 and later, login, delegates, AI, cloud (spec Out of Scope).
- Open questions that need the owner: T026 (recovery-key format), T058 (reactivation), T080 (USCDI layout), T100 (promoting PLAN decisions).
- All fixtures are synthetic; no real personal data or email addresses (constitution).
