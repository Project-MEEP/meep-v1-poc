# PLAN-18: All-or-nothing writes and crash safety

- **ID:** PLAN-18
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Every record operation is one `BEGIN IMMEDIATE` transaction holding the record, its key and its log entry; WAL with `synchronous=FULL`; failure rolls back and returns `save-failed`; a feature-gated failpoint macro lets tests stop the process between steps.

## Alternatives considered

Writing record and log separately; the `fail` crate (last updated 2022).

## Notes

Gives FR-015 and is proven by kill tests.
