# LOG-01: Logging

- **ID:** LOG-01
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Refs:** OWASP Logging Cheat Sheet; ASVS 7.1.1, 7.1.2, 7.3.1; OWASP Top 10 2025 A09

## Decision

1. **Two logs.** The **History log** is owner-facing, append-only and hash-chained, with concise fixed-field entries (timestamp, event type, record alias, actor, outcome); it also serves as the transaction tail for debugging. The **Diagnostic log** is levelled, size-capped with rotation, local only, with no telemetry.
2. No log ever contains credentials, keys, the recovery key, tokens or record content. This is stricter than "no PII": logs are content-free.
3. Both logs are stored at the same protection level as the record store.
4. Data is identified only by FieldRef (a typed enum of schema paths, for example `name.part[i].family`), record alias and rule ID. Allowed value metadata: grapheme count, byte count, script class, NFC-changed flag.
5. **Record alias:** random, assigned at creation, stored in the encrypted store, never derived from content; a curated character set so aliases never look like names. A deleted record's alias cannot be resolved (R8).
6. The log API accepts only FieldRef, alias, rule ID, shape and enum types, never `&str` or `String` for data. Entries are structured fields, never free text (no log injection).
7. Notification-tray entries are dismissible; dismissals are recorded in the History log (UI-02, UI-03).
8. The owner-only debug view resolves aliases to values at display time; exported debug text stays alias-only.
9. **Canary tests:** fixtures seed unique canary strings; CI fails if any appears in any log output.

## Placement

Spec 002 (foundation) owns all of this and the record alias. Amended: FR-014, new FR-046 to FR-050, data-model (`records.alias`, `change_log` columns), contracts/change-log-entry.md, new contracts/logging.md, and a new task phase (T124 onward). The name validation rule IDs (NAMEVAL-nn) that the logs cite belong to spec 001 (SEC-05).

## Notes

The owner first sent this as "LOG-01" plus "LOG-02, extends LOG-01" and then folded both into this one record; there is no LOG-02.
