| `E-ADDRESS-UNREGISTERED` | `AddressInvalid` (`Unregistered`) | The address is well formed but no feature registered it (FOUND-13) | `internal-error` |
| `E-ADDRESS-ALREADY-REGISTERED` | `AddressInvalid` (`AlreadyRegistered`) | A feature registered the same address twice (a programmer error) | `internal-error` |
# Contract: Diagnostic codes (foundation)

The registry of every diagnostic code the foundation can produce (ERR-01, NAMING.md section 2). A code is `E-<AREA>-<CONDITION>`, uppercase, with every multi-word condition hyphenated. It is the `Display` text of its `Error` variant (`Error::diagnostic_code()`), so it is also what developers and support see in the Diagnostic log and in the owner's exported debug text.

**Rules**

- Codes are permanent: once shipped, a code is never reused for something else and never changes meaning. A removed code stays in this table, marked `Removed`.
- A code never holds record content, a key, a name or a path.
- The owner never sees prose from `Display` or `Debug` (R10). The owner sees the message in the last column. Programmer errors share `internal-error`, which shows the code itself.
- A test (`crates/meep-core/tests/errors.rs`) fails if a code in the code is missing here or listed twice.

| Code | Variant | Meaning | Owner message ID |
|---|---|---|---|
| `E-ADDRESS-SEGMENT-COUNT` | `AddressInvalid` (`WrongSegmentCount`) | The address does not have exactly five parts | `address-invalid-segments` |
| `E-ADDRESS-SEGMENT-CHARS` | `AddressInvalid` (`BadSegment`) | A part does not start with a letter or holds a character other than letters, digits or hyphens | `address-invalid-segment` |
| `E-STORE-READ-ONLY` | `StoreReadOnly` | A write was attempted on a read-only store | `store-read-only` |
| `E-STORE-WRITE-FAILED` | `StoreFailed` | A save, delete or append failed and nothing was changed; carries a content-free `Cause` | `save-failed` |
| `E-RECORD-STALE` | `Stale` | The record changed since it was read | `record-changed` |
| `E-RECORD-NOT-FOUND` | `NotFound` | No such record | `record-not-found` |
| `E-RECORD-UNREADABLE` | `Unreadable` | One record cannot be decrypted or validated | `record-unreadable` |
| `E-RECORD-DELETED` | `RecordKeyDestroyed` | The record's key was destroyed (the record was deleted) | `record-deleted` |
| `E-KIND-EXISTS` | `KindExists` | The event kind is already registered (programmer error) | `internal-error` |
| `E-KIND-UNKNOWN` | `UnknownKind` | The event kind is not registered (programmer error) | `internal-error` |
| `E-DETAIL-REFUSED` | `DetailRefused` | A detail key is not allowed for this event kind; the `DetailKey` goes to the Diagnostic log only (programmer error) | `internal-error` |
| `E-HISTORY-UNVERIFIED` | `ChainBroken` | The history chain does not verify; carries the first broken entry number | `history-does-not-verify` |

Not an `Error` variant: `E-PANIC` is the fixed code the panic hook writes (ERR-01 item 1), followed by a repository path and line, or `<dep>` for a panic outside the repository.
