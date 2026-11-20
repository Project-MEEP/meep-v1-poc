# FOUND-03: Damaged store handling (spec 002)

- **ID:** FOUND-03
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 clarification, question 1)

## Decision

1. A damaged or tamper-failed store opens read-only with a clear warning; only an owner backup of what can be read is offered. A store that cannot be read at all is refused, pointing to the latest owner backup.
2. Meep never modifies, repairs or deletes the damaged store file. **The plan must specify how it is opened without altering it, including pending write-ahead-log content.**
3. The rescue backup contains only records that read and validate fully, so it always restores. It is labeled as from a damaged store and lists what could not be read; importing it repeats that warning.
4. The old change log is not restored as live history. It is kept as a separate, clearly unverified archive inside the rescue backup. The new store starts a fresh chain whose first entry records "restored from a rescue backup of a damaged store" with the date.
5. Restoring into a new store adopts the backup's account ID (R9) and issues a new recovery key; the owner is told plainly that the old recovery key does not open the new store.
6. Messages distinguish "some data could not be read" (damage) from "the history doesn't verify" (chain break). The chain-break message says the likely causes are a software fault or editing the store outside Meep and does not imply an intruder.
7. Tests: a rescue backup from a partly damaged store restores cleanly; the restored store verifies; the old recovery key fails on the new store.
