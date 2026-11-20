# Contract: Ports (what features may call)

The three capabilities from FOUND-01 and FR-025, described as behavior with the failure each can return. Names (spec 001) and the shared UI (spec 010) use only these; they never touch encryption, storage or the log directly. Signatures are shown as intent, not final Rust.

## RecordStore

| Operation | Behavior | Fails with |
|---|---|---|
| `save(address, content, provenance)` | Creates a record with a full envelope (id, account ID, address, provenance, `authority: none`, timestamps), a fresh record key, and a `record_created` log entry, all in one transaction (FR-006, FR-015) | `AddressInvalid{reason}`; `StoreReadOnly`; `StoreFailed` (nothing changed) |
| `update(id, expected_updated_at, content)` | Replaces content in one transaction and appends a log entry; refuses if the record changed since `expected_updated_at` so features can show the stale-save warning (NAMES-05) | `Stale`; `NotFound`; `StoreReadOnly` |
| `read(id)` | Returns the envelope and decrypted content | `NotFound`; `Unreadable{id}` (the record cannot be decrypted or validated) |
| `list(address_prefix?)` | Returns envelopes, optionally for one feature's address | none beyond store state |
| `delete(id)` | One transaction: destroy the record key, delete the record, append `record_deleted` with only the date (FR-010, FR-011) | `NotFound`; `StoreReadOnly` |

Rules: addresses go through the shared checker on every save and update (FR-008); a failure leaves nothing changed (FR-015); a read-only store refuses every write operation with `StoreReadOnly` (FR-028).

## ChangeLog

| Operation | Behavior | Fails with |
|---|---|---|
| `register_kind(kind, allowed_detail_keys)` | A feature declares an event kind and the only detail keys it may carry | `KindExists` |
| `append(kind, record_id?, details)` | Appends one entry at the end, linked to the previous; refuses detail keys that were not registered or that look like content or keys (FR-014) | `UnknownKind`; `DetailRefused{key}`; `StoreReadOnly` |
| `verify()` | Recomputes the chain from the genesis entry; returns the number of entries checked, or the first broken `seq` (FR-013) | `ChainBroken{first_seq}` |
| `read(range)` | Returns entries; there is no operation to change or remove an entry (FR-014) | none |

`append` normally happens inside the same transaction as the record change it describes; the store exposes it to features only through the record operations (PLAN-18), plus store-level kinds (`store_created`, `restored_from_rescue_backup`, `backup_made`).

## KeyCustody

| Operation | Behavior | Fails with |
|---|---|---|
| `purpose_value(record_id, purpose, input)` | Returns a keyed, non-reversible value tied to the record's own key and the named purpose (for example the edit fingerprint, NAMES-02); meaningless after the record key is destroyed (FR-012) | `NotFound`; `RecordKeyDestroyed` |
| `destroy_record_key(record_id)` | Destroys the record's wrapped key; only `delete` and restore call it (FR-010) | `NotFound` |

Features never receive a record key, the store key or the recovery key. There is no operation that returns key material (FR-025, principle 15).

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
