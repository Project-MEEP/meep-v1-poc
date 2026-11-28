# FOUND-10: Anchor the change-log head outside the store

- **ID:** FOUND-10
- **Status:** Confirmed (owner, 2026-11-22)
- **Decision:** The change-log head (seq and hash) is anchored outside the live store in two places in v1.0: every owner backup records it in its manifest, and the History view shows it. Restoring a backup compares the backup's recorded head with the live store's head and tells the owner how far back or ahead the restore goes before they confirm. This makes replacement of the whole store with an older copy visible at backup time, at restore time, and on screen.
- **Deferred:** a continuous anchor in the OS keychain (checked on every open) is deferred to the R7 key-custody review before v1.1, so the keychain design changes once.
- **Why:** detects accidental rollback (OS "previous versions", sync tools restoring an old file) with no new runtime failure modes; a same-account attacker can rewrite a keychain anchor too, so the keychain option mainly protects against accidents.
- **Date:** 2026-11-20
- **Owner:** Owner
