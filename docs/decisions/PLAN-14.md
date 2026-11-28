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
- **Update 2026-11-28 (owner, D8-A, D9 item d):** the store is opened with a raw 256-bit key, `PRAGMA key = "x'<64 lowercase hex characters>'"`, so SQLCipher does no key derivation of its own (the key is already derived from the store key). The SQLCipher 4 settings are pinned with `PRAGMA cipher_compatibility = 4`, so a library default change cannot silently change the file format. `PRAGMA cipher_memory_security = ON` wipes SQLCipher's memory after use. `busy_timeout` is 5000 ms.
- **Implementation note 2026-11-28 (orchestrator, reversible):** `foreign_keys = ON` on every open (research.md); every setting that SQLite or SQLCipher can report back is read back after it is set, and a value that did not take refuses the open.
