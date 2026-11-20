# FOUND-08: Feature events travel with record writes (spec 002)

- **ID:** FOUND-08
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner decision (ports question from the spec 001 alignment PR)

## Decision

`RecordStore` `save`, `update` and `delete` accept the calling feature's events and append them to the change log in the same transaction as the record change (FR-015). The foundation still validates every event: registered kind, registered detail keys, no content and no keys (FR-014). One invalid event fails the whole call and nothing changes. `ChangeLog::append` becomes foundation-internal. Updated: contracts/ports.md, foundation task T019 (and the fake, record-store and registry tasks), spec 001 T057 (unblocked).

## Known limit

Each call concerns one record. A feature operation that changes two records (names' "Make this my legal name" and "Replace the Preferred name") is two calls and is not atomic across both. Until the owner decides otherwise, the feature orders its calls so a crash leaves a state it can detect and repair at the next start. **Superseded by FOUND-09 (Confirmed 2026-11-20):** multi-part changes are applied as one atomic batch operation.

## Notes

The owner asked for this to be recorded as FOUND-07, which is already the "test-only features never ship" decision, so it is recorded as FOUND-08.
