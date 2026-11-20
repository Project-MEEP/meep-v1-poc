# Data Model: Foundation (v1.0)

Phase 1 output. What the store holds, how keys relate, and the rules that keep it valid. It names no SQL; table shapes are intentions for the implementation tasks. All example data is synthetic.

## Store layout on disk

| Item | Purpose |
|---|---|
| `store.db` | The SQLCipher-encrypted database (all tables below) |
| `store.db-wal` | The database's write-ahead log, created and removed by the database engine |
| `store.lock` | A lock file the running Meep holds with an operating-system lock (PLAN-19); contains nothing the owner needs |
| `backups/` | Default folder for owner backups, with `.partial` files only while one is being written (PLAN-21) |
| `damaged-work/` | Private owner-only working folder used only while a damaged store is examined; empty otherwise, and any leftover is deleted at the next start (FR-045, PLAN-20) |

The data key (called the store key elsewhere in this document, FOUND-12) is never stored in the clear: the keychain slot holds a key that unwraps it, and each unlock method has a wrapped copy in `key_slots`. The recovery key is never stored anywhere (FOUND-02).

## Tables

### `store_header` (one row)

| Field | Meaning | Rule |
|---|---|---|
| `store_id` | Random identifier of this store | UUID v4; names the keychain entry and salts the recovery wrap (PLAN-15) |
| `account_id` | The owner's random account ID | UUID v4, same on every record (R9, FR-007); a restore adopts the backup's value (FR-032) |
| `format_version` | Version of the store's own layout | Integer; a newer value than the running Meep understands is refused (not guessed) |
| `created_at` | When the store was created | UTC, RFC 3339 |
| `last_successful_backup_at` | Time of the last verified backup | Written only after a backup is verified and renamed (FR-044); null until the first |

### `key_slots` (one row per unlock method, FOUND-12)

| Field | Meaning | Rule |
|---|---|---|
| `slot_id` | Identifier of the slot | Integer, unique |
| `method` | The unlock method | Enum: `keychain`, `recovery` in v1.0; `passkey`, `hardware` later (AUTH-01) |
| `wrapped_data_key`, `wrap_nonce` | The data key encrypted under that method's key | XChaCha20-Poly1305, random nonce never reused; the only copy of the data key in the file (R7, FR-004, FR-055) |
| `key_id` | The short key ID (FR-056) | Digits, non-secret; set for `recovery` slots, null otherwise |
| `created_at` | When the slot was made | UTC, RFC 3339 |

The `keychain` slot's wrapping key is a random 256-bit key held in the operating-system keychain (FR-005); the `recovery` slot's wrapping key comes from the recovery key (PLAN-15, PLAN-29). Adding or replacing a slot rewraps the data key only (FOUND-12).

### `records` (one row per record)

| Field | Meaning | Rule |
|---|---|---|
| `id` | Record identifier | UUID v4 (envelope, FR-006) |
| `alias` | Short random name used by logs instead of the id or content | Random, assigned at creation, from a curated character set that cannot spell words or look like names (PLAN-28); unique among records; never derived from content; gone with the row, so a deleted record's alias resolves to nothing (FR-048, R8) |
| `address` | Where the record lives | Five dot-separated segments, validated by the shared checker (R16, FR-008); stored as written, compared case-insensitively |
| `provenance` | Where the record came from | Text from a small set: entered, imported, restored (PLAN-24) |
| `authority` | The statement Meep claims no authority | Always the value `none` (FR-006) |
| `created_at`, `updated_at` | Timestamps | UTC; informational, never used to order the chain (spec edge case) |
| `content_ct`, `content_nonce` | The record's content, encrypted with its own record key | XChaCha20-Poly1305; additional authenticated data binds `id` and `address` so ciphertext cannot be moved to another record |

The account ID is carried by every record through the header, not repeated as unprotected text in each row; the envelope returned to features includes it (FR-006, FR-007).

### `record_keys` (one row per record)

| Field | Meaning | Rule |
|---|---|---|
| `record_id` | The record the key belongs to | Primary key |
| `wrapped_key`, `wrap_nonce` | The 256-bit record key encrypted under the record-wrapping subkey of the store key | Overwritten with zeros and then deleted when the record is deleted (FR-009, FR-010, PLAN-16) |

### `change_log` (append-only)

