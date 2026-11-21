# PLAN-18: All-or-nothing writes and crash safety

- **ID:** PLAN-18
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Every record operation is one `BEGIN IMMEDIATE` transaction holding the record, its key and its log entry; WAL with `synchronous=FULL`; failure rolls back and returns `save-failed`; a feature-gated failpoint macro lets tests stop the process between steps.

## Alternatives considered

Writing record and log separately; the `fail` crate (last updated 2022).

## Notes

Gives FR-015 and is proven by kill tests.

Analysis fix (2026-11-19): failpoints can also return injected I/O errors (disk full, permission denied), and all crash and fault tests are one target, `crates/meep-storage/tests/crash/`.
