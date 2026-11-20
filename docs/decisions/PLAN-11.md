# PLAN-11: Identifiers and addresses

- **ID:** PLAN-11
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Random UUID (version 4) as the account ID on every record (`uuid` 1.29.3, `rand` 0.10.4); a small hand-written parser for the five-segment address syntax (implements R6, R9, R16).

## Alternatives considered

A regular-expression crate for one rule (extra dependency); sequential IDs (leak ordering, break the peer-to-peer plan in R9).

## Notes

The grammar is tiny and fully specified.