| Field | Meaning | Rule |
|---|---|---|
| `seq` | Entry number | Starts at 0 with the genesis entry; strictly increasing, no gaps |
| `ts` | Time of the event | UTC, informational |
| `record_alias` | The record concerned, by alias | Null for store-level events; replaces the earlier `record_id` column so the log never carries the id (LOG-01, FR-048) |
| `actor` | Who or what did it | Enum: `owner`, `system` (LOG-01) |
| `outcome` | How it ended | Enum: `ok`, `refused`, `failed` (LOG-01) |
| `kind` | Kind of event | From a registry; each feature registers its kinds and the detail keys each may carry (FR-014, PLAN-17) |
| `details` | Feature-defined details | Only registered keys; never content or keys (FR-014, R8) |
| `prev_hash`, `hash` | The chain links | SHA-256 over the canonical entry body, which includes `prev_hash` (PLAN-17) |

Triggers inside the store refuse any update or delete of a row (FR-013, FR-014).

### `unverified_log_archive`

Holds the old change log carried inside a rescue backup. It is read-only (triggers refuse updates and deletes), never part of the live chain, and any place that shows it says it is unverified (FR-031, FOUND-03). Absent unless a rescue restore happened.

| Field | Meaning | Rule |
|---|---|---|
| `seq` | The entry number as found in the backup | Kept as found; gaps and duplicates are not corrected |
| `ts` | The time as found | Informational only |
| `record_id` | The record concerned, as found | May be null |
| `kind` | The kind of event, as found | Not checked against the registry |
| `details` | The details as found | Kept as text; never interpreted |
| `stored_prev_hash`, `stored_hash` | The links as found | Never recomputed or trusted |
| `source_as_of` | The as-of time of the rescue backup it came from | UTC |
| `imported_at` | When it was imported | UTC |

## Key hierarchy

```text
store_key (32 random bytes)  --- kept in the OS keychain, entry named by store_id
   |-- HKDF "meep/db-key/v1"          -> the SQLCipher database key
   |-- HKDF "meep/record-wrap/v1"     -> wraps each record key (AAD = record id)
   `-- (wrapped copy in the header under the recovery wrap key)

recovery_key (32 random bytes)  --- shown once, never stored (FOUND-02)
   `-- HKDF(salt = store_id, "meep/recovery-wrap/v1") -> recovery wrap key

record_key (32 random bytes, one per record)  --- wrapped in record_keys
   `-- HKDF "meep/purpose/<purpose>"  -> purpose subkey (for example an edit fingerprint, NAMES-02)
```

Destroying a record's wrapped key makes its content and every purpose subkey unrecoverable even if the store key later leaks (R8, FR-012).

## Validation rules (each citing the spec)

1. Every record carries the whole envelope; a record missing a field is refused (FR-006).
2. `account_id` is one random value for the store; the label "Alpha" is never an identifier (FR-007).
3. `address` must pass the shared checker: exactly five dot-separated segments, each starting with a letter and holding only letters, digits and hyphens; comparison ignores case (FR-008, R16).
4. Every write operation is one transaction: record, key and log entry land together or not at all (FR-015, PLAN-18).
5. The change log accepts only appends, never carries content or keys, and verifies by recomputing every hash from the genesis entry (FR-013, FR-014).
6. A record's content can be read only with its own key; deleting the key destroys the record (FR-009, FR-010).
7. A store opens only with the unwrapped data key, through the keychain slot or the recovery slot; the file alone reveals nothing (FR-001, FR-003, FR-004).
8. A restore targets an empty store unless the owner chooses "Replace everything"; any breached rule refuses the whole restore (FR-017).

## State transitions

```text
          (no store)
              | first run: create store key, recovery key (shown once), header, genesis entry
              v
        CLOSED  --- open with store key (silent) or recovery key -->  OPEN (read/write)
           ^                                                           |  |
           |   lock held by another / cannot tell                      |  | close
           +--- refuse (plain message) --------------------------------+  |
                                                                            v
        OPEN --- damaged or chain fails on start --> OPEN READ-ONLY (damaged copy) --> rescue backup
        OPEN --- unreadable --> REFUSED (point to latest owner backup)
        OPEN --- unexpected stop --> next start: lock free; integrity and chain check; open or follow damaged path
```

## Synthetic fixtures used by tests

| Fixture | Used for |
|---|---|
| Record A at `Alpha.Meep.Personal.Identity.Names`, content "synthetic sample A" | Envelope, save, read, delete, backup |
| Record B at `Alpha.Meep.Personal.Notes.Sample`, content "synthetic sample B" | Two-feature backup and restore |
| Addresses with four or six segments, a leading digit, an underscore, a space | Address checker refusals |
| Sentinel content strings and a fixed fake record key | Raw-byte destruction scans |
| A partly corrupted copy of a fixture store | Damaged-store, rescue backup and chain-break tests |
| A fixture owner backup with a newer major format version | Newer-version refusal |

No real personal data and no email addresses appear anywhere.
