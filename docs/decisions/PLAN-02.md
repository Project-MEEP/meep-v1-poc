# PLAN-02: Encrypted database access

- **ID:** PLAN-02
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

`rusqlite` 0.43.0 with `bundled-sqlcipher` (implements R2).

## Alternatives considered

`sqlx` (async, less direct SQLCipher use, no benefit here); plain SQLite plus application-level encryption (contradicts R2); an embedded key-value store.

## Notes

Mainstream synchronous SQLite crate with SQLCipher exposed as a feature. Risk: the bundled build may need the vendored-OpenSSL variant on Windows.
