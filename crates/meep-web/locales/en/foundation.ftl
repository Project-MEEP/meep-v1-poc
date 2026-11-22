# Foundation owner-facing messages (FR-035, FR-042, PLAN-26, R10).
# Exact wording comes from specs/002-foundation/contracts/owner-messages.md; change both together.

store-already-open = Meep may already be open. If it isn't, restart your computer and try again.
store-created-recovery-key = Your recovery key is shown below. It is shown only once. Print it or write it down and keep it somewhere safe. If you lose both this key and the one on your computer, your data cannot be recovered.
store-needs-recovery-key = Meep cannot find the key it keeps on this computer. Enter your recovery key to open your data.
store-lost-both-keys = Your data cannot be opened without a key, and there is no way around this. If you have an owner backup, you can restore it into a new store.
store-wrong-key = That key does not open this store. Check it and try again.
store-damaged-readonly = Some of your data could not be read. Meep has opened what it can, and you cannot make changes. You can make a rescue backup of what can be read.
store-unreadable = Meep cannot read your data file. It has not been changed. Restore your latest owner backup into a new store.
history-does-not-verify = Your history does not verify. The first entry that does not match is number { $seq }. The most likely causes are a software fault or the store being edited outside Meep.
backup-not-encrypted-warning = This backup is not encrypted. Anyone who has the file can read everything in it. Keep it somewhere safe.
backup-made = Backup saved. It shows your data as of { $time }.
backup-changes-after = Changes you made after { $time } aren't in this backup.
backup-make-another = Make another backup
backup-failed = The backup did not finish, so no backup file was made. Your data is unchanged. Try again.
last-successful-backup = Last successful backup: { $time }
last-successful-backup-none = You have not made a backup yet.
restore-needs-replace-everything = This store already holds data. Restoring will replace everything in it. Choose "Replace everything" to continue.
restore-refused = This backup cannot be restored, and nothing was changed. These problems were found: { $problems }
restore-version-too-new = This backup was made by a newer version of Meep and cannot be read by this one. Nothing was changed. Update Meep and try again.
restore-rescue-warning = This is a rescue backup from a damaged store. It does not include: { $unreadable }. The old history is kept only as an unverified archive.
restore-new-recovery-key = Your data was restored into a new store. The recovery key you had before does not open this new store. Your new recovery key is shown below, once. Keep it somewhere safe.
address-invalid-segments = That address must have exactly five parts separated by dots.
address-invalid-segment = Each part of the address must start with a letter and use only letters, numbers and hyphens.
save-failed = Your change could not be saved, and nothing was changed. Try again.
store-read-only = Your data is open in read-only mode, so changes cannot be saved.
request-refused = Meep refused a request that did not come from this computer's own Meep page.
record-changed = This item was changed since you opened it, so your change was not saved. Reload it and try again.
record-not-found = That item no longer exists.
record-unreadable = This item could not be read. The rest of your data is not affected. You can make a rescue backup of what can be read.
record-deleted = This item was deleted, and its contents can no longer be recovered.
internal-error = Something went wrong inside Meep, and nothing was changed. If this keeps happening, report this code: { $code }
