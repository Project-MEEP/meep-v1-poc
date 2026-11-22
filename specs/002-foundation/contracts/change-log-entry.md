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
| `details` | object | Only keys registered for that kind. Every value is a number, a time or date, or a member of a closed registered set, never free text, content or keys (LOG-01 clarification 2026-11-22); the value type of each key is in "Detail keys" below |
| `prev_hash` | string | Lowercase hex of the previous entry's hash; 64 zeros for the genesis entry |

`hash` = SHA-256 of the canonical body bytes, stored as lowercase hex. Because `prev_hash` is inside the body, changing, removing or reordering any earlier entry changes every later hash.

## Detail keys

Each registered detail key has exactly one value type. Code builds details from typed values (`DetailValue` in `crates/meep-core/src/detail.rs`), so free text cannot be represented. Stored text is canonical: it is accepted back only if re-encoding the parsed value gives the same text (no leading zeros, no time zone offset). A feature that registers a kind with a new key adds the key here with its type; it never uses an open string.

| Key | Value type | Stored text | Example |
|---|---|---|---|
| `format_version` | Whole number, 0 to 4,294,967,295 | Decimal digits | `1` |
| `address` | A valid five-segment address (R16; ASCII letters, digits and hyphens). Whether it must also belong to a registered address space is an open owner decision | The address as written | `Alpha.Meep.Personal.Notes.Sample` |
| `changed_parts` | One to 20 pairs of a part role and a position. Role is one of `given`, `given_additional`, `family`, `particle`, `prefix`, `suffix` (spec 001 data-model.md, ID-04); position is the part's `seq`, 0 to 65,535 | `role[position]` pairs joined by commas, no spaces | `family[2],given[1]` |
| `as_of` | A point in time | UTC RFC 3339 ending in `Z` | `2026-11-22T08:00:00Z` |
| `date` | A calendar date | `YYYY-MM-DD` | `2026-11-22` |
| `record_count` | Whole number, 0 to 4,294,967,295 | Decimal digits | `42` |
| `notice_id` | A member of a closed set, one per tray-notice kind, its code the notice's Fluent message ID. The foundation lists `backup-made`, `backup-failed`, `save-failed`; a feature adds the notices it raises | The code | `backup-failed` |

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
