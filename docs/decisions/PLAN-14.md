# PLAN-14: Encrypted store access

- **ID:** PLAN-14
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

`rusqlite` 0.43.0 with `bundled-sqlcipher`, or `bundled-sqlcipher-vendored-openssl` when no system OpenSSL exists (the owner's R2 choice at product level); open with a raw 256-bit key derived from the store key, WAL mode, `synchronous=FULL`, `secure_delete=ON`.

## Alternatives considered

Plain SQLite with application-level encryption; a system SQLCipher through the `sqlcipher` feature; another encrypted database.

## Notes

Spike on this PC (2026-11-19): `bundled-sqlcipher` fails without `OPENSSL_DIR`; the vendored build fails because it needs a native Windows `perl` (`no-asm` is already used, so no NASM). Open: which route to use on Windows. Refines PLAN-02.
