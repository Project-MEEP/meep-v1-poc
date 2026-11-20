# PLAN-01: Workspace layout

- **ID:** PLAN-01
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Three crates: `meep-core` (pure domain), `meep-storage` (encryption, envelope, change log, key custody; shared with spec 002), `meep-web` (server-rendered UI).

## Alternatives considered

One crate (mixes web types into the domain, blocks Tauri reuse); five or more crates (more ceremony than one owner and one feature need).

## Notes

Matches the spec's proposed first-session scaffold; keeps the domain testable without a database or browser (R3).
