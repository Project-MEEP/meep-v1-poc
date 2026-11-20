# Meep Spec v1.0 (2026-11-19 22:22 CT)

Last changed 2026-11-19 22:22 CT · Samantha Schroff

## How to use this spec (for implementers)

Read this section first. It tells an implementer, human or AI, what is binding and what to build now.

1. **Build only v1.0.** v1.0 is the names feature plus the app shell, shared UI elements and minimum foundation described below. Everything labeled v1.1, v2, v3, v4 or Deferred is context, not work.
2. **Only "Confirmed" is binding.** "Proposed", "Open" and the defaults in the Readiness table are not decisions. If a default has not been confirmed by the owner, ask before building on it; do not silently adopt it.
3. **Principle 26 applies to code:** warn, never block, except for data integrity and authorization.
4. **No real personal data in the repo.** Use only the synthetic fixtures in this spec. Real data lives in a git-ignored `data/` folder.
5. **IDs** (ID-04, R7, AT-28 and so on) are stable references; use them in commit messages, tests and pull requests. Every ID is defined in this file; see the Glossary for open-question IDs.
6. **Pull requests only (ENG-03).** Never commit or push to `main`. Work on a feature branch, open a pull request, and merge only after every CI check is green.
7. **Ambiguity rule:** if this spec does not answer a question, stop and ask the owner. Do not fill gaps with assumptions or with any assistant's own platform preferences (principle 11).

### v1.0 scope at a glance

**In scope**

