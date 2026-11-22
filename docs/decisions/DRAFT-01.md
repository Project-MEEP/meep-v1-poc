# DRAFT-01: vNext unsaved changes become encrypted drafts

- **ID:** DRAFT-01
- **Status:** Confirmed (direction, vNext)
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** [Meep Access Review 2026-11-20 08:48 CT](../reviews/Meep-Review-Access_2026-11-20_0848-CT.md) (the owner's decisions of 09:46 CT)

## Decision

On lock or close-out, edits are saved as a draft in the user's encrypted store, never in browser storage. The next unlock shows a field-by-field difference with Resume or Discard, and flags it if the record changed since the draft. Resuming still goes through the NAMES-07 confirmations; validation happens on save, not on draft. Drafts are kept until resumed or discarded. The History log records draft saved, resumed and discarded by field reference and record alias only (LOG-01).

## Notes

Not v1.0. Needs a draft record kind in the foundation when specified.
