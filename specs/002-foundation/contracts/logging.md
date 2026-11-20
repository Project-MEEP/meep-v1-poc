# Contract: Logging

LOG-01, FR-046 to FR-050, PLAN-28. Two logs, both content-free.

## History log

The change log of contracts/change-log-entry.md. Fixed fields: `ts`, `kind`, `record_alias`, `actor`, `outcome`, registered `details`, and the chain links. It is the owner-facing record and the transaction tail used for debugging. Notification-tray dismissals are appended as kind `notice_dismissed` with details `notice_id` (an enum, never text) (UI-02, UI-03).

## Diagnostic log

Levelled (`error`, `warn`, `info`, `debug`), size-capped with rotation (oldest entries dropped first), kept on this computer only, never sent anywhere (FR-027). It is stored at the same protection level as the record store: encrypted under a subkey of the store key (PLAN-28). Before the store key is available (a start-up failure), diagnostics are held in memory for that run only and are not written.

## What an entry may contain

| Member | Type |
|---|---|
| `level` | enum |
| `event` | enum of registered diagnostic events |
| `field` | `FieldRef`, a closed enum of schema paths such as `name.part[i].family`, each feature adding its own variants |
| `record_alias` | alias type |
| `rule` | `RuleId`, a stable ID such as `NAMEVAL-03`, shared by the spec, the Fluent message, the log and the tests |
| `shape` | grapheme count, byte count, script class (enum), NFC-changed flag, and nothing else about a value |

Never: credentials, keys, the recovery key, tokens, record content, free text.

## Interface rules (FR-049)

- The only way to write a log entry is the foundation's logging functions; their parameters are the types above. There is no `&str` or `String` parameter for data, so content cannot be passed and an input cannot inject a log line.
- CI check `check-logging.sh` fails on `println!`, `eprintln!`, `dbg!` and direct use of `tracing` or `log` macros outside the logging module.
- The owner-only debug view resolves an alias to its value at display time by reading the record; a deleted record resolves to nothing. Exported debug text contains aliases only.

## Canary tests (FR-050)

Test fixtures seed unique canary strings into names, addresses and credentials; the test run scans the History log, the Diagnostic log, panic output and standard output and error, and fails if any canary appears.
