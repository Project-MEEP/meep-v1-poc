# PLAN-21: Owner backup archive

- **ID:** PLAN-21
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

One zip (`zip` 9.8.0) with READ-ME-FIRST.txt, manifest.json (format version, as-of, counts, SHA-256 of every entry), data per fragment and a schema per fragment (`schemars` 1.2.3); written to a `.partial` name, verified by re-reading, flushed, renamed; snapshot is one read transaction.

## Alternatives considered

A folder of files; tar; hand-written schemas.

## Notes

Meets FOUND-05 and FOUND-06; a failure never leaves a file that looks complete.
