# Data Model: Identity and Names (v1.0)

Phase 1 output. Describes what is stored and the rules that keep it valid. It names no storage syntax; that arrives with implementation tasks. All example data is the synthetic fixtures from the spec.

## Entities

### Name

One name the owner uses. Stored as one record in the owner's encrypted database. Its content (text, parts, qualifiers) is encrypted with the record's own key (R8); its envelope fields are managed by the foundation (spec 002).

| Field | Meaning | Rule |
|---|---|---|
| `id` | Record identifier | Assigned by the envelope; stable |
| `account_id` | Random account ID on every record | R9; same for every record of this single owner |
| `address` | Where the record lives | Exactly five dot-separated segments, each starting with a letter, letters, digits and hyphens only, compared case-insensitively (R16). Names use `Alpha.Meep.Personal.Identity.Names` (R6) |
| `text` | The full name exactly as entered | Normalized to NFC, accents kept; leading and trailing spaces trimmed, runs of spaces collapsed to one, apostrophes and hyphens kept (ID-09) |
| `use` | Kind of name | One of Legal, Preferred, Also known as, Nickname (ID-01) |
| `status` | In use or not | Active or No longer used (ID-02) |
| `period` | Optional dates | Optional start and optional end; end is offered when status becomes No longer used (ID-01) |
| `short_form` | Optional owner-set short name | Display only; never used for matching; never in outbound exports (NAMES-04). Empty means "use the computed default" (R15) |
| `share_outbound` | Include in outbound exports | Boolean, default true; no control in v1.0 (NAMES-01) |
| `parts` | Ordered labeled parts | See Name part; must rebuild `text` exactly (ID-04, R14) |
| `match_key` | Accent-folded key | For matching only; never changes stored text (ID-04) |

### Name part

| Field | Meaning | Rule |
|---|---|---|
| `seq` | Position | Order used to rebuild text |
| `value` | The words | 1 to 128 user-perceived characters (NAMES-03) |
| `role` | What the part is | given, given_additional, family, particle, prefix, suffix (ID-04) |
| `qualifier` | Optional label | For family parts: father's, mother's, birth, spouse; plus flags `goes_by` and `initial` (ID-05, ID-07) |

Mapping of roles to the standards (ID-04): `given` is First; `given_additional` is Middle; family is Last; particle is part of the family name; prefix and suffix map to Prefix and Suffix.

### Pronoun setting

One record per owner.

| Field | Meaning | Rule |
|---|---|---|
| `enabled` | Pronouns on or off | Default off (PRON-01, R4) |
| `choice` | Which pronouns | she/her, he/him, they/them, use my name, or custom text |
| `visibility` | Who sees them | `only_me` (default when first enabled) or `in_exports`; the stored value may also hold `delegates` later but v1.0 never offers or writes it (PRON-01a) |

### Change-log entries (owned by spec 002; names supplies the payloads)

See contracts/domain-events.md. An in-place correction stores only the new wording; the log entry carries the date, which parts changed, and a keyed fingerprint of the prior value (NAMES-02, PLAN-05). A deletion stores only "a name was deleted" with the date (R8).

## Validation rules (each citing the spec)

1. **Rebuild**: joining the parts in `seq` order with one space and no inserted punctuation must equal `text` exactly; otherwise the save is refused as an integrity error and nothing is written (ID-04, R14, AT-28).
2. **Required parts**: at least one of First or Last; Middle is never required and is never stored empty (ID-05).
3. **Uniqueness of use**: exactly one Legal name; at most one Preferred name; any number of Also known as and Nickname (R12). Preferred falls back to Legal when blank (ID-01).
4. **Status**: Legal and Preferred are always Active and cannot be retired (ID-01, ID-02). Several names may be Active.
5. **Limits**: each box at most 128 user-perceived characters (minimum 1), the full-name box at most 384, at most 50 names in total; over a limit shows a message and saves nothing, never truncating (NAMES-03).
6. **Compound surnames**: several words in Last are one compound surname unless the owner splits it or chooses two last names (ID-07, ID-08).
7. **Conventions**: only US and Spanish two-surname forms in v1.0 (ID-07).
8. **Initials** keep their period (ID-05).
9. **Matching** uses `match_key` only and never merges or alters records (ID-04).

## State transitions

```text
                add name
                   |
                   v
   (new) ----> ACTIVE --- retire (owner) ---> NO LONGER USED
                     \                           |  (optional end date)
                      \--- promote to Legal ---> (old Legal becomes ACTIVE Also known as)

   Legal and Preferred: ACTIVE only; replacing Preferred turns the old
   Preferred into an ACTIVE Also known as (R13).
   Delete (owner): content and record key destroyed; log keeps only
   "a name was deleted" + date (R8).
```

Reactivating a retired name is not part of this feature: the spec has no requirement for it and ID-02 only defines the two statuses. It is not built until the owner decides (task T058); no screen, event or test exists for it in v1.0.

## Relationships

- One **owner** has many **Name** records (maximum 50) and one **Pronoun setting**.
- A **Name** has an ordered list of **Name parts**.
- A **Name** is referenced by **change-log entries** (foundation) and by exports; no other entity references it in v1.0 (outside-record links are v1.1, ID-06).

## Synthetic fixtures used by tests

| Name | Use | Status | Exercises |
|---|---|---|---|
| Jordan Lee Rivera Okafor (last = compound) | Legal | Active | Compound surname |
| Jordan Rivera | Preferred | Active | Fallback and short form ("Jordan R.") |
| Jordan L. Rivera (as on a card) | Also known as | Active | Initial, document rendering |
| Jordan Lee Pak (last tagged birth) | Also known as | No longer used | Retired birth name; "old" and Previous Name in exports |
| María José García López | Legal | Active | Compound given name, two surnames, accents, grapheme counting |

No real personal data and no email addresses appear anywhere in tests or fixtures.
