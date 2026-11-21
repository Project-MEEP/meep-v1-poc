# PLAN-16: Crypto-shredding and proof of destruction

- **ID:** PLAN-16
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Delete is one transaction that zeroes and deletes the wrapped key row, deletes the record and appends `record_deleted`, then checkpoints and truncates the write-ahead log; `secure_delete=ON` zeroes freed content. Proof: raw-byte scan of an unencrypted-mode run for a planted sentinel and fake key, plus a check that the encrypted store has no key row.

## Alternatives considered

Scan only the encrypted file (proves nothing); trust the engine without a test.

## Notes

Encrypted pages hide content, so the proof needs an unencrypted run. Limit: copies held by the operating system or earlier backups are outside what Meep manages. Refines PLAN-04.
