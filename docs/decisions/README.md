# Decision records

One short record per owner decision. Only the owner marks a decision Confirmed.
Every future confirmation lands as a reviewable pull request (ENG-03).

R5 (no sign-in in v1.0) was confirmed earlier and is recorded in the spec.

| ID | Decision | Status | Date |
| --- | --- | --- | --- |
| [R1](R1.md) | First platform for v1.0 | Confirmed | 2026-11-19 |
| [R2](R2.md) | Encryption at rest in v1.0 | Confirmed | 2026-11-19 |
| [R3](R3.md) | Web front end | Confirmed | 2026-11-19 |
| [R4](R4.md) | Pronouns in v1.0 | Confirmed | 2026-11-19 |
| [COMMIT-ID](COMMIT-ID.md) | Commit identity: alias and its email in commit metadata accepted; no noreply requirement | Confirmed | 2026-11-19 |
| [NAMES-01](NAMES-01.md) | Retired names included in outbound exports; sharing flag in the data model, control before v2 | Confirmed | 2026-11-19 |
| [NAMES-02](NAMES-02.md) | Corrections keep only the new wording; real changes add a new name | Confirmed | 2026-11-19 |
| [NAMES-03](NAMES-03.md) | Limits: 128 characters per box, 384 full-name box, 50 names | Confirmed | 2026-11-19 |
| [NAMES-04](NAMES-04.md) | Short form: per name, display only, not exported | Confirmed | 2026-11-19 |
| [PLAN-13](PLAN-13.md) | Foundation workspace layout (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-14](PLAN-14.md) | Encrypted store access (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-15](PLAN-15.md) | Key hierarchy (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-16](PLAN-16.md) | Crypto-shredding and proof of destruction (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-17](PLAN-17.md) | Change-log entry and hash chain (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-18](PLAN-18.md) | All-or-nothing writes and crash safety (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-19](PLAN-19.md) | One running copy (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-20](PLAN-20.md) | Damaged-store handling (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-21](PLAN-21.md) | Owner backup archive (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-22](PLAN-22.md) | Published format description and CI check (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-23](PLAN-23.md) | Restore rules (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-24](PLAN-24.md) | Address checker, identifiers and time (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-25](PLAN-25.md) | Loopback server and anti-forgery (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-26](PLAN-26.md) | Owner-facing messages and Fluent (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-27](PLAN-27.md) | Test strategy (foundation plan) | Proposed | 2026-11-19 |
| [PLAN-01](PLAN-01.md) | Workspace layout | Proposed | 2026-11-19 |
| [PLAN-02](PLAN-02.md) | Encrypted database access | Proposed | 2026-11-19 |
| [PLAN-03](PLAN-03.md) | Key custody | Proposed | 2026-11-19 |
| [PLAN-04](PLAN-04.md) | Per-record keys and crypto-shredding | Proposed | 2026-11-19 |
| [PLAN-05](PLAN-05.md) | Tamper-evidence for corrections | Proposed | 2026-11-19 |
| [PLAN-06](PLAN-06.md) | Text normalization and character counting | Proposed | 2026-11-19 |
| [PLAN-07](PLAN-07.md) | Web server and templates | Proposed | 2026-11-19 |
| [PLAN-08](PLAN-08.md) | Interface text | Proposed | 2026-11-19 |
| [PLAN-09](PLAN-09.md) | Export formats | Proposed | 2026-11-19 |
| [PLAN-10](PLAN-10.md) | Design patterns in Rust | Proposed | 2026-11-19 |
| [PLAN-11](PLAN-11.md) | Identifiers and addresses | Proposed | 2026-11-19 |
| [PLAN-12](PLAN-12.md) | Test tooling | Proposed | 2026-11-19 |
| [FOUND-09](FOUND-09.md) | Multi-part changes (for example exactly one Legal name) are one atomic batch operation | Confirmed | 2026-11-20 |
| [FOUND-10](FOUND-10.md) | Anchor the change-log head outside the store (open question) | Proposed | 2026-11-20 |
| [FOUND-08](FOUND-08.md) | Feature events travel with record writes (save, update, delete), validated by the foundation | Confirmed | 2026-11-20 |
| [FOUND-07](FOUND-07.md) | Test-only features (plaintext store, in-memory key store) never ship: CI check, start-up guard | Confirmed | 2026-11-19 |
| [FOUND-06](FOUND-06.md) | Backups while saving: as-of snapshot, nothing delayed, verified before naming, last-backup shown | Confirmed | 2026-11-19 |
| [FOUND-05](FOUND-05.md) | Owner backup: zip with read-me, data, schema; versioned; docs published and checked | Confirmed | 2026-11-19 |
| [FOUND-04](FOUND-04.md) | One running copy: OS-held lock, check after unexpected stop, refuse when unsure | Confirmed | 2026-11-19 |
| [FOUND-03](FOUND-03.md) | Damaged store: read-only open, rescue backup, fresh chain, new recovery key | Confirmed | 2026-11-19 |
| [FOUND-02](FOUND-02.md) | Foundation: plain open backup with a warning; one-person computer; recovery key not replaceable in v1.0 | Confirmed | 2026-11-19 |
| [VZ-02](VZ-02.md) | AI review of every pull request: Claude Code Action then Codex; CodeQL; grouped Dependabot | Confirmed | 2026-11-20 |
| [VZ-01](VZ-01.md) | Release gate "vulns at zero" for the first stable long-term release | Proposed | 2026-11-19 |
| [PLAN-28](PLAN-28.md) | Logging design: change log fields, encrypted diagnostic file, alias alphabet, typed log API | Proposed | 2026-11-20 |
| [LOG-01](LOG-01.md) | Logging: History log and Diagnostic log, content-free, FieldRef and record alias | Confirmed | 2026-11-20 |
| [SEC-05](SEC-05.md) | Input validation and injection: static SQL, autoescape, localhost hardening, name input rules, hostile-input tests | Confirmed | 2026-11-20 |
| [BUILD-01](BUILD-01.md) | v1.0 build order: spec 002, then 010, then names screens; no throwaway placeholders | Confirmed | 2026-11-19 |
| [NAMES-05](NAMES-05.md) | Checklist resolutions (CHK001 to CHK039) | Confirmed | 2026-11-19 |
| [NAMES-07](NAMES-07.md) | Keystone changes need explicit confirmation, a toast and a tray entry | Confirmed | 2026-11-20 |
| [UI-02](UI-02.md) | Toasts are transient notices | Confirmed | 2026-11-20 |
| [UI-03](UI-03.md) | Notifications tray: timestamped, dismissible, dismissals in the History log | Confirmed | 2026-11-20 |
| [NAMES-06](NAMES-06.md) | Reactivating a retired name | Confirmed | 2026-11-19 |
| [FOUND-01](FOUND-01.md) | Spec 002 ownership of key, storage, log-format and export-envelope primitives | Confirmed | 2026-11-19 |
| [CONST-27](CONST-27.md) | Principle 27 (no lock-in) confirmed | Confirmed | 2026-11-19 |
| [PRON-01a](PRON-01a.md) | Pronoun visibility in v1.0 (Only me / Included in exports; JSON backup always includes) | Confirmed | 2026-11-19 |
| [R6](R6.md) | First domain address for names (ARCH-02) | Confirmed | 2026-11-19 |
| [R7](R7.md) | Database key custody with no login | Confirmed | 2026-11-19 |
| [R8](R8.md) | Deleting a name while the change log is append-only | Confirmed | 2026-11-19 |
| [R9](R9.md) | Account identity for later peer-to-peer | Confirmed | 2026-11-19 |
| [R10](R10.md) | Localization in v1.0 | Confirmed | 2026-11-19 |
| [R11](R11.md) | Release tags | Confirmed | 2026-11-19 |
| [R12](R12.md) | Name uniqueness | Confirmed | 2026-11-19 |
| [R13](R13.md) | Replacing the Preferred name | Confirmed | 2026-11-19 |
| [R14](R14.md) | How parts rebuild the full text | Confirmed | 2026-11-19 |
| [R15](R15.md) | Default short form | Confirmed | 2026-11-19 |
| [R16](R16.md) | Address segment syntax | Confirmed | 2026-11-19 |
| [R17](R17.md) | Signed builds in v1.0 | Confirmed | 2026-11-19 |

## Open items

- **v2 entry condition (HCP-X):** a "Don't share this name outside Meep" control before any automatic outbound sharing (NAMES-01).
- **Delegation entry condition:** edits made by someone other than the owner keep the prior wording visible to the owner (NAMES-02).
- **R7, before v1.1:** recovery drill from the printed key; decide on a hardware key as a second unlock.
