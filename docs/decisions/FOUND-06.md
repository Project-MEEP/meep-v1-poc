# FOUND-06: Backups while changes are being saved (spec 002)

- **ID:** FOUND-06
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 clarification, question 4)

## Decision

1. Option B: everything carries on as normal during a backup; the backup shows the store as of the moment it started, and says so in the read-me and on screen.
2. Changes made during a backup are saved to the store normally (FR-015); a backup never delays or holds them.
3. When a backup finishes, if any change was made after its "as of" moment, Meep says "Changes you made after <time> aren't in this backup" with a "Make another backup" action.
4. A backup is written under a temporary name, fully verified (archive readable, every record and the read-me present), and only then given its final name. An interrupted or failed backup leaves no file that looks like a complete backup, and Meep says plainly that it failed.
5. Settings shows "Last successful backup: <date and time>" so the owner always knows what an emergency restore would lose.
6. Tests: a crash at any point during a backup leaves either a verified backup or no backup-named file; saves made during a backup survive a crash.
