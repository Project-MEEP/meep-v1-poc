# PLAN-27: Test strategy

- **ID:** PLAN-27
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Unit tests for the master spec's foundation list; property tests (`proptest` 1.12.0) for chain verification and backup round trips; failpoint and child-process kill tests for every all-or-nothing claim; two-process lock tests; raw-byte destruction scans; before-and-after hash comparison for damaged-store opens; `cucumber` 0.23.0 acceptance features for the six stories.

## Alternatives considered

Hand-written acceptance tests only.

## Notes

Each '100% of tested cases' criterion needs a test that can fail. Refines PLAN-12.
