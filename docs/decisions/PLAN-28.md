# PLAN-28: Logging design (foundation)

- **ID:** PLAN-28
- **Status:** Confirmed (owner, 2026-11-20, as proposed)
- **Date:** 2026-11-20
- **Owner:** Owner (confirmation pending); implements LOG-01 (Confirmed)

## Decision

History log = the change log with fields `record_alias`, `actor`, `outcome`. Diagnostic log = a second encrypted database under an HKDF subkey of the store key, fixed columns, oldest rows dropped at a size cap (default 5 MiB). Record aliases come from a 32-symbol vowel-free alphabet in two groups of three or four, unique at save. Before the store key exists, diagnostics go to a size-capped plaintext boot log of fixed error codes and timestamps only, moved into the encrypted Diagnostic log after unlock and then cleared (LOG-01 item 3). A typed `log` module is the only way to log; CI scripts ban other logging calls.

## Alternatives considered

- A plain-text rotating file: rejected, it would not be at the store's protection level (LOG-01 item 3).
- Putting diagnostics in the main database: rejected, a size-capped churning table would add free-page and file-size noise to the store the destruction proof scans (FR-010); a separate file keeps that proof simple.
- Alias derived from the record id by hashing: rejected, LOG-01 item 5 forbids deriving it from anything; it is random and stored.
