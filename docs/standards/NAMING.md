# Meep naming standards

Status: Proposed (record as STD-01). Drafted 2026-11-21 21:10 CT.

**Rule zero: codify what exists.** Where the repo already has a working pattern, this file adopts it. Renaming existing things is a separate, explicit task, never a side effect of other work.

---

## 1. Decision records (`docs/decisions/`)

- **ID:** `PREFIX-NN`, two digits, zero-padded (`LOG-01`). Use three digits only after 99.
- **File:** `<ID>.md`, matching the ID exactly.
- **Status words (closed list):**
  - `Proposed`
  - `Confirmed`
  - `Confirmed (vNext)`
  - `Superseded by <ID>`
  - `Rejected`
- **Amendments:** a change to a Confirmed record gets a new record that says "Amends <ID>", unless the owner says to edit in place. Don't create new letter suffixes like `PRON-01a`; existing ones stay as they are.
- **Retired series:** `R` (R1–R17) is closed. Don't add new R numbers.
- **Gaps to fill:** R5, ENG-03 and ARCH-02 are referenced but have no file. Create them, copying the master-spec text, with no new content.
- **New prefixes need owner approval.** Add each new prefix to the registry below in the same PR.

**Prefix registry**

| Prefix | Covers |
|---|---|
| ACCESS | Access layers: custody, local access, identity, authorization |
| AUTH | Sign-in and identity (vNext) |
| BUILD | Build order and release sequencing |
| COMMIT | Commit identity (existing file `COMMIT-ID.md` keeps its name) |
| CONST | Constitution principles |
| DRAFT | Unsaved-change drafts (vNext) |
| ENG | Engineering process rules |
| ERR | Error and panic handling |
| FOUND | Foundation (spec 002) |
| LANG | Language: implementation language (LANG-01) and UI language and localization (LANG-02+) |
| LOCK | App auto-lock (vNext) |
| LOG | Logging |
| NAMES | Names feature (spec 001) |
| PLAN | Implementation-plan choices |
| PRON | Pronouns |
| R | v1.0 readiness, closed series |
| SEC | Security controls |
| STD | Standards, including this one |
| UI | Shared interface behavior (spec 010) |
| USER | User model (vNext) |
| VAL | Field validation pattern |
| VZ | Vulnerability gate and AI review |

---

## 2. Spec-local IDs

These IDs are numbered within each spec, so a reference from outside the spec must name it: `002:FR-020`, `001:T103`. Inside the spec itself, the bare ID is fine.

| Kind | Pattern | Example |
|---|---|---|
| Requirement | `FR-NNN` | `FR-052` |
| Task | `TNNN` | `T103` |
| Checklist item | `CHKNNN` | `CHK050` |
| Owner question | `QN` | `Q2` |

Global IDs are numbered across the whole project and are never prefixed with a spec:

| Kind | Pattern | Example |
|---|---|---|
| Acceptance test | `AT-NN` | `AT-28` |
| Validation rule | `<DOMAIN>VAL-NN` | `NAMEVAL-03`. Future: `DATEVAL-01`, `MEMBERVAL-01` |
| Diagnostic code | `E-<AREA>-<CONDITION>`, uppercase, every multi-word condition hyphenated | `E-STORE-READ-ONLY`, `E-RECORD-NOT-FOUND` |

- **IDs are never reused.** A removed requirement, rule or code keeps its number, marked `Removed`.
- **Diagnostic codes are permanent.** They appear in owners' exported debug text, so once shipped they never change meaning.
- **Registry:** every diagnostic code is listed in `specs/002-foundation/contracts/diagnostic-codes.md` with its variant, meaning and owner message ID, and is never reused or redefined.

---

## 3. Specs and documents

- **Spec folders:** `specs/NNN-kebab-name`, using the master-spec number (`001-identity-names`, `002-foundation`, `010-shared-ui`).
- **Master spec copies:** `Meep-Spec-v<build>_<YYYY-MM-DD>_<HHMM>-CT.md`. This pattern already exists. The newest timestamp is current.
- **Other dated documents** (reviews, directives, reports): `Meep-<Kind>-<Topic>_<YYYY-MM-DD>_<HHMM>-CT.md`. Kinds: `Review`, `Directive`, `Report`.
  - Example: `Meep-Review-Access_2026-11-20_0848-CT.md`.
  - Both existing files get renamed on commit: the access review to that name, and `ORC-Directive_…` to `Meep-Directive-ORC_2026-11-21_2050-CT.md`.
  - Locations: reviews go in `docs/reviews/`, directives in `docs/directives/`.
