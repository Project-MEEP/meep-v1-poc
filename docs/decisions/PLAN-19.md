# PLAN-19: One running copy

- **ID:** PLAN-19
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

At start open `store.lock` and call the standard library's `File::try_lock`; hold the handle for the life of the process; on WouldBlock or any error refuse with `store-already-open`; never read a marker's owner. After an unexpected stop the OS has released the lock, so the next start checks the store and chain.

## Alternatives considered

`fs4` 1.1.0 or `fd-lock` 4.0.4; a PID marker file.

## Notes

Verified on this PC: refused while held, free after a hard kill. No crate needed. Limit: network drives and some cloud-synced folders may not honor locks.
