# PLAN-24: Address checker, identifiers and time

- **ID:** PLAN-24
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

A hand-written address parser in `meep-core` following R16; UUID v4 for store id, account ID and record ids (`uuid` 1.29.3, `rand` 0.10.4); UTC RFC 3339 timestamps with `time` 0.3.55 and an injectable clock.

## Alternatives considered

A regular-expression crate; sequential ids.

## Notes

The grammar is tiny; an injectable clock makes as-of and chain tests deterministic. Refines PLAN-11.
