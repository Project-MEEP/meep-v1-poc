# Contract: What names needs from other specs

Interfaces only. Spec 002 (foundation) and spec 010 (shared UI) own their designs; names must not redesign them. If either spec cannot provide an item below, that is raised against that spec, not worked around here.

## From spec 002 (foundation)

| Need | Why names needs it |
|---|---|
| Record envelope: id, account ID, address, provenance, `authority: none`, timestamps | Every Name carries it (R9) |
| Address parser and checker | R6, R16; names supplies its own address and relies on the shared rules |
| Encrypted local store that will not open without its key | R2, R7 |
| Key custody: master key in the OS keychain plus one-time recovery key | R7 |
| Per-record key wrapping, derivation of purpose-specific subkeys, and destruction (including `secure_delete`, wrapped-key overwrite and log checkpoint) | R8, NAMES-02; **spec 002 owns and implements these primitives**; names only calls them through the `KeyCustody` and `RecordStore` ports and tests against fakes until they land (PLAN-04, PLAN-05) |
| Append-only, hash-chained change log that accepts the events in domain-events.md | NAMES-02, R8 |
| JSON export and import of all records | FR-019; names supplies its fragment |

## From spec 010 (shared UI elements and theme)

| Need | Why names needs it |
|---|---|
| Text field, ⋯ menu, button, toggle, list row, status label, Settings page | Composing the three screens |
| Theme toggle and Light, Dark, Match my device setting | Theme persistence (AT-41) is spec 010's |
| WCAG 2.2 AA in both themes (AT-42) | Names screens are part of the v1.0 screens checked |

## What names provides to others

- The Name entity, its rules and the exporters.
- Domain events (domain-events.md).
- Message IDs for the names screens in the shared Fluent catalog.