- **Standing documents** (no date): `docs/standards/UPPER.md`, for example `docs/standards/NAMING.md`.

---

## 4. Git and GitHub

- **Branches:** `<type>/<id>-<short-kebab>`, where `<id>` is the main spec, task or decision ID, lowercase.
  - Types: `feat`, `fix`, `spec`, `docs`, `chore`, `ci`, `deps`.
  - Examples: `feat/002-t014-errors`, `spec/010-shared-ui`, `docs/err-01`.
- **PR titles:** `<Area>: <what changed> (<IDs>)`.
  - Example: `Foundation: error Display as diagnostic codes (ERR-01, 002:T014)`.
  - The squash commit uses the PR title, so the title is the commit message.
- **PR bodies** must have these sections: what changed, verified, not verified, conflicts, owner questions. (This is ORC directive Part A rule 14.)
- **CI job names:** kebab-case, matching existing jobs (`all-green`, `acceptance-web`).

---

## 5. Rust code

- **Crates:** `meep-<area>`, kebab in Cargo and `meep_<area>` in code.
- **Modules and files:** `snake_case`. Errors live in `errors.rs`, the existing name.
- **Types:** `UpperCamelCase`.
  - ID newtypes: `<Thing>Id` (`RecordId`).
  - Each crate has one `Error` enum. Reason sub-enums are named `<Thing>Reason` (`AddressReason`).
- **Functions and variables:** `snake_case`. Constants: `SCREAMING_SNAKE_CASE`.
- **Cargo features:** lowercase, one word or kebab (`testing`, `failpoints`).
- **Environment variables:** `MEEP_<NAME>` (`MEEP_BIND`, `MEEP_FAILPOINTS`).
- **Unit test names:** `<behavior>_when_<condition>`, with the spec ID in a `///` doc comment on the test.
  - Example: `refuses_save_when_part_exceeds_byte_cap`, with `/// 001:FR-001a, NAMEVAL-04`.
- **Acceptance features:** `<spec>-us<N>-<kebab>.feature` (`002-us1-open-store.feature`). Each scenario title starts with its AT or FR ID.

---

## 6. Messages, logs and data paths

- **Fluent message IDs:** `<feature>-<thing>[-<state>]`, lowercase kebab (`store-wrong-key`, `backup-made`). This pattern already exists.
  - Feature prefixes: `store`, `history`, `backup`, `restore`, `address`, `save`, `request`, `names`, `ui`, `settings`.
- **Fluent variables:** lowercase. Multi-word names use kebab (`$first-seq`). Single words stay as they are (`$time`).
- **Locale files:** `locales/<bcp47>/<feature>.ftl` (`locales/en/foundation.ftl`). The test-only pseudo-locale code is `en-XA`.
- **Log event names:** `snake_case`, past tense (`notice_dismissed`, `record_saved`, `draft_discarded`). This pattern already exists.
- **FieldRef paths:** lowercase, dotted, using schema terms, with an index in brackets (`name.part[2].family`). Never a display label.
- **Record aliases:** use the format fixed by LOG-01. It is display-only, never a name and never derived from content.

---

## 7. Words people see

- **Product name** in the interface and docs: **Meep**. `MEEP` appears only in the GitHub org name `Project-MEEP`.
- **Interface terms come from the glossary** (master spec Glossary). A new term the owner will see needs owner approval before use. Terms already fixed:
  - Legal name, Also known as, Active / No longer used
  - vanity username, recovery key, owner backup, rescue backup
  - persisted session, notifications tray
- **Spec vocabulary:**
  - *owner*: the person whose records these are
  - *caregiver*: a person granted access, vNext
  - *user*: anyone using an instance, vNext user model

  Don't use them interchangeably.
- **Plain words over jargon** in every owner-facing message. For example, "this changed somewhere else", not "stale write".
