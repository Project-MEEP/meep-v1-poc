# USER-01: vNext user model: one person, one account, close-out, persisted session (D3, D11)

- **ID:** USER-01
- **Status:** Confirmed (direction, vNext)
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** [Meep Access Review 2026-11-20 08:48 CT](../reviews/Meep-Access-Review_2026-11-20_0848-CT.md) (the owner's decisions of 09:46 CT)

## Decision

- One person, one Meep account, one operating-system account. Several people behind one account only in caregiver mode (vNext). The v1 documentation states that anyone using the owner's operating-system account can see everything.
- One device may sign in to different instances one at a time; this needs a per-instance unlock, so it ships with AUTH-01. v1.0 is one instance per operating-system account.
- **Switching users is a close-out:** end the session, invalidate the cookie, zero in-memory keys, close the store, send `Clear-Site-Data` for cache, cookies and storage, delete working copies, and start the new user with an empty tray. The close-out message says that exports and downloads stay in the operating system.
- The sign-in list shows only users who opted in to "persisted session" (off by default; privacy over ease). It requires a device screen lock. Users are listed by a vanity username: display only, never identity, unique per device. Turning it off removes the user from the list and deletes their unlock material from the device.
- **Open (HOST-01):** where a non-persisted user's data lives, so a shared device keeps nothing of theirs.

## Notes

Not v1.0. v1.0 documentation (FR-022) already states that the same operating-system account can see everything.
