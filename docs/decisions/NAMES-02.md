# NAMES-02: Corrections to a name

- **ID:** NAMES-02
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 001 clarification)

## Decision

1. A correction keeps only the new wording. The history records "name edited", the date, which parts changed, and tamper-evidence of the prior value that is destroyed with the record's key (R8). The mechanism is chosen in planning.
2. Real name changes add a new name entry, so an in-place edit only ever discards a typo.
3. **Entry condition for the delegation feature:** when delegates can edit names, edits by someone other than the owner must keep the prior wording visible to the owner.
