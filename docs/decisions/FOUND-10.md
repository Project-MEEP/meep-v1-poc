# FOUND-10: Anchor the change-log head outside the store

- **ID:** FOUND-10
- **Status:** Proposed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

**Open question, not decided:** anchor the change-log hash chain head outside the store so rollback or truncation of the whole file is detectable.

## Notes

The owner could not restate the original instruction exactly, and no earlier instruction was found in the repository or in this session's earlier record. Today the chain detects edits inside the log but not replacement of the whole store file with an older copy (spec 002 honest limits, FR-013). Not in v1.0 scope until the owner decides.
