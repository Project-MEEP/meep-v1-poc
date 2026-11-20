# FOUND-04: One running copy at a time (spec 002)

- **ID:** FOUND-04
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 clarification, question 2)

## Decision

1. "One copy at a time" is enforced by a lock the operating system holds for the running Meep and releases automatically when it stops, including after a crash or power loss. No marker file whose owner has to be guessed.
2. After an unexpected stop, the next start checks that the store is intact and the change log verifies before opening normally; if either fails, FOUND-03 applies (read-only, rescue backup).
3. If Meep cannot determine whether another copy holds the store, it does not ask the owner to confirm. It refuses with: "Meep may already be open. If it isn't, restart your computer and try again."
4. "Copy" means a running Meep program, not a browser tab or window. Several tabs on one running Meep are allowed; concurrent edits follow the NAMES-05 stale-save warning.
