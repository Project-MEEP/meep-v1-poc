# PLAN-23: Restore rules

- **ID:** PLAN-23
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Validate everything first, collect every problem, refuse the whole restore if any; otherwise apply in one transaction into an empty store or after 'Replace everything'; adopt the backup's account ID; create a new store key and recovery key; for a rescue backup restore readable records only, load the unverified archive and start a fresh chain with `restored_from_rescue_backup`.

## Alternatives considered

Validate while writing and roll back; partial restore.

## Notes

A dry-run validation is what lets Meep list every problem (NAMES-05, FOUND-03).
