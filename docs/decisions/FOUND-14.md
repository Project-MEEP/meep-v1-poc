# FOUND-14: Owner-backup encryption (v1.1)
- **Status:** Confirmed (vNext) (owner, 2026-11-22)
- **Decision:** From v1.1 (insurance intake), owner backups offer encryption, pre-selected; the owner may untick it. An encrypted backup opens with the owner's existing recovery key, so no new secret is introduced. v1.0 backups stay plaintext as decided in FOUND-02.
- **Consequence:** losing the recovery key also makes encrypted backups unreadable; the backup screen says so in plain words.
- **Why:** v1.1 adds member IDs and insurance details; a plaintext backup of those is a ready-made medical identity theft kit.
