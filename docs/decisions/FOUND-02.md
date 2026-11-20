# FOUND-02: Foundation clarifications (spec 002)

- **ID:** FOUND-02
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 clarification)

## Decision

1. The owner backup is a plain, open, documented file. Meep warns the owner when making it that anyone who has the file can read everything in it, and suggests keeping it somewhere safe.
2. v1.0 does not guard against other user accounts on the same computer. It assumes the computer is used by one person and says so in its documentation. **Amended 2026-11-20 (FOUND-11):** v1.0 blocks other local operating-system accounts at the port with the launch-token exchange; the same account and administrator or root access remain out of scope.
3. The recovery key is shown once and cannot be replaced after setup in v1.0. The owner backup is the protection against losing the normal key as well. The v1.1 revisit of key custody (R7) may reopen this.

Cross-reference (2026-11-20): item 2 is amended by FOUND-11; access layers are ACCESS-02; key wrapping is FOUND-12.
