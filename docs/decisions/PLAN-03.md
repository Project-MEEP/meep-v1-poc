# PLAN-03: Key custody

- **ID:** PLAN-03
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

`keyring` 4.2.0 for the master key in the OS keychain, plus a one-time recovery key the owner prints or stores offline (implements R7).

## Alternatives considered

Direct Windows credential APIs (platform-locked); a passphrase-derived key (needs a login-like prompt, against R5); a key file beside the database (defeats encryption).

## Notes

Cross-platform keychain access implementing R7. Open for scaffolding: per-platform backend features and the recovery-key format.
