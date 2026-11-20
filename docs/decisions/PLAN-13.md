# PLAN-13: Foundation workspace layout

- **ID:** PLAN-13
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Four crates: `meep-core` (pure domain and the three ports), `meep-storage` (encrypted store, keys, lock, damaged-store open, restore), `meep-backup` (archive writer and reader, read-me, schemas, verification), `meep-web` (server shell and message catalog; screens added later by specs 010 and 001). The server and anti-forgery parts of PLAN-07 move into the foundation.

## Alternatives considered

Three crates with backup inside storage; one crate.

## Notes

Keeps the domain testable without a database and keeps web types out of the crates an Android wrapper will reuse (R3). Refines PLAN-01.
