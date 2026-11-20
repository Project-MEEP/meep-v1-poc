# Contract: Owner backup archive (format version 1.0)

FR-016 to FR-019, FR-036 to FR-044, FOUND-02, FOUND-03, FOUND-05, FOUND-06, PLAN-21, PLAN-22, PLAN-23.

## Layout

An owner backup is **one zip archive**. The file name is `meep-backup-<as-of UTC, compact>.zip` (for example `meep-backup-20261119T223000Z.zip`); a rescue backup uses `meep-rescue-backup-...zip`.

```text
meep-backup-20261119T223000Z.zip
├── READ-ME-FIRST.txt            plain text, plain language, complete on its own (FR-037)
├── manifest.json                format version, as-of time, features, counts, SHA-256 of every other entry
├── data/
│   ├── store.json               the store-level data: account ID, format info
│   └── <fragment_id>.json       one file per feature (for example names.json)
├── schema/
│   ├── manifest.schema.json     machine-readable schema of manifest.json
│   └── <fragment_id>.schema.json  machine-readable schema of that feature's data
└── unverified-history/          rescue backups only
    └── change-log.json          the old change log, clearly unverified (FR-031)
```

## manifest.json

| Member | Meaning |
|---|---|
| `format_version` | `"1.0"`: major.minor. A reader refuses a **greater major** version with the plain message "version-too-new" and never guesses; a greater minor with the same major is read, ignoring members it does not know (FR-039) |
| `as_of` | UTC time the backup's snapshot was taken, RFC 3339 (FR-041) |
| `kind` | `"owner-backup"` or `"rescue-backup"` |
| `account_id` | The account ID a restore adopts (R9, FR-032) |
| `features` | List of `{fragment_id, fragment_version, record_count}` |
| `files` | For every other entry: its path, size and SHA-256 |
| `rescue` | Only for a rescue backup: what could not be read, as a list of `{record_id?, address?, reason}`, and the warning text that must be shown on import (FR-030) |

## READ-ME-FIRST.txt

Rendered from `crates/meep-backup/templates/READ-ME-FIRST.txt` with placeholders for the format version, the as-of time, the feature sections and (for rescue backups) the unread-items list and the unverified-history explanation. It must say, in plain language and with no jargon:

1. What Meep is.
2. What each part of the archive holds.
3. How the fields map to the standard name formats (each feature supplies its own section through `BackupFragment::readme_section`).
4. How to restore the backup into Meep.
5. That the file is **not encrypted**: anyone who has it can read everything in it (FOUND-02).
6. The format version and the as-of moment, and that later changes are not in it (FOUND-06).
7. For a rescue backup: what could not be read and why the old history is kept only as an unverified archive (FOUND-03, FR-038).

The same full description is published at `docs/format/` (see PLAN-22); a check on every change confirms the template's fixed text equals the published copy (FR-040).

## Writing a backup (FR-043)

1. Take one consistent snapshot (a read transaction on the store) and record `as_of`.
2. Write every entry into `meep-backup-<...>.zip.partial` in the target folder.
3. Re-open the partial file and verify: the archive is readable, `manifest.json` parses, every file listed exists with the recorded SHA-256, every record counted is present, and `READ-ME-FIRST.txt` is present and non-empty.
4. Flush to disk and rename to the final name. A failure at any step deletes the partial file and reports "backup-failed"; no file with a final name exists unless step 3 passed.
5. Only after the rename, record `last_successful_backup_at` and append `backup_made` to the log.

Changes saved during steps 1 to 4 are written to the store normally and are not held up (FR-041); after the rename, if any change is newer than `as_of`, the owner is told which time the backup reflects (FR-042).

## Restoring (FR-017, FR-029 to FR-032)

1. Read `manifest.json`; refuse a newer major version (FR-039).
2. Check the store: restore into an empty store, or require the owner to choose "Replace everything" (FR-017).
3. Validate **everything first** without writing: each fragment through `BackupFragment::validate`, every address through the shared checker, every limit; collect every problem.
4. If any problem exists, refuse the whole restore and list every problem; nothing is written (FR-017).
5. Otherwise apply all fragments in one transaction, adopt the backup's `account_id`, create a new store key and a **new recovery key** shown once, and tell the owner plainly that the old recovery key does not open the new store (FR-032).
6. A rescue backup repeats its warning and unread-items list on import, restores only the readable records, loads `unverified-history/change-log.json` into `unverified_log_archive`, and starts a fresh chain whose first normal entry is `restored_from_rescue_backup` (FR-030, FR-031).
