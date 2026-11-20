# PLAN-04: Per-record keys and crypto-shredding

- **ID:** PLAN-04
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Each name record is encrypted with its own random key using XChaCha20-Poly1305 (`chacha20poly1305` 0.11.0); the key is stored wrapped under the master key; deleting a name destroys its wrapped key; in-memory keys use `zeroize` 1.9.2 (implements R8).

## Alternatives considered

AES-GCM via `aes-gcm` 0.11.1 (acceptable fallback); whole-database encryption alone (cannot destroy one record); overwriting content in place (unreliable on SQLite pages).

## Notes

Authenticated cipher with a long random nonce. Required tests: with `secure_delete` on, the key row overwritten and the log checkpointed, a deleted name's key and ciphertext cannot be found in the database file or log.
