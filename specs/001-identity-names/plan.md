# Implementation Plan: Identity and Names (v1.0)

**Branch**: `001-identity-names` (spec) / `001-plan-names` (this plan) | **Date**: 2026-11-19 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-identity-names/spec.md`, the constitution v1.0.0, and the decision records in `docs/decisions/`.

## Summary

The owner can record names in three familiar boxes, keep several names with Active or No longer used status, and keep pronouns hidden until opted in. Every name is stored as labeled parts that must rebuild its text exactly (ID-04), exported from those parts to the US health-data name form (USCDI) and to FHIR R4 HumanName, and always included in the owner backup.

Technical approach, all drawn from Confirmed decisions (R1 to R17, NAMES-01 to NAMES-04, PRON-01a) and marked **Proposed** where this plan picks a specific library or design: a Rust workspace that uses the foundation's four crates (core, storage, backup, web; spec 002, BUILD-01) and adds only names code to them; an encrypted SQLCipher database reached through `rusqlite`; per-record keys wrapped under the store's one data key, which the foundation holds through its key slots and the operating-system keychain (FOUND-12), so that deleting a name destroys its key (R8); a keyed fingerprint of the prior value for tamper-evidence of corrections (NAMES-02); server-rendered HTML with minimal JavaScript, bound to the owner's own machine only (R3, R5); and Fluent message files for all text (R10). Names depends on the foundation (spec 002) and shared UI elements (spec 010); this plan defines only the names parts and the interfaces they need from those. Build order (BUILD-01): spec 002, then spec 010, then the names screens on the real 010 elements; the names domain and core can proceed now against port fakes.

## Technical Context

**Language/Version**: Rust, stable channel (LANG-01). Installed toolchain: rustc 1.100.0. Edition and minimum supported version are fixed when the workspace is scaffolded.

**Primary Dependencies** (all verified on crates.io 2026-11-19, see [research.md](research.md)): `rusqlite` 0.43.0 (`bundled-sqlcipher`), `keyring` 4.2.0, `unicode-segmentation` 1.13.3, `unicode-normalization` 0.1.26, `chacha20poly1305` 0.11.0, `hmac` 0.13.0, `sha2` 0.11.1, `hkdf` 0.13.0, `zeroize` 1.9.2, `rand` 0.10.4, `uuid` 1.29.3, `serde` 1.0.229 and `serde_json` 1.0.154, `thiserror` 2.0.21, `axum` 0.8.9 on `tokio` 1.54.1, `askama` 0.16.1, `fluent-templates` 0.15.1. Test-only: `cucumber` 0.23.0, `proptest` 1.12.0. All are MIT or Apache-2.0 (two carry the additional Unicode-3.0 terms). Every choice is **Proposed** until the owner confirms its decision record.

**Ownership note (FOUND-01, BUILD-01)**: storage, keys, encryption, the change log, backup and restore, and the loopback server are the foundation's (spec 002); names' own dependencies reduce to the text, template, Fluent and test crates, and the crates listed above for storage and keys are used through the foundation, not by names.

**Storage**: One encrypted local database file (SQLCipher through `rusqlite`) owned by the foundation (spec 002). Names adds tables and per-record encrypted content, described in [data-model.md](data-model.md).

**Testing**: `cargo test` for unit tests; `cucumber` Given/When/Then for the star-gated acceptance tests (AT-23, AT-26 to AT-31); `proptest` for round-trip and rebuild properties. Browser-level checks (AT-41, AT-42, WCAG 2.2 AA) belong to spec 010 and run against the names screens once they exist.

**Target Platform**: Web browser first, served from the owner's own machine, reachable only on the loopback address, no login (R1, R5). Android through Tauri wrapping comes in a later point release (R3), so the core and storage crates stay free of web-server types.

**Project Type**: Rust workspace with three crates (library core, library storage, web application).

**Performance Goals**: Single owner, tens of names at most (cap 50): every screen action completes with no perceptible delay, and an export of all names completes in well under one second. No throughput targets are needed.

**Constraints**: Offline only; no network calls of any kind (no cloud, no AI); loopback binding only; deleted names unrecoverable (R8); no real personal data in the repository (constitution); all interface text in message files (R10).

**Scale/Scope**: One owner, at most 50 names (NAMES-03), three screens (Names, Add or edit a name, Settings pronouns section), two outbound export formats plus JSON backup.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design: see the end of this section.*

| Principle | Gate | Status |
|---|---|---|
| 1, 2 (one human, owner owns data) | Single owner, no accounts, no caregiver code | Pass |
| 3, 4, 5 (roles, delegation) | No roles or delegation in 001; entry conditions recorded for later (NAMES-02) | Pass (out of scope) |
| 6, 7 (domain addressing, repositories) | Names stored under the confirmed first address `Alpha.Meep.Personal.Identity.Names` (R6, R16) | Pass |
| 10, 11, 12 (AI skin, vendor-neutral, self-hosted) | No AI feature; no vendor assumed | Pass |
| 15 (secrets isolated, Proposed) | Keys never logged or exposed to templates; not binding | Pass |
| 16 (encryption, audit) | SQLCipher at rest, per-record keys, hash-chained log from the foundation | Pass |
| 18 (free and open source, MIT, no mandatory paid service) | All dependencies MIT or Apache-2.0 compatible; checked in CI by `cargo-deny` (Unicode-3.0 must be allowed) | Pass |
| 19, 20 (no clinical claims) | Names only; exports labeled as informational | Pass |
| 22 (BYOx) | Uses existing crates and the standards named in the spec | Pass |
| 23 (specs, requirements, engineering stay separate) | Spec stays technology-agnostic; technology lives here | Pass |
| 24 (verbatim) | Not applicable | Pass |
| 25 (Spec Kit order) | constitution, specify, clarify done; plan now; checklist, tasks, analyze, implement, converge follow | Pass |
| 26 (warn, never adjudicate) | Names never blocked on legal or factual grounds; blocks only for integrity (rebuild must equal text; limits) and authorization (none in v1.0) | Pass |
| 27 (no lock-in, Confirmed) | Owner backup always complete (FR-019, FR-024a) | Pass |
| Engineering (ENG-01 to ENG-03, LANG-01, EGRESS-01) | Rust; clean code and Gang of Four patterns mapped in research; unit and acceptance tests; PRs only; no egress | Pass |
| No personal data in repo | Synthetic fixtures only | Pass |

No violations, so **Complexity Tracking** is empty.

**Re-check after Phase 1 design**: Pass. The design adds no network code, no secrets in templates or logs, and no real data; three crates is the smallest split that keeps the core reusable by Tauri (R3).

## Project Structure

### Documentation (this feature)

```text
specs/001-identity-names/
├── plan.md              # This file
├── research.md          # Phase 0: decisions, rationale, verification
├── data-model.md        # Phase 1: entities, rules, state
├── quickstart.md        # Phase 1: how to validate end to end
├── contracts/           # Phase 1: UI, export, event and foundation contracts
│   ├── ui-screens.md
│   ├── export-formats.md
│   ├── domain-events.md
│   └── foundation-interfaces.md
└── tasks.md             # Phase 2 (/speckit-tasks, not created here)
```

### Source Code (repository root)

```text
Cargo.toml                    # workspace
crates/
├── meep-core/                # domain, no I/O: Name, parts, NameBuilder, rules,
│   └── src/                  #   grapheme counting, address parser, exporters (Strategy/Adapter)
│       ├── name/             #   model, builder, status (State), validation
│       ├── export/           #   fhir.rs, uscdi.rs, json_backup.rs
│       ├── address.rs        #   R6 / R16 segment syntax
│       └── events.rs         #   domain events (Observer subject)
├── meep-storage/             # owned by the foundation (spec 002): encrypted database, envelope,
│   └── src/                  #   change log, key custody; names adds no storage code of its own
├── meep-backup/              # owned by the foundation (spec 002); names adds its backup fragment only
└── meep-web/                 # the foundation's server (launch-token exchange, FR-052); names adds
    ├── src/                  #   askama templates, routes and its Fluent catalog
    ├── templates/
    └── locales/en/           # names.ftl (names text, R10); the foundation's loader is reused
tests/
├── acceptance/               # cucumber features: AT-23, AT-26 to AT-31
│   └── features/
└── fixtures/                 # synthetic names only (spec fixtures table)
docs/decisions/               # one record per real choice (PLAN-nn)
```

**Structure Decision**: a workspace of `meep-core` (pure domain, testable without a database or browser), `meep-storage` (encryption, envelope, log; foundation-owned, with a names repository added), and `meep-web` (server-rendered UI, depends on the other two). It matches the spec's proposed first-session scaffold (core, storage, web) and keeps `meep-core` and `meep-storage` free of web types so a Tauri Android build can reuse them (R3). The workspace is created at implementation time, not by this plan.

## Complexity Tracking

No constitution violations. Nothing to justify.
