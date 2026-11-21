# PLAN-14: Encrypted store access

- **ID:** PLAN-14
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

`rusqlite` 0.43.0 with `bundled-sqlcipher-vendored-openssl` on Windows (verified), and `bundled-sqlcipher` where a system OpenSSL exists (the owner's R2 choice at product level); open with a raw 256-bit key derived from the store key, WAL mode, `synchronous=FULL`, `secure_delete=ON`.

## Alternatives considered

Plain SQLite with application-level encryption; a system SQLCipher through the `sqlcipher` feature; another encrypted database.

## Notes

Spike on this PC (2026-11-19): plain `bundled-sqlcipher` fails without `OPENSSL_DIR`. `bundled-sqlcipher-vendored-openssl` **passes** once Strawberry Perl is installed and the project is on a short path: SQLCipher 4.20.0 community, content not visible in the file, opening without the key fails; a cold debug build took 7 min 41 s. Windows build prerequisites: C++ Build Tools, Strawberry Perl on PATH, the vendored feature, and a **short build-folder path** (the `target` folder path must be 121 characters or fewer; Windows long paths were enabled and did not fix a deep path because the OpenSSL build tools are not long-path aware; fallback `CARGO_TARGET_DIR=C:\mt`). CI gets a `windows-latest` build and unit-test job in the `all-green` gate, with a build cache because OpenSSL is compiled from source. Open: macOS and Linux builds. Refines PLAN-02.

Analysis fix (2026-11-19): `plaintext-test-store` and `in-memory-key-store` are test-only cargo features, never default, never in release builds (FOUND-07). The store core is built in the foundational phase so later tasks can use it.
