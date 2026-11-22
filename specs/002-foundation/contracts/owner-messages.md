# Contract: Owner-facing messages (foundation)

Every message the foundation shows or returns, with its Fluent message ID (R10, PLAN-26). The wording is plain language, short sentences, no jargon, each error saying what to do next (spec 001 FR-034 principle, applied here). Text in quotation marks that the owner specified is **fixed and must not be reworded** (FOUND-04, FOUND-06). Variables are written `{name}`. All messages live in `crates/meep-web/locales/en/foundation.ftl`; only English ships in v1.0.

| Fluent ID | When | Text |
|---|---|---|
| `store-already-open` | Another running Meep holds the store, or Meep cannot tell (FR-024, FR-035, FOUND-04) | "Meep may already be open. If it isn't, restart your computer and try again." |
| `store-created-recovery-key` | First run (FR-002) | "Your recovery key is shown below. It is shown only once. Print it or write it down and keep it somewhere safe. If you lose both this key and the one on your computer, your data cannot be recovered." |
| `store-needs-recovery-key` | The normal key is not available (FR-004) | "Meep cannot find the key it keeps on this computer. Enter your recovery key to open your data." |
| `store-lost-both-keys` | Neither key works (FR-004) | "Your data cannot be opened without a key, and there is no way around this. If you have an owner backup, you can restore it into a new store." |
| `store-wrong-key` | Wrong key or no key (FR-003) | "That key does not open this store. Check it and try again." |
| `store-damaged-readonly` | Partly damaged store opened read-only (FR-028, FOUND-03) | "Some of your data could not be read. Meep has opened what it can, and you cannot make changes. You can make a rescue backup of what can be read." |
| `store-unreadable` | Nothing can be read (FR-028) | "Meep cannot read your data file. It has not been changed. Restore your latest owner backup into a new store." |
| `history-does-not-verify` | Chain check fails (FR-033) | "Your history does not verify. The first entry that does not match is number {seq}. The most likely causes are a software fault or the store being edited outside Meep." |
| `backup-not-encrypted-warning` | Making an owner backup (FR-023, FOUND-02) | "This backup is not encrypted. Anyone who has the file can read everything in it. Keep it somewhere safe." |
| `backup-made` | Backup verified and named (FR-043) | "Backup saved. It shows your data as of {time}." |
| `backup-changes-after` | Changes exist after the as-of time (FR-042, FOUND-06) | "Changes you made after {time} aren't in this backup." |
| `backup-make-another` | Action label next to the message above | "Make another backup" |
| `backup-failed` | A backup was interrupted or failed (FR-043) | "The backup did not finish, so no backup file was made. Your data is unchanged. Try again." |
| `last-successful-backup` | Shown by the Settings screen (FR-044) | "Last successful backup: {time}" |
| `last-successful-backup-none` | No backup yet | "You have not made a backup yet." |
| `restore-needs-replace-everything` | Restoring into a store that holds data (FR-017) | "This store already holds data. Restoring will replace everything in it. Choose \"Replace everything\" to continue." |
| `restore-refused` | Any problem found in a backup (FR-017) | "This backup cannot be restored, and nothing was changed. These problems were found: {problems}" |
| `restore-version-too-new` | Newer incompatible format version (FR-039) | "This backup was made by a newer version of Meep and cannot be read by this one. Nothing was changed. Update Meep and try again." |
| `restore-rescue-warning` | Importing a rescue backup (FR-030) | "This is a rescue backup from a damaged store. It does not include: {unreadable}. The old history is kept only as an unverified archive." |
| `restore-new-recovery-key` | After a restore into a new store (FR-032) | "Your data was restored into a new store. The recovery key you had before does not open this new store. Your new recovery key is shown below, once. Keep it somewhere safe." |
| `address-invalid-segments` | Address does not have five segments (FR-008) | "That address must have exactly five parts separated by dots." |
| `address-invalid-segment` | A segment breaks the rule | "Each part of the address must start with a letter and use only letters, numbers and hyphens." |
| `save-failed` | Any save, delete or append failed (FR-015) | "Your change could not be saved, and nothing was changed. Try again." |
| `store-read-only` | A write was attempted on a read-only store | "Your data is open in read-only mode, so changes cannot be saved." |
| `request-refused` | A request fails Host, Origin or token checks (FR-021) | "Meep refused a request that did not come from this computer's own Meep page." |
| `record-changed` | A save was refused because the record changed since it was read (FR-015, ports.md `Stale`) | "This item was changed in another window after you opened it, so your change wasn't saved. Copy anything you typed that you want to keep, then reload to see the latest version and try again." |
| `record-not-found` | The record does not exist (ports.md `NotFound`) | "That item no longer exists. It may have been deleted." |
| `record-unreadable` | One record cannot be decrypted or validated (FR-028, ports.md `Unreadable`) | "This item could not be read. The rest of your data is not affected. You can make a rescue backup of what can be read." |
| `record-deleted` | The record's key was destroyed (FR-010, ports.md `RecordKeyDestroyed`) | "This item was deleted, and Meep can no longer open it. If you made a backup before it was deleted, it is still in that backup." |
| `internal-error` | A programmer error reached the owner: `KindExists`, `UnknownKind` or `DetailRefused` (FR-025) | "Something went wrong inside Meep. If this keeps happening, report this code: {code}" |

The last five were added on the owner's instruction (2026-11-21): the four record messages, and one generic message with a diagnostic code (`E-KIND-EXISTS`, `E-KIND-UNKNOWN`, `E-DETAIL-REFUSED`; registry in diagnostic-codes.md) in place of wording for each programmer error. The wording of `record-changed`, `record-not-found`, `record-deleted` and `internal-error` was approved by the owner on 2026-11-22 and is exact (tests assert it). `record-unreadable` is still draft wording. `record-changed` is shown with the owner's typed input still on screen (VAL-01), so they can copy it before reloading.

## Rules

- The two fixed owner-specified texts are `store-already-open` and `backup-changes-after` (with `{time}`); tests assert them exactly.
- Messages never contain a key, a recovery key (except the one-time display messages, which show it in a separate element), record content, or an implication of an intruder (FR-033).
- `history-does-not-verify` and `store-damaged-readonly` are different messages: one is about history, the other about unreadable data (FR-033).
- A lint test fails the build if any foundation code path produces owner-visible text that is not one of these IDs (R10).
