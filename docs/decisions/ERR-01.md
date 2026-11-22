# ERR-01: Error and panic handling

- **ID:** ERR-01
- **Status:** Confirmed
- **Date:** 2026-11-21
- **Owner:** Owner (ORC directive 2026-11-21 20:50 CT, Part C)

## Decision

1. **Panics.**
   - Workspace clippy lints deny `unwrap_used`, `expect_used`, `panic`, `todo` and `unimplemented` in non-test code. Tests may use them through `clippy.toml` (`allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests`).
   - The release profile sets `panic = "abort"`.
   - A panic hook writes only a fixed boot-log code plus the crate or module location, never the panic payload or message.
   - Storage code never uses `PoisonError::into_inner`; a poisoned lock in storage is fatal. Clock and failpoint code may keep it, with a one-line comment at each saying why.
2. **`Display`** for every `Error` variant is its stable diagnostic code (the same string `diagnostic_code()` returns, for example `E-STORE-READ-ONLY`), never prose. A test asserts `to_string() == diagnostic_code()` for every variant.
3. **Content-free cause.** A `Copy` cause type: `None`, `Io(std::io::ErrorKind)`, `Sqlite(i32 extended result code)`. Variants that wrap a storage or I/O failure carry it. It goes to the diagnostic log only. The underlying error's message text is never stored or logged.
4. **`DetailRefused.key`** is a typed closed enum (`DetailKey`), not a `String`.
5. **Messages.** Stale, NotFound, Unreadable and RecordKeyDestroyed have dedicated messages; the other programmer errors share one generic message plus the diagnostic code. The owner reviews the English wording in the PR.
6. **Web boundary** (recorded now, implemented with meep-web): handlers map `Error` to an HTTP status and a message ID, and never send `Display` or `Debug` text to the browser. Refusals stay 403 `request-refused`.
7. **Where errors show up** (handed to spec 010): validation inline per field, all failures at once, with rule IDs; a transient save or backup failure as a toast plus a tray entry; a store-level failure (wrong key, damaged, read-only) as a blocking screen. Severity becomes part of the message catalog.
8. **Retries.** No automatic write retries beyond SQLite `busy_timeout`. Retrying is the owner's choice.
9. **Release safety.** A CI check fails if a release build of any product crate enables `failpoints` or `testing`.

## Notes

- Items 1 to 4 and 9 are implemented on the foundation messages branch (#40). Item 4 uses `DetailKey`, a closed `Copy` enum in `meep-core` (owner decision 2026-11-22), not `FieldRef`: a refused detail names an event-detail key, not a name field. The code registry is `specs/002-foundation/contracts/diagnostic-codes.md`. Items 6 and 7 are implemented later (meep-web, spec 010).
- Amends nothing; extends PLAN-26 (messages) and LOG-01 (boot log).
