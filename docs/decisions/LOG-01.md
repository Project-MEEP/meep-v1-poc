# LOG-01: Logging

- **ID:** LOG-01
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Refs:** OWASP Logging Cheat Sheet; ASVS 7.1.1, 7.1.2, 7.3.1; OWASP Top 10 2025 A09

## Decision

1. **Two logs.** The **History log** is owner-facing, append-only and hash-chained, with concise fixed-field entries (timestamp, event type, record alias, actor, outcome); it also serves as the transaction tail for debugging. The **Diagnostic log** is levelled, size-capped with rotation, local only, with no telemetry.
2. No log ever contains credentials, keys, the recovery key, tokens or record content. This is stricter than "no PII": logs are content-free.
3. Both logs are stored at the same protection level as the record store. **Boot log:** before the store is unlocked, diagnostics go to a plaintext boot log that holds only fixed error codes and timestamps (no paths, no usernames, no free text) and is size-capped. Its entries move into the encrypted Diagnostic log after unlock, and it is then cleared.
4. Data is identified only by FieldRef (a typed enum of schema paths, for example `name.part[i].family`), record alias and rule ID. Allowed value metadata: grapheme count, byte count, script class, NFC-changed flag.
5. **Record alias:** random, assigned at creation, stored in the encrypted store, never derived from content; a curated character set so aliases never look like names. A deleted record's alias cannot be resolved (R8).
6. The log API accepts only FieldRef, alias, rule ID, shape and enum types, never `&str` or `String` for data. Entries are structured fields, never free text (no log injection).
7. Notification-tray entries are dismissible; dismissals are recorded in the History log (UI-02, UI-03).
8. The owner-only debug view resolves aliases to values at display time; exported debug text stays alias-only.
9. **Canary tests:** fixtures seed unique canary strings; CI fails if any appears in any log output.

## Placement

Spec 002 (foundation) owns all of this and the record alias. Amended: FR-014, new FR-046 to FR-050, data-model (`records.alias`, `change_log` columns), contracts/change-log-entry.md, new contracts/logging.md, and a new task phase (T124 onward). The name validation rule IDs (NAMEVAL-nn) that the logs cite belong to spec 001 (SEC-05).

## Update 2026-11-20

Item 3 gained the boot log (owner). It replaces the earlier in-memory-only handling before the key exists (PLAN-28).

## Notes

The owner first sent this as "LOG-01" plus "LOG-02, extends LOG-01" and then folded both into this one record; there is no LOG-02.

**Clarification (owner, 2026-11-22):** every detail value in a log entry is a number, a time or date, or a member of a closed registered set. There are no open vocabularies. Details are built from typed values in code, so free text, a name or a key cannot be represented as a detail; text is checked only when a stored entry is read back or a backup is imported (spec 002 contracts/change-log-entry.md, `crates/meep-core/src/detail.rs`).
