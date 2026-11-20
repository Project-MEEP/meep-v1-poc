# PLAN-12: Test tooling

- **ID:** PLAN-12
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

`cucumber` 0.23.0 for Given/When/Then acceptance tests, `proptest` 1.12.0 for properties, plain `cargo test` for unit tests (implements ENG-02).

## Alternatives considered

Hand-written acceptance tests (less readable for the owner).

## Notes

The spec lists `cucumber` as a candidate to verify at G2; verified current.
