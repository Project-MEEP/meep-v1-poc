# Contract: Domain events (names)

Names passes these events as the `events` argument of the foundation's `save`, `update` and `delete`, and, for a change to more than one record, with each change of `apply` (FOUND-08, FOUND-09); the foundation validates them and appends each to its append-only, hash-chained log in the same transaction as the record change. Event payloads carry no name text unless stated; the log keeps only what is listed here. The log stores a record's alias, never its id (FR-048, LOG-01), so where a row below says "record id" the registered detail is that record's alias, including the second record of NamePromoted and PreferredReplaced. The foundation also writes its own `record_created`, `record_updated` and `record_deleted` entries; these events add the names meaning on top of them (PLAN-10).

| Event | When | Payload (log keeps) | Notes |
|---|---|---|---|
| NameAdded | A name is saved the first time | record id, use, status, date | Content stays only in the encrypted record |
| NameEdited | A correction is saved in place | record id, date, which parts changed, keyed fingerprint of the prior value | Only the new wording is stored; the fingerprint is non-reversible and is meaningless once the record key is destroyed (NAMES-02, R8) |
| NameRetired | Status set to No longer used | record id, date, optional end date | |
| NameReactivated | The owner chooses "Mark as in use again" | record id, date | Status returns to Active and the end date is cleared (FR-028, NAMES-06) |
| NamePromoted | "Make this my legal name" | record id promoted, record id of the former Legal name, date | The former Legal name becomes an Active Also known as (ID-01) |
| PreferredReplaced | A new Preferred name replaces the old one | new id, old id, date | Old becomes an Active Also known as (R13) |
| NameDeleted | The owner deletes a name | record id, date only | Content and key destroyed (R8) |
| PronounSettingChanged | Pronouns turned on or off, choice or visibility changed | date, which setting changed | No pronoun text in the log |

## Rules

- The log never contains a name's words, parts, short form or pronoun text.
- A failed integrity check (parts do not rebuild text) emits no event and writes nothing (ID-04).
- When delegates can edit names (later), an edit by anyone other than the owner will keep the prior wording visible to the owner; that is an entry condition for the delegation feature (NAMES-02) and is not built here.
