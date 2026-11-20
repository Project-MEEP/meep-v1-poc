# PLAN-15: Key hierarchy

- **ID:** PLAN-15
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

A random 256-bit store key in the OS keychain (`keyring` 4.2.0); a random 256-bit recovery key shown once, encoded as base32 groups, never stored; HKDF-SHA-256 subkeys with distinct labels for the database key, the record-wrapping key and the recovery wrap key; per-record 256-bit keys wrapped with XChaCha20-Poly1305 (record id as associated data); purpose subkeys derived from each record key; no recovery-key replacement in v1.0; a restored store gets a new store key and recovery key.

## Alternatives considered

A passphrase-derived recovery key; a 24-word phrase encoding; a single key for everything.

## Notes

Both keys are full-strength random values so no password-stretching is needed. Open: recovery-key presentation, `keyring` backends per platform. Refines PLAN-03, PLAN-04, PLAN-05.