- App shell: header with app name and the sun/moon theme toggle; simple navigation between the screens below
- Screens: **Names** (list of the owner's names with type and status), **Add or edit a name** (three fields, ⋯ menus, "More name options", full-name escape hatch), **Settings** (theme: Light, Dark, Match my device; pronouns if R4 is confirmed)
- Names model and rules: ID-01 to ID-05, ID-07 to ID-10, PRON-01 (if R4), R12 to R16 once confirmed
- Exports: USCDI-style First/Middle/Last/Suffix, FHIR HumanName, and JSON export and import of all records
- Foundation: record envelope (R9), address parser (R6, R16), encrypted local database (R2, R7), append-only hash-chained change log, crypto-shredding on delete (R8), single owner with no login, server bound to the owner's own machine only (R5). The R items here are defaults until the owner confirms them
- Shared UI elements (UI-01) meeting WCAG 2.2 AA in both themes
- Interface text in Fluent message files, English only (R10)
- CI workflow and tests: unit tests plus the ★ acceptance tests in Verification

**Out of scope for v1.0** (do not build)

- Login, SSO, 2FA, delegates, caregivers, roles beyond the single owner
- Insurance cards, documents, OCR, barcodes, any AI or persona feature, any cloud call
- Outside-record linking (ID-06) and its tests (AT-24, AT-25, AT-32)
- Android, iOS, native Windows; in-app updates; release signing beyond R17
- Languages other than English; font size, menu size, text reader and haptics settings from DOD-16
- Any name convention other than US and Spanish two-surname

## Overview

Meep v1 is close to buildable: v1.0 (the names feature, in Rust, web first) has every Readiness decision confirmed (R1 to R17, 2026-11-19) and is clear to start coding. One item, R7 key custody, is revisited before v1.1.

Meep (Alpha.Meep, product line v1) is a free, MIT-licensed, user-owned system of record and coordination. It takes the administrative work of care off patients, families and caregivers. It is a librarian, archivist, tracker and coordinator. It is not a clinician, pharmacy, legal verifier or EHR, and it claims no authority over any record it holds.

**v1 audience:** adults in mutual-support relationships who keep the cognitive ability to revoke a delegation. Many will be older or not native English speakers. Children wait for v4; wards for v3 or v4.

**v1 platforms (Confirmed 2026-11-19):** a web browser client and an Android app. iOS and native Windows are Deferred.

**Approach (Confirmed 2026-11-19):** v1 ships one feature per point release (v1.0, v1.1, …). Each feature is kept, not thrown away, and the next starts only once the prior one is feature complete and merged to main with all tests green. Spec Kit runs per feature.

**Publishing identity:** Samantha Schroff. Code: github.com/Project-MEEP/meep-v1-poc (public, owned by the Project-MEEP GitHub organization; empty as of 2026-11-19 20:28 CT).

**Spec naming (Confirmed 2026-11-19):** a spec carries the build version it targets plus the date and time it was last changed, e.g. "Meep Spec v1.0 (2026-11-19 22:22 CT)". The newest time is the current copy. When v1.0 ships, a new spec starts as v1.1.

| Status | Meaning |
| --- | --- |
| Confirmed | Set or corrected by the owner; binding |
| Proposed | Suggested; not approved |
| Open | Needs an owner decision |
| Deferred | Planned for later; never added to v1 silently |
| Superseded | Replaced by a later owner decision |

Sources: owner whiteboards and answers; ChatGPT checkpoint rev 1.0 (2026-11-03); Claude review rev 1.1 (2026-11-08); owner session with Claude (2026-11-19, this spec). Only the owner marks anything Confirmed.

## Decisions made 2026-11-19

The owner settled 48 items in this session (32 below plus Readiness R1 to R4 and R6 to R17, confirmed 2026-11-19); each is binding unless marked otherwise.

| ID | Decision | Status |
| --- | --- | --- |
| REL-01 | v1.0 is a generic app shell with the full names feature and standard UI elements, for owner hands-on feedback | Confirmed |
| SPEC-NAME | Specs are named for the build version they target plus last-changed date and time | Confirmed |
| ENG-01 | Clean code (Robert C. Martin) and Gang of Four design patterns, adapted to Rust | Confirmed |
| ENG-02 | Every feature ships unit tests and acceptance tests; CI runs them on every push and pull request; main merges only when green | Confirmed |
| ENG-03 | Every change reaches main through a pull request, enforced by a branch protection rule; no direct pushes by anyone, including the owner and AI agents | Confirmed |
| UI-01 | Light/dark toggle button with a sun/moon icon in the app header in v1.0, plus a theme setting, for sensory comfort | Confirmed |
| PLAT-01 | v1 platforms are web browser and Android | Confirmed |
| PLAT-02 | iOS starts after the first stable long-term release, when the owner takes on the Apple developer program cost; native Windows Deferred | Deferred |
| APPROACH | v1 ships one feature per point release; nothing is disposed | Confirmed |
| DOD-ORDER | v1.0 is names; v1.1 is insurance-card intake, dogfooded on the owner's real card | Confirmed |
| LANG-01 | Implementation language is Rust | Confirmed |
| EGRESS-01 | AI extraction runs locally by default; cloud is opt-in per item | Confirmed |
| LICENSE-01 | MIT license | Confirmed |
| NAME-POL | Forks may not use the MEEP name; protected by a separate TRADEMARKS.md, not by editing MIT | Confirmed |
| PUB-ID | Public publishing identity is "Samantha Schroff" | Confirmed |
| P26 | Warn, never adjudicate: Meep never blocks an owner change on legal or factual grounds | Confirmed |
| P26a | All uploaded artifacts are informational; Meep claims no authority | Confirmed |
| ID-01 | Name types: Legal name, Preferred name, Also known as, Nickname | Confirmed |
| ID-02 | Name status is binary: Active or No longer used; several names may be Active | Confirmed |
| ID-03 | "Name style" setting | Superseded by ID-05 |
| ID-04 | Names are stored as labeled parts that rebuild the full text exactly | Confirmed |
| ID-05 | Entry defaults to US three fields (First, Middle, Last) with a ⋯ menu per field | Confirmed |
| ID-06 | Each outside record links to the name that organization has on file | Proposed |
| ID-07 | v1 name conventions: US plus Spanish two-surname; others Deferred | Confirmed |
| ID-08 | Multi-word last name defaults to one compound surname | Confirmed |
| PRON-01 | Pronouns hidden until the owner opts in under Settings | Confirmed |
| UPD-01 | In-app updates from an owner-run pipeline | Confirmed |
| DIST-01 | Signed, verifiable distribution to protect users from lookalike builds | Confirmed |
| V3-SIMPLE | v3 adds a simplified interface for cognitive impairment over the same structure | Deferred |
| COMMERCIAL | Commercializing or moving under Sandbox Servers | Deferred, no version |
| UPCOMING | "Upcoming" status for a pending legal name | Rejected; Active covers it |
| BUILD-01 | v1.0 build order: spec 002 (foundation), then spec 010 (shared UI elements), then the names screens on the real 010 elements; names domain work may proceed against port fakes; no throwaway placeholders | Confirmed |

**Added after the spec review (Confirmed 2026-11-19):** ID-09 spacing rule; ID-10 FHIR use mapping; v1.0 runs with no login, reachable only from the owner's own machine (R5); AT-24, AT-25 and AT-32 move to v1.1; AT-41 and AT-42 added for the theme toggle; v1.0 includes the minimum foundation listed under Engineering rules.

**Decisions recorded in the repository** (`docs/decisions/`, Confirmed 2026-11-19): R1 to R17 (Readiness); PRON-01a (pronoun visibility); NAMES-01 (retired names in outbound exports, sharing flag); NAMES-02 (corrections keep only the new wording); NAMES-03 (limits); NAMES-04 (short form); NAMES-05 (checklist resolutions, CHK001 to CHK039); NAMES-06 (reactivating a retired name); FOUND-01 (spec 002 owns key, storage, change-log-format and export-envelope primitives); CONST-27 (principle 27 Confirmed); COMMIT-ID (alias and its email accepted in commit metadata); BUILD-01 (v1.0 build order). The planning choices PLAN-01 to PLAN-12 are Proposed until the owner confirms them.

## Roadmap

v1 is at gate G1 (requirement baseline), now scoped per feature; v1.0 can enter G2 once the Readiness items are answered. Each version starts once the prior one operates and its entry decisions are made. No dates are set.

| Version | Contents | Entry condition (Proposed) |
| --- | --- | --- |
| v1 Alpha | Assistant platform, Dr. Office patient mode, identity and names, caregiver workspaces, vault, P2P; web + Android | One feature per point release: v1.0 names, v1.1 insurance intake |
| First stable long-term release | iOS app begins (PLAT-02); owner enrolls in the Apple developer program | v1 stable and operating |
| v2 | Optional cloud backup (BKP); caregiver and provider exchange roles (HCP-X); import from provider apps (PRN) | v1 passes G5; HCP-01 decided |
| v3 | Provider views and importable modules (HCP-V); Digital Ombudsman (OMB); simplified interface for cognitive impairment (V3-SIMPLE); wardship (WARD, v3 or v4) | v2 operating; OMB-01 defined |
| v4 | Minors and guardian governance (GOV-01) | Legal review and guardian model before G1 |
| Later, no version | Native Windows client; more name conventions (family-name-first, Portuguese, Filipino); multi-persona; Exec Mgmt integration; further domains (ARCH-02); emergency access to DNR and POA (ACCESS-01); commercialization | Owner places them |

**Gates:** G0 discovery (done), G1 requirement baseline, G2 architecture plan, G3 build readiness, G4 verified build, G5 operate.

## Constitution

These 27 principles bind every spec, plan and task; no assistant may override them. Each is Confirmed unless marked.

**Identity and ownership**

1. One human = one account = one logical instance, across any number of devices, apps and workers.
2. Every person owns their own data. Caregiver access never makes a patient's records the caregiver's.
3. Owner/Patient and Caregiver are base roles a person can hold separately or together. Admin is hidden, belongs to the app team only, and grants no general access to private records.
4. The owner grants and revokes all delegation, and can always revoke. An in-app role is application authorization, not proof of legal POA.
5. Caregiver access is least-privilege, task-scoped, view-only and temporary by default. Persistent copies need a separate, attributable opt-in.

**Structure**

6. Domain-driven design: Domain → Namespace → Repository, addressed `INSTANCE.NAME.Domain.Namespace.Repository`. The owner defines domains one at a time.
7. Repositories are data-agnostic; source, provenance and authority are modeled explicitly.
8. POA exists both as a domain (legal documents) and as an access role (label provisional).
9. Views are standardized by role and authorization state. MVC is the analogy, not a chosen framework.

**Runtime and AI**

10. The AI persona is a conversational skin over the application. It never holds authoritative permissions, data or workflow state.
11. Meep is AI-platform-agnostic. No vendor, model or product is assumed, and an assistant may not propose its own platform as a default.
12. Meep runs on its own owner-built, self-hosted system and harness, not on My Pal Clara or Indi.
13. Workers are replaceable; durable state survives the failure of any one of them.
14. Accounts interact peer-to-peer; access across accounts never transfers ownership.

**Security and privacy**

15. Secrets stay isolated from the AI persona; vault adapters act on scoped credentials. The persona never asks for a credential in conversation. *(Proposed, pending SEC-01)*
16. Encryption in transit and at rest, audit logs, threat modeling and a secure SDLC wherever feasible. No certification is claimed.
17. Patient workspaces never mix data, AI context or job results. The one exception is the minimized cross-patient appointment view (CARE-07).
18. Free and open source under MIT, standards first, no mandatory paid cloud or credential provider.

**Scope**

19. No diagnosis, treatment, dosing, triage or prescribing; Meep never presents itself as a provider.
20. Providers with their own systems document there. Meep imports and attributes; it never replaces their record.
21. v1 serves self-governing adults in mutual support. Minors wait for v4; wards for v3 or v4.
22. BYOx: if an adequate tool or standard exists, use it.

**Process**

23. Requirements, specifications and engineering recommendations stay separate. Only the owner marks anything Confirmed.
24. Whiteboards are transcribed verbatim before anyone interprets them.
25. Spec Kit order: constitution, specify, clarify, plan, checklist, tasks, analyze, implement, converge.

**Authority**

26. **Warn, never adjudicate.** Meep records, attributes and warns. It never blocks an owner's change on legal or factual grounds and never claims a document is valid. Every uploaded artifact is informational only. Meep still blocks for data integrity (a record that would be corrupt) and authorization (a delegate acting outside granted scope).
27. **No lock-in.** Every record can be exported in an open, documented format at any time, without Meep running.

## v1 features

v1 keeps all 17 whiteboard Definition of Done lines, adds identity and names, and narrows platforms to web + Android.

| ID | Feature | Status | Open points |
| --- | --- | --- | --- |
| DOD-01 | Add, remove, update calendar | Confirmed | Sources and conflict policy |
| DOD-02 | Add, remove, update contacts | Confirmed | Owner and caregiver books stay separate |
| DOD-03 | Add, remove, update credentials | Confirmed | Vault trust boundary (SEC-01) |
| DOD-04 | Add, remove, update email, messages, social media | Confirmed | Boundary vs Exec Mgmt social media (SCOPE-SM) |
| DOD-05 | Answer questions over all of the above | Confirmed | Source attribution; secrets never enter AI context |
| DOD-06 | Voice to text: verbatim transcript plus clean copy | Confirmed | LOC-02, LOC-03 |
| DOD-07 | Image, web page and content scanning via the AI platform du jour | Confirmed | Local default, cloud opt-in (EGRESS-01) |
| DOD-08 | Dr. Office patient-mode area | Confirmed | Administrative only |
| DOD-09 | Scribing and notes | Confirmed | User note vs provider clinical note |
| DOD-10 | Care-coordination follow-up | Confirmed | Includes "not yet on file with this provider" prompts |
| DOD-11 | Insurances | Confirmed | v1.1 |
| DOD-12 | Care-team address book | Confirmed | Cross-account role boundaries |
| DOD-13 | Pharmacy | Confirmed | No dispensing or prescribing |
| DOD-14 | Home health | Confirmed | External source of record stays authoritative |
| DOD-15 | Patient journal: photo, video, text, date-time, ID | Confirmed | Retention and deletion (DATA-01) |
| DOD-16 | UX: Tsalagi, English, Spanish, "American"; themes; fonts; menu sizes; text reader; haptics; **web + Android** | Confirmed, platforms revised | UX-02; Spanish gender-neutral copy |
| DOD-17 | User ownership, bounded caregiver access, secure records, one working persona | Confirmed | Acceptance gates |
| ID | Identity and names (next section); v1.0 | Confirmed | ID-06 |
| DELEG | Delegation levels Read, Read + Update, Complete; owner can always revoke | Confirmed | ROLE-04 |
| CARE-07 | Cross-patient appointment view with stored short names | Confirmed | CARE-07a |
| WS | Caregiver workspaces: one tab per patient, no mixing | Confirmed | CARE-03 |
| JRN | Caregiver journal: private by default, per-entry sharing | Confirmed | CARE-05 |
| AUTH | SSO login with authenticator 2FA | Confirmed | Not in v1.0 (R5); passkeys and recovery (SEC-03) |
| VAULT | Local encrypted credential vault plus connectors | Confirmed | SEC-02 |
| ARCH-04 | One persona at first, model-agnostic | Confirmed |  |
| ARCH-06 | Owner-built, self-hosted system and harness | Confirmed | HOST-01 |
| P2P | Peer-to-peer between separately owned accounts | Confirmed | P2P-01 |
| HCP-RULE | Providers document in their own apps; Meep never becomes the clinical chart | Confirmed |  |

**Spanish copy rule (Proposed):** Spanish interface text uses gender-neutral phrasing by default, e.g. "También me conocen como" for Also known as.

## Identity and names (v1.0)

People enter names in three familiar fields; Meep stores them as labeled parts mapped to FHIR, so nothing downstream ever re-parses a name.

**Name types (ID-01).** Legal name, Preferred name, Also known as, Nickname. Each name is Active (in use now) or No longer used (ID-02); several can be Active during a transition. Legal and Preferred are always Active. "Make this my legal name" promotes another entry; the old legal name becomes an Active Also known as, which the owner may retire. Retired names stay visible, greyed out, so old records still match. Switching to No longer used offers an optional end date. Preferred name falls back to Legal if blank.

**Entry (ID-05).** Three fields: First name, Middle name, Last name. Each accepts several words. Only one of First or Last is required; Middle is never required and no filler like "NMN" is stored. Several words in Last default to one compound surname (ID-08).

Each field's ⋯ menu (labeled for screen readers, at least 44 × 44 px (owner decision NAMES-05; WCAG 2.2 SC 2.5.8 requires only 24 × 24)):

- Change a word's type; split or keep together
- Two separate last names (Spanish), with optional father's/mother's labels
- "This is the name I go by" (callme)
- Initial only
- Optional origin, owner-set: birth or from a spouse

"⋯ More name options" (text-labeled): prefix, suffix, particle (de, del, van), and "My name doesn't fit these boxes", which switches to one full-name box plus tap-to-label.

**Storage (ID-04).** A name = `text` (exactly as entered, Unicode NFC, accents kept), `use`, `active`, optional `period`, `source`, `short_form`, `pronunciation`, and ordered `parts`:

| Part role | FHIR | US form / USCDI |
| --- | --- | --- |
| `given` | `given[0]` | First |
| `given_additional` | `given[1..]` | Middle |
| family (seq; optional fathers/mothers/birth/spouse qualifier) | family + extensions | Last |
| particle | family prefix | Last |
| prefix / suffix | prefix / suffix | Prefix / Suffix |

Rules: the parts must rebuild `text` exactly or the save fails (integrity block). The AI receives parts as JSON, never the raw string. Every outside format is an export from the parts; anything a target system cannot hold is logged as an Also known as rendering. A separate accent-folded key is used for matching only. Matching suggests links but never merges without the owner (AT-25).

**Spacing (ID-09, Confirmed).** Leading and trailing spaces are trimmed and runs of spaces collapse to one; apostrophes and hyphens are kept as typed.

**FHIR use mapping (ID-10, Confirmed).** Legal → `official`; Preferred → `usual`; Nickname → `nickname`; Also known as → `usual` while Active, `old` once No longer used.

**Outside records (ID-06, Proposed, v1.1).** Each insurance card or provider record links to the name that organization has on file, and exports to it use that name. Retiring a linked name warns, naming the organization (AT-32).

**Implementation rules pending confirmation:** uniqueness of Legal and Preferred (R12), what happens to an old Preferred name (R13), how parts join into text (R14), the default short form (R15) and address segment syntax (R16). See Readiness.

**Pronouns (PRON-01).** Hidden everywhere until the owner enables them in Settings. Options: she/her, he/him, they/them, use my name, custom. Owner picks visibility: only me, delegates, or exports. When off, the interface uses the name or "you" and never guesses. Open: whether a caregiver can hide a patient's shared pronouns (PRON-02).

**Test fixtures (synthetic; real owner data stays in local test data only, never in the repo):**

| Name | Type | Status | Exercises |
| --- | --- | --- | --- |
| Jordan Lee Rivera Okafor (last = compound) | Legal | Active | Compound surname |
| Jordan Rivera | Preferred | Active | Fallback and short form |
| Jordan L. Rivera (as on a card) | Also known as | Active | Initial, document rendering |
| Jordan Lee Pak (last tagged birth) | Also known as | No longer used | Retired birth name |
| María José García López | Legal | Active | Compound given name, two surnames, accents |

## v1.1: insurance-card intake

v1.1 turns a photo of the front and back of an insurance card into a structured, owner-confirmed coverage record with its originals kept unchanged.

**In scope:** one owner, no delegates; v1.0's names model, since the card's name must link to one; local extraction; export and delete.

**Out of scope:** caregivers, P2P, provider exchange, eligibility checks with insurers, Q&A over records (DOD-05) (a later feature).

**Flow**

1. Owner adds front and back photos as one card. Originals are stored immutable.
2. Local OCR reads the text; the back barcode is decoded if present.
3. Fields are proposed, each with its printed text, cleaned value and confidence.
4. Owner reviews, corrects and confirms. Nothing is filed unconfirmed.
5. The record is filed at `Alpha.Meep.Medical.Insurance.<coverage>` with source and history, and linked to the matching name (a card-printed name becomes an Active Also known as).
6. Owner can export the record as JSON or delete it.

**Coverage record fields:** member name as printed, member ID, group number, plan sponsor (employer), administrator, network/product, claims routing ("file with your local plan"), phone numbers and URLs, preauthorization text kept verbatim plus a labeled list of required and may-require services, card capture date, valid-from/valid-to, and a link to any card that replaces it. Every field carries `authority: none` and its provenance.

**Data and code rules**

- Real card images and extracted data never enter git; tests use a synthetic card.
- Cloud AI is used only after a per-item yes from the owner (EGRESS-01), through a swappable adapter with no default vendor.

**Candidate crates (verify at G2):** `ocrs` (OCR), `rxing` (PDF417 barcode).

**Done when:** front and back are ingested; originals unchanged; fields extracted with confidence; owner confirms; record filed with provenance and linked name; export and delete work; AT-13, AT-24, AT-25, AT-32 and AT-37 to AT-40 pass on the synthetic card, and the owner signs off on her real card.

## Distribution, updates and signing

Lookalike builds are treated as a security threat: every genuine release is signed and verifiable, and existing installs accept only updates signed by the owner's release key.

**DIST-01: how each surface is protected**

| Surface | Protection | When |
| --- | --- | --- |
| Releases (server, web, Android) | Sigstore signature from the GitHub Actions build, recorded in the public Rekor log; `meep verify` checks an install | From the first public release |
| Official website and update server | Let's Encrypt HTTPS | Before the first public release |
| Android app | APK signed with the owner's release key; Play Store and/or F-Droid listing (cost and policy to confirm) | First Android release |
| Existing installs | Release public key built into the app | From the first public release |
| Native Windows installer | Azure Artifact Signing under the current legal name, SignPath as fallback; re-validate after a name change | Deferred with native Windows |

**User-facing notice (everywhere):** "Meep is always free. We will never charge you, call you, or ask for your passwords or insurance login. If 'Meep' asks for your password in a chat, it's fake." Plus a report-a-fake contact in `SECURITY.md`.

**Name policy:** MIT text stays unmodified; `TRADEMARKS.md` says forks may rebrand freely but not use the MEEP name or logo. Run a USPTO search before relying on the name; register before the first app-store launch.

**UPD-01: update rules**

1. Signature checked before install; unsigned or tampered updates are refused (integrity block).
2. No silent downgrades; a deliberate rollback warns but proceeds.
3. Snapshot before any data migration; a failed migration restores it automatically.
4. The user decides when to install; security fixes prompt louder; Meep keeps working offline on the old version.
5. Update checks send no account ID or device fingerprint and can be turned off.

**Platform limits:** the self-hosted server and web client update through the owner's pipeline. On Android, a Play Store build updates through the store (store policy, as understood; verify). The signing key stays off networked machines, ideally on a hardware key. TUF (Rust: `tough`) and Tauri's signed updater are candidates to verify at G2.

## Specs and open questions

Meep needs 22 feature specs (16 for v1); v1.0 touches 001, 002, 010, 015 and 016; v1.1 adds 006 and 011.

| Spec | Scope | Version | Draft state |
| --- | --- | --- | --- |
| 001 | Identity, accounts, roles, access; **names and pronouns (ID-01 to ID-10, PRON-01)** | v1.0 | Seed v0.4; add names |
| 002 | Domain, namespace, repository organization; **record envelope and address parser** | v1.0 | Seed v0.4 |
| 003 | Calendar, contacts, credentials, queries | v1 | Seed v0.4; no tests yet |
| 004 | Communications and cross-source queries | v1 | Seed v0.4; no tests yet |
| 005 | Accessibility, speech, content input, interpretation | v1 | Seed v0.4 |
| 006 | Patient-office administration and journal; **insurance intake** | v1.1 | Seed v0.4; add v1.1 |
| 007 | Peer-to-peer, delegation levels, POA | v1 | Seed v0.4; ROLE-04 floor |
| 008 | Persona, replaceable runtimes, queues, workers | v1 | Seed v0.4; remove GPT assumption |
| 009 | Adaptive assistance scaffold | v1 | Seed v0.4; not on the DoD board |
| 010 | UX, localization, **shared UI elements and theme toggle**, web + Android | v1.0 | Seed v0.4; revise platforms |
| 011 | Credential vault and connectors | v1.1 | Seed v0.4 |
| 012 | Caregiver workspaces and cross-patient view | v1 | Not drafted |
| 013 | Caregiver journal and sharing | v1 | Not drafted |
| 014 | Self-hosted system and harness | v1 | Not drafted; no tests yet |
| 015 | Distribution, updates, signing, **CI pipeline** (DIST-01, UPD-01, ENG-02) | v1.0 | New; CI drafted |
| 016 | Export, backup and no lock-in (principle 27, DATA-01) | v1.0 | New; not drafted |
| 101 | Optional cloud backup | v2 | Not drafted |
| 102 | Provider exchange and PRN import | v2 | Not drafted |
| 201 | Provider views and importable modules | v3 | Not drafted |
| 202 | Digital Ombudsman | v3 | Not drafted |
| 203 | Incapacity and wardship; simplified interface | v3 or v4 | Not drafted |
| 301 | Minors and guardian governance | v4 | Not drafted |

**Resolved since rev 1.1:** DOD-ORDER, LICENSE-01. **ID clean-up:** ROLE-06 now means only the open v1 question (what happens if an owner loses capacity mid-use); the deferral is WARD. CARE-07 is the Confirmed view; its open details are CARE-07a.

**Open, needed for v1.0 or v1.1 (G1/G2):** R7 revisit before v1.1 (recovery drill, hardware key as second unlock); plus HOST-01 (default: the owner's own machine), DATA-01 (retention per record type), SEC-02 (encryption choices), O-01 (where authoritative data lives), REG-01 (FTC breach rule vs HIPAA; depends on HOST-01), ARCH-05 (what F(x) means).

**Open, not blocking v1.0:** ROLE-04, ROLE-05, ROLE-06, ROLE-07, ROLE-08, SEC-01, SEC-03, SEC-04, CARE-03, CARE-05, CARE-06, CARE-07a, LOC-02, LOC-03, UX-02, UX-03, VOICE-02, ARCH-01, ARCH-02, ARCH-03, P2P-01, PERF-01, HCP-01, OMB-01, PRON-02 (can a caregiver hide a patient's shared pronouns?), SCOPE-SM (DOD-04 social media vs Exec Mgmt), ACCESS-01 (emergency access to DNR/POA vs lock-screen security).

## Verification

38 draft tests exist; none has been run or approved. ★ gates v1.0 (names); ◆ gates v1.1 (insurance). Specs 003, 004 and 014 still have no tests.

| Test | Scenario | Spec |
| --- | --- | --- |
| AT-01 | Adding a device never creates a second identity | 001 |
| AT-02 | Switching Patient and Caregiver roles leaves records and identity unchanged | 001 |
| AT-03 | A killed worker resumes elsewhere with no duplicate external action | 008 |
| AT-04 | Rapid patient-tab switching never crosses data, context or results (except AT-21) | 012 |
| AT-05 | A read-only caregiver's edit is denied and audited | 007 |
| AT-06 | A Read + Update delegate changes only the delegated task | 007 |
| AT-07 | Revoking access while a job is queued makes it re-check and stop | 007 |
| AT-08 | Closing a patient workspace leaves no app-managed copies | 012 |
| AT-09 | An unshared caregiver journal entry stays private and encrypted | 013 |
| AT-10 | Sharing one entry exposes only that entry, to that recipient | 013 |
| AT-11 | Editing the clean transcript never changes the verbatim original | 005 |
| AT-12 | A personal-lexicon mapping stays inside approved policy | 005 |
| AT-13 ◆ | A summary of a document keeps its source and gives no clinical advice | 006 |
| AT-14 | An unrelated persona tool asking for a secret is denied | 011 |
| AT-15 | Capture and copy protection works as documented; gaps disclosed | 010 |
| AT-16 | A provider import never replaces the external record | 102 |
| AT-17 | A cross-domain job reaches only authorized repositories | 002 |
| AT-18 | The UI shows pending, blocked and done honestly | 008 |
| AT-19 | Low- and high-assistance profiles keep the same records and permissions | 009 |
| AT-20 | Each delegation level behaves per the approved matrix | 007 |
| AT-21 | The aggregate appointment view shows only appointment fields and short names | 012 |
| AT-22 | Owner revocation always wins | 007 |
| AT-23 ★ | Pronouns appear nowhere until the owner enables them (if R4 = yes) | 001 |
| AT-24 ◆ | A delegate sees the legal name only when an authorized task needs it | 001 |
| AT-25 ◆ | A document name differing from all names on record raises a task and never auto-merges | 001 |
| AT-26 ★ | A name with accents round-trips through storage, export and display unchanged | 001 |
| AT-27 ★ | The full-name escape hatch yields the same labeled parts as three-field entry | 001 |
| AT-28 ★ | A name's parts always rebuild its full text exactly | 001 |
| AT-29 ★ | Every export comes from the parts; anything a target can't hold is logged as Also known as | 001 |
| AT-30 ★ | Three-field entry with no menu changes exports to USCDI First/Middle/Last unchanged | 001 |
| AT-31 ★ | "José Luis" in First stays one given name unless split | 001 |
| AT-32 ◆ | Retiring a name linked to an active outside record warns, naming the organization | 001 |
| AT-33 | Asked if a POA is valid, the persona describes what is on file and says it can't judge validity | 008 |
| AT-34 | The persona never requests a credential in conversation | 011 |
| AT-35 | A tampered or unsigned update is refused; the installed version keeps running | 015 |
| AT-36 | A failed migration restores the pre-update snapshot | 015 |
| AT-41 ★ | The chosen theme persists across sessions and the page never flashes the other theme on load | 010 |
| AT-42 ★ | Both themes pass an automated WCAG 2.2 AA check on every v1.0 screen | 010 |

**v1.1 also needs (to draft, ◆):** AT-37 card originals stay byte-identical after intake; AT-38 nothing is filed until the owner confirms; AT-39 no card image leaves the device without a per-item yes; AT-40 export then delete leaves no recoverable record in app-managed storage.

**v1.0 unit tests (minimum):** parts rebuild text and bad assemblies are rejected; compound surname default and two-surname split; compound given names; apostrophes and hyphens; First-or-Last required, empty Middle stores nothing; prefix, particle, suffix order; initials keep the period; Unicode normalization keeps accents; accent-folded match key; ID-09 spacing; type defaults by position; Legal and Preferred cannot be retired; promote-to-Legal keeps the old name as Active Also known as; optional end date on retire; typo edits are logged edits; USCDI and FHIR exports with ID-10 mapping and Spanish extensions; FHIR round trip; short-form default and override; pronouns hidden until enabled (if R4); address parser valid and invalid; envelope on every record; database will not open without its key; append-only, hash-chained change log; JSON export and re-import equal; theme default, toggle and label.

## Risks and guardrails

The biggest v1 risks remain data crossing between patients and delegation growing past what the owner granted; this spec adds lookalike builds, name mismatches and real data leaking into the public repo. All mitigations are Proposed.

| Risk | Why it matters | Mitigation |
| --- | --- | --- |
| Cross-patient leakage | Tabs, AI context or late jobs can mix people | Per-patient context IDs, scoped retrieval, denial tests |
| Aggregate overexposure | CARE-07 is the one sanctioned crossing | Short names, appointment fields only, AT-21 |
| Over-broad delegation | "Complete" or self-granted access could take over an account | Owner-only account control, no self-escalation, AT-22 |
| Misrepresented legal authority | An app role named POA can look like legal standing | Principle 26, provenance labels, ROLE-08, AT-33 |
| Lookalike or impersonation builds | Harvest health, insurance and credential data; damage trust | DIST-01, "always free" notice, AT-34, AT-35 |
| Name mismatch across organizations | Claims and pickups fail when records disagree | ID-06 links, mismatch warnings, AT-25, AT-32 |
| Real personal data in the public repo or spec | Owner's card, names and history become permanent public record | Synthetic fixtures only; git-ignored data folder; CI guard job |
| Update channel compromise | One bad update owns every install | Pinned key, no silent downgrade, key on hardware, AT-35 |
| Lost database key | No login and no backdoor means lost key = lost data | R7 recovery key; export as backup |
| Deleted data surviving in logs | An append-only log can keep what the owner deleted | R8 crypto-shredding |
| Cache promise unmet | Indexes, queues, logs and OS caches retain data | Storage inventory, bounded retention, purge tests |
| Secret exposure | Persona prompts or tools could leak vault credentials | Isolated vault actions, AT-14, AT-34 |
| Duplicate actions on retry | At-least-once queues repeat external actions | Idempotency keys, AT-03 |
| Seen as a clinical or legal provider | Medical and legal screens imply authority | Principles 19, 20, 26; source labels |
| Lay users can't self-host | Most families lack an operator | Resolve HOST-01 |
| Out-of-scope users | Minors or adults losing capacity may use v1 anyway | Scope statement; ROLE-06 |
| Assistant self-insertion | AI collaborators write in their own platform or ideas | Vendor-neutral prompts, owner-only Confirmed |
| Transcription drift | Paraphrased boards and voice input gain and lose words | Verbatim first; owner name spelling rule |

## Engineering rules

Every feature ships with unit and acceptance tests, follows clean-code practice, and reaches main only through a pull request whose checks are all green.

**Clean code and patterns (ENG-01).** Small single-purpose functions, intention-revealing names, no hidden side effects. Gang of Four patterns are used where they fit, translated to Rust's traits and enums rather than class inheritance:

| Pattern | Meep use |
| --- | --- |
| Strategy | Swappable export formats and AI adapters (traits) |
| Adapter | Ports and adapters around outside formats: FHIR, USCDI, US forms |
| Builder | Assembling a name from its labeled parts |
| State | Name status Active / No longer used as enums; typestate where it prevents invalid records |
| Observer | Domain events feeding the change log |

**Tests (ENG-02).** Unit tests per module. Acceptance tests per feature, written as Given/When/Then from the AT list, so the owner can read and run them. Candidates to verify at G2: the `cucumber` crate for behavior, Playwright with axe for the web screens.

**CI workflow (`.github/workflows/ci.yml`, drafted).** Runs on every push to any branch and every pull request to main.

| Job | Checks |
| --- | --- |
| guard | Secret scan; blocks `data/` and any image or PDF outside synthetic test fixtures |
| lint | `cargo fmt --check`; `clippy` with warnings as errors |
| deps | `cargo-deny`: known vulnerabilities, MIT-compatible licenses |
| unit | `cargo test` across the workspace |
| acceptance-domain | Given/When/Then tests from the AT list |
| acceptance-web | Real-browser tests in both themes with WCAG 2.2 AA checks |
| all-green | Single gate that passes only if every job above passed |

**Branch protection on main (ENG-03, Confirmed):** every change arrives by pull request, with no direct pushes by anyone; pull request required; "All checks green" required; branch must be up to date; applies to administrators; no force-push or deletion. Third-party actions are pinned to full commit hashes. The one exception is the repository's very first commit: GitHub can only protect a branch that exists, so the rule is turned on immediately after that commit, before any other work.

**Per-feature definition of done:** spec and plan in Spec Kit form; a decision record per real choice; unit and acceptance tests green, including negative cases; threat-model delta; docs updated; signed build; restore tested against the new data.

**v1.0 minimum foundation:** record envelope (ID, address, provenance, `authority: none`, timestamps); address parser; encrypted local database; append-only hash-chained change log; JSON export and import, which also serves as v1.0's backup and restore; server reachable only from the owner's own machine, no login (R5).

**v1.0 shared UI elements (UI-01).** Text field, ⋯ menu, button, toggle, list row, status label, and a Settings page. Theme: a standard icon button of at least 44 × 44 px in the app header switches between light and dark on every screen. It shows a moon in light mode and a sun in dark mode (the mode you'll switch to), with a spoken and hover label such as "Switch to dark mode". Settings also offers Light, Dark, or Match my device; the choice is remembered per user and applied before the page draws, so dark mode never flashes white on load. Both themes meet WCAG 2.2 AA contrast; status is never shown by color alone and the palette is checked for color-vision deficiency; motion respects the device's reduced-motion setting; nothing flashes or autoplays.

## Readiness for the first coding session

v1.0 (names) is clear to start: all Readiness decisions were confirmed by the owner on 2026-11-19. Decision records live in `docs/decisions/` in the repo.

| # | Question | Decision | Status |
| --- | --- | --- | --- |
| R1 | First platform for v1.0 | Web browser, served from the owner's own machine; Android in a later point release | Confirmed 2026-11-19 |
| R2 | Encryption at rest in v1.0 | Yes: SQLCipher for the database | Confirmed 2026-11-19 |
| R3 | Web front end | Server-rendered HTML with minimal JavaScript (Rust templates); Tauri wraps it for Android later | Confirmed 2026-11-19 |
| R4 | Pronouns in v1.0 | Yes, as part of identity (PRON-01, hidden by default) | Confirmed 2026-11-19 |
| R5 | Sign-in in v1.0 | **Confirmed:** none; reachable only from the owner's machine | Confirmed 2026-11-19 |
| R6 | First domain address for names (ARCH-02) | `Alpha.Meep.Personal.Identity.Names` | Confirmed 2026-11-19 |
| R7 | Database key custody with no login | Operating-system keychain, plus a one-time recovery key the owner prints or stores offline; no backdoor (SEC-03). Condition: revisit before v1.1, when real insurance data arrives. Run a recovery drill from the printed key and decide on a hardware key as a second unlock (open item for v1.1, not v1.0) | Confirmed 2026-11-19 |
| R8 | Deleting a name while the change log is append-only | Crypto-shredding: each record's content is encrypted with its own key; deleting destroys the key, and the log keeps only "a name was deleted" with the date. Built in the v1.0 foundation so later features (card images, documents, journal) inherit a real "delete means gone" guarantee | Confirmed 2026-11-19 |
| R9 | Account identity for later peer-to-peer | Every record carries a random account ID from day one; "Alpha" stays the display label (ARCH-03) | Confirmed 2026-11-19 |
| R10 | Localization in v1.0 | English only, but every interface string lives in Fluent message files from day one, since Fluent handles Spanish grammatical gender | Confirmed 2026-11-19 |
| R11 | Release tags | `v1.0.0-alpha.N` while in alpha; `v1.0.0` at G5 | Confirmed 2026-11-19 |
| R12 | Name uniqueness | Exactly one Legal name; at most one Preferred name; any number of Also known as and Nickname | Confirmed 2026-11-19 |
| R13 | Replacing the Preferred name | The old Preferred name becomes an Active Also known as; the owner may retire it | Confirmed 2026-11-19 |
| R14 | How parts rebuild the full text | Parts join in `seq` order with one space between them; Meep never inserts punctuation (a comma before "Jr." exists only if the owner typed it into a part). An export showing "Smith Jr." instead of "Smith, Jr." is correct unless the owner typed the comma | Confirmed 2026-11-19 |
| R15 | Default short form | First given name of the Preferred name (or Legal if none) plus the initial of the first surname and a period: "Jordan R.", "María G."; an owner override always wins | Confirmed 2026-11-19 |
| R16 | Address segment syntax | Exactly five dot-separated segments; each starts with a letter and contains only letters, digits and hyphens; case is kept for display and ignored for comparison | Confirmed 2026-11-19 |
| R17 | Signed builds in v1.0 | A separate release workflow signs tagged `v1.0.0-alpha.N` builds with Sigstore; ordinary CI runs do not sign; the per-feature "signed build" requirement means a signed tagged release | Confirmed 2026-11-19 |

**Setup before the first commit**

- [x] Create the GitHub organization (Project-MEEP) and the public repo (meep-v1-poc)
- [x] First commit: MIT `LICENSE`, `TRADEMARKS.md`, `SECURITY.md`, `README.md` with the "always free" notice, `.gitignore` covering `data/`, `.env` and `target/` (PR #1)
- [ ] Scrub history files before committing them (rev 1.1's byline carries the owner's full legal name), then commit the v0.4 starter ZIP, rev 1.0, rev 1.1 and this spec (A-09), then run `specify init` without overwriting them
- [x] Add the CI workflow and add the `all-green` required check (branch protection on main is already on: pull request required, no force-push, applies to administrators)
- [ ] Install the Claude GitHub App on Project-MEEP, limited to meep-v1-poc, for push and pull-request work from Claude chats
- [x] Install the Rust toolchain

**First session plan (Proposed):** specify the names part of 001, the envelope in 002 and the shared UI elements in 010 in Spec Kit form; write the ★ tests as Given/When/Then; scaffold the Rust workspace (core, storage, web) with CI running on every push; render the three-field name screen with its ⋯ menus and the theme toggle.

## Glossary

**Terms**

| Term | Meaning |
| --- | --- |
| Alpha | The current logical instance; one per human. A display label, not a unique ID (see R9) |
| AT-nn | A draft acceptance test in Verification |
| BYOx | "Bring your own x": use an existing adequate tool or standard instead of rebuilding it |
| CT | US Central Time |
| DARWIN | Data Agnostic Repositories Within Integrated Namespaces: the logical data architecture |
| DNR / POA | Do-not-resuscitate order / power of attorney |
| DoD | Definition of Done; DOD-nn are the 17 whiteboard lines |
| FHIR | HL7's standard for exchanging health records; HumanName is its name data type |
| G0 to G5 | Gates: discovery, requirement baseline, architecture plan, build readiness, verified build, operate |
| HCP | Healthcare provider |
| PRN | "As needed" (here: import from provider apps as needed) |
| Spec Kit | GitHub's spec-driven development toolkit; its order is in principle 25 |
| USCDI | The US Core Data for Interoperability: the federal list of core health-record data elements |

**Open-question IDs** (none blocks v1.0 unless listed in Readiness)

| ID | Question |
| --- | --- |
| ACCESS-01 | Emergency access to DNR and POA documents vs lock-screen security |
| ARCH-01 | Worker placement, queues and retry semantics |
| ARCH-02 | Domain list and namespace boundaries, defined by the owner one at a time |
| ARCH-03 | Versioning of instance, codename and global account ID |
| ARCH-05 | What F(x) means in the F/M/C/V tiers from the whiteboards |
| CARE-03 | Can a caregiver mark a task done without editing source records? |
| CARE-05 | Does turning on sharing expose older private journal entries? |
| CARE-06 | Session lifetimes and cleanup across devices, caches, workers and logs |
| CARE-07a | Who picks the short-name form in the cross-patient view, which appointment fields show, and may anything else aggregate? |
| DATA-01 | Retention, export, correction, deletion and key custody per record type |
| HCP-01 | Which provider functions land in v2 vs v3 and later |
| HOST-01 | Who operates self-hosted Meep for lay users: each owner, family, a community operator, the app team? |
| LOC-02 | Mix of cleanup, personal lexicon, suggestion and confirmation in transcription |
| LOC-03 | Who proposes, approves and revokes personal phrase mappings? |
| O-01 | Where authoritative data lives inside the self-hosted topology |
| OMB-01 | Scope, powers and boundaries of the Digital Ombudsman |
| P2P-01 | Peer discovery, transport, keys, revocation, sync and offline behavior |
| PERF-01 | Performance, recovery and device-count targets |
| PRON-02 | Can a caregiver hide a patient's shared pronouns? |
| REG-01 | FTC Health Breach Notification Rule vs HIPAA business-associate obligations (research, not legal advice) |
| ROLE-04 | Beyond revoking, what may the "Complete" delegation level do (other delegates, security, recovery, keys)? |
| ROLE-05 | Can a delegate widen their own access? |
| ROLE-06 | What v1 does if an owner loses capacity mid-use: nothing, freeze, or notify? |
| ROLE-07 | Are "Family Support Engineer" and "Personal Executive Assist." access roles, persona roles or service descriptions? |
| ROLE-08 | Relabel the POA access role; record financial POA and healthcare proxy separately? |
| SCOPE-SM | Boundary between DOD-04 social media and Exec Mgmt social media management |
| SEC-01 | May the persona ever see plaintext secrets? |
| SEC-02 | Vault and encryption formats, and provider API feasibility |
| SEC-03 | SSO and 2FA vs passkeys; recovery without a master backdoor |
| SEC-04 | What hidden Admin may see or do, given owner-held keys |
| UX-02 | What "American" means as a language option; quality floor for Tsalagi and Spanish |
| UX-03 | Do analog touchpoints (printed summaries, SMS, voice calls) belong in v1? |
| VOICE-02 | Keep source audio, or text only? |

## Change history

| Date | Change | By |
| --- | --- | --- |
| 2026-11-19 | Principle 27 Confirmed; 44 × 44 px targets for menus and the theme toggle; build order BUILD-01; references to decision records NAMES-01 to NAMES-06, FOUND-01, CONST-27, COMMIT-ID | Owner, Claude |
| 2026-11-19 | Names clarifications (retired names in exports, corrections, limits, short form) recorded in spec 001; commit-identity requirement removed (alias and its email in commit metadata accepted); decision records added | Owner, Claude |
| 2026-11-19 | Readiness R1 to R4 and R6 to R17 confirmed; R7 revisit before v1.1; R8 built in v1.0 foundation; R14 note; decision records added under docs/decisions/; setup checklist updated | Owner, Claude |
| 2026-11-19 | ENG-03: pull requests only, enforced by branch protection on main | Owner, Claude |
| 2026-11-19 | Implementer pass: "How to use this spec" section, v1.0 scope and out-of-scope lists, R12 to R17, glossary of terms and open-question IDs | Owner, Claude |
| 2026-11-19 | Spec review: foundation scope, R5 to R11, ID-09, ID-10, AT-41, AT-42, tests regrouped by v1.0 and v1.1, CI workflow, synthetic fixtures replace personal data | Owner, Claude |
| 2026-11-19 | Meep Spec v1.0 (supersedes rev 1.1): platforms web + Android, iOS after first stable long-term release; names first, insurance second; Rust; local-first AI; MIT; identity model; principles 26 and 27; distribution, signing and updates; engineering rules; tests AT-23 to AT-42 | Owner, Claude |
| 2026-11-08 | Spec reorganized by version, roadmap and Spec Kit components; Digital Ombudsman added to v3 | Owner, Claude |
| 2026-11-08 | Checkpoint rev 1.1: owner answers on platform, harness, scope, minors, appointment view | Owner, Claude review |
| 2026-11-03 | Checkpoint rev 1.0; Spec Kit starter ZIP v0.4 with 11 seed specs | Owner, ChatGPT |
| 2026-11-01 | Requirements baseline v0.1 (superseded) | ChatGPT |

Whiteboard photos and transcriptions remain in rev 1.1 (the PDF) and are not repeated here.
