# Contract: Change-log entry and chain

FR-013, FR-014, FOUND-01, FOUND-03, PLAN-17, LOG-01. This log is the History log; the Diagnostic log is in contracts/logging.md.

## Entry body (what is hashed)

An entry's body is a JSON object with these members, serialized in a canonical form (RFC 8785 JSON Canonicalization Scheme) so the same entry always gives the same bytes on every platform:

| Member | Type | Meaning |
|---|---|---|
| `seq` | integer | 0 for the genesis entry, then 1, 2, 3 ... with no gaps |
| `ts` | string | UTC time, RFC 3339; informational only, never used for ordering |
| `record_alias` | string or null | The record concerned, by its random alias, never its id or content (LOG-01, FR-048) |
| `actor` | string | `owner` or `system` |
| `outcome` | string | `ok`, `refused` or `failed` |
| `kind` | string | A registered event kind |
| `details` | object | Only keys registered for that kind; values are ids, counts, enums or short fixed phrases, never content or keys |
| `prev_hash` | string | Lowercase hex of the previous entry's hash; 64 zeros for the genesis entry |

`hash` = SHA-256 of the canonical body bytes, stored as lowercase hex. Because `prev_hash` is inside the body, changing, removing or reordering any earlier entry changes every later hash.

## Registered kinds in v1.0 (foundation)

| Kind | When | Details allowed |
|---|---|---|
| `store_created` | Genesis entry when a store is created | `format_version` |
| `record_created` | After a save | `address` |
| `record_updated` | After an update | `address`, `changed_parts` (role and position only, supplied by the feature) |
| `record_deleted` | After a delete | none (only the date and the record alias, R8, FR-048) |
| `backup_made` | After a verified backup | `as_of`, `record_count` |
| `restored_from_rescue_backup` | First normal entry of a store restored from a rescue backup | `date` (FOUND-03) |
| `restored_from_backup` | First entry after a normal restore | `date` |
| `notice_dismissed` | When the owner dismisses a notification-tray entry (UI-03, FR-049) | `notice_id` (an enum) |

Features add their own kinds through `register_kind` and append entries by passing events to `save`, `update` and `delete` (FOUND-08); names adds its own (added, edited, retired, reactivated, promoted, replaced) and the log never holds a name's words, parts, short form or pronoun text (spec 001 domain-events contract).

## Verification

`verify()` reads entries in `seq` order and recomputes each hash from the canonical body and the previous hash. It returns the count checked, or `ChainBroken{first_seq}` naming the first entry whose stored hash or `prev_hash` does not match, or whose `seq` leaves a gap. The owner-facing message for a break is in contracts/owner-messages.md (FR-033).

## Append-only enforcement

- Triggers inside the database refuse any update or delete of a `change_log` row.
- The `ChangeLog` port has no operation to change or remove an entry.
- The chain is **tamper-evident, not tamper-proof**: someone who holds the store key could rewrite the whole log and recompute every hash. The spec asks for detectable alteration, removal or reordering (FR-013), which this gives; it does not claim protection against the key holder.

## After a rescue restore

A store restored from a rescue backup starts a fresh chain: its genesis entry is `store_created`, and its first normal entry is `restored_from_rescue_backup` with the date. The old log is stored read-only in `unverified_log_archive` and is never linked into the new chain (FR-031, FOUND-03).
