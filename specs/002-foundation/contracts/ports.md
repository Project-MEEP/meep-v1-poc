# Contract: Ports (what features may call)

The three capabilities from FOUND-01 and FR-025, described as behavior with the failure each can return. Names (spec 001) and the shared UI (spec 010) use only these; they never touch encryption, storage or the log directly. Signatures are shown as intent, not final Rust.

## RecordStore

| Operation | Behavior | Fails with |
|---|---|---|
| `save(address, content, provenance, events)` | Creates a record with a full envelope (id, account ID, address, provenance, `authority: none`, timestamps), a fresh record key, a `record_created` log entry and the calling feature's `events`, all in one transaction (FR-006, FR-015, FOUND-08) | `AddressInvalid{reason}`; `UnknownKind`; `DetailRefused{key}`; `StoreReadOnly`; `StoreFailed` (nothing changed) |
| `update(id, expected_updated_at, content, events)` | Replaces content in one transaction and appends a `record_updated` entry and the calling feature's `events`; refuses if the record changed since `expected_updated_at` so features can show the stale-save warning (NAMES-05) | `Stale`; `NotFound`; `UnknownKind`; `DetailRefused{key}`; `StoreReadOnly` |
| `read(id)` | Returns the envelope and decrypted content | `NotFound`; `Unreadable{id}` (the record cannot be decrypted or validated) |
| `list(address_prefix?)` | Returns envelopes, optionally for one feature's address | none beyond store state |
| `delete(id, events)` | One transaction: destroy the record key, delete the record, append `record_deleted` with only the date and the calling feature's `events` (FR-010, FR-011, FOUND-08) | `NotFound`; `UnknownKind`; `DetailRefused{key}`; `StoreReadOnly` |

**Feature events (FOUND-08).** The calling feature passes `events`, a list of `{kind, details}` (the record concerned is the one the call is about). The foundation validates **every** event before writing anything, with the same rules as the change log: the kind must be registered and the detail keys must be registered for that kind, and details that look like content or key material are refused (FR-014, R8). The foundation's own entry (`record_created`, `record_updated` or `record_deleted`) is appended first, then the feature's events in the order given, all in the same transaction as the record change. One invalid event fails the whole call and nothing changes (FR-015).

**Known limit in v1.0:** each call concerns one record. A feature operation that must change two records at once (for example names' "Make this my legal name") is two calls and is not atomic across both; the feature orders its calls so that a crash leaves a state it can detect and repair at the next start. A batch call that applies several record changes and their events together is the suggested later extension (recorded in FOUND-08).

Rules: addresses go through the shared checker on every save and update (FR-008); a failure leaves nothing changed (FR-015); a read-only store refuses every write operation with `StoreReadOnly` (FR-028).

## ChangeLog

| Operation | Behavior | Fails with |
|---|---|---|
| `register_kind(kind, allowed_detail_keys)` | A feature declares an event kind and the only detail keys it may carry | `KindExists` |
| `append(kind, record_id?, details)` (the foundation stores the record's alias, not the id, FR-048) | **Foundation-internal** (store-level kinds such as `store_created`, `backup_made`, `restored_from_rescue_backup`); features do not call it. Appends one entry at the end, linked to the previous; refuses detail keys that were not registered or that look like content or keys (FR-014) | `UnknownKind`; `DetailRefused{key}`; `StoreReadOnly` |
| `verify()` | Recomputes the chain from the genesis entry; returns the number of entries checked, or the first broken `seq` (FR-013) | `ChainBroken{first_seq}` |
| `read(range)` | Returns entries; there is no operation to change or remove an entry (FR-014) | none |

Features add entries only by passing `events` to `save`, `update` and `delete` (FOUND-08), so an entry and the change it describes are always in one transaction (PLAN-18). `append` is used by the foundation for store-level kinds (`store_created`, `restored_from_rescue_backup`, `backup_made`).

## KeyCustody

| Operation | Behavior | Fails with |
|---|---|---|
| `purpose_value(record_id, purpose, input)` | Returns a keyed, non-reversible value tied to the record's own key and the named purpose (for example the edit fingerprint, NAMES-02); meaningless after the record key is destroyed (FR-012) | `NotFound`; `RecordKeyDestroyed` |
| `destroy_record_key(record_id)` | Destroys the record's wrapped key; only `delete` and restore call it (FR-010) | `NotFound` |

Features never receive a record key, the store key or the recovery key. There is no operation that returns key material (FR-025, principle 15). Unlock methods and their wrapped copies of the data key (FOUND-12) are foundation-internal; no port exposes them, and v1.0 offers no way to add or replace one.

## BackupFragment (contributed by each feature)

| Member | Behavior |
|---|---|
| `fragment_id()` | Stable id, for example `names` |
| `export(snapshot)` | Returns the feature's data for the owner backup, read from one consistent snapshot (FR-041) |
| `schema()` | Returns the machine-readable schema of that data (FR-036) |
| `readme_section()` | Returns the feature's part of READ-ME-FIRST.txt, including how its fields map to standard formats where relevant (FR-037) |
| `validate(data)` | Checks imported data against the feature's own rules and returns every problem found (FR-017) |
| `apply(data)` | Writes validated data through `RecordStore` during a restore |

## Test doubles

`meep-core` provides in-memory fakes of the three ports behind a `testing` cargo feature so other crates and features can be tested before the real store lands (BUILD-01). The fakes follow the same rules above.
