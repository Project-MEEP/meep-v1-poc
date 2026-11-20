# Contract: Export formats (names)

Three exporters, each built only from the stored labeled parts (ID-04, AT-29). Anything a target format cannot hold is logged as an Also known as rendering (AT-29). Items marked *to confirm at implementation* are not yet checked against the published standard.

## Terms (NAMES-05)

- **Outbound export**: any export made for someone other than the owner (the US health-data form and FHIR in v1.0). **Owner backup**: the owner's own full JSON export. Exports are snapshots: later changes never alter files already made.

## Common rules

| Rule | Source |
|---|---|
| Built from parts, never by re-parsing text | ID-04 |
| Outbound exports respect `share_outbound` (default share; no UI in v1.0) | NAMES-01 |
| Outbound exports never include `short_form` | NAMES-04 |
| Outbound exports include pronouns only if enabled and visibility is `in_exports` | PRON-01a |
| The owner backup always includes every name (retired included), short forms, sharing flags and pronoun settings | FR-019, FR-024a, NAMES-05 |
| A retired name's end date goes out where the format can hold it (FHIR period); where it cannot, it is omitted and logged | FR-018a, FR-017 |
| Deleted names never appear in an export made after the deletion; earlier files cannot be recalled | FR-033 |
| A new outbound format needs an explicit owner decision | FR-024a |
| Retired names are included in outbound exports | NAMES-01, ID-10 |

## 1. FHIR R4 HumanName (outbound)

Verified against the R4 name-use code system: `usual`, `official`, `temp`, `nickname`, `anonymous`, `old`.

| Meep | FHIR `use` |
|---|---|
| Legal | `official` |
| Preferred | `usual` |
| Nickname | `nickname` |
| Also known as, Active | `usual` |
| Also known as, No longer used | `old` |

| Meep part role | FHIR element |
|---|---|
| given | `given[0]` |
| given_additional | `given[1..]` |
| family (seq, optional qualifier) | `family` plus extensions for the qualifier |
| particle | family prefix |
| prefix, suffix | `prefix`, `suffix` |
| `text` | `text` |
| `period` | `period` |

A name's `use` and the retired state combine as ID-10 says. A later FHIR release (R6, listed stable Sept 2026) is **not verified**; re-check before targeting it.

## 2. US health-data (USCDI) name form (outbound)

- Three-field names with no menu changes export First, Middle, Last unchanged (AT-30).
- Prefix and suffix export in their own elements (spec Identity and names).
- Retired names export as **Previous Name** (owner-verified: USCDI Patient Demographics data class, NAMES-01). The exact element layout is *to confirm at implementation*; this site blocks automated checking.
- Values a target cannot hold are logged as an Also known as rendering (AT-29).

## 3. Owner backup (JSON; export and import)

Complete, open and documented (FR-019); import of an export reproduces equal records.

Names fragment, fields to be fixed at implementation (*to confirm*):

```text
names: [
  { id, account_id, address, text, use, status, period?, short_form?,
    share_outbound, parts: [ { seq, value, role, qualifier? } ] }
]
pronouns: { enabled, choice?, visibility }
```

Fixtures only in tests: no real data, no email addresses.
