# Decision records

One short record per owner decision. Only the owner marks a decision Confirmed.
Every future confirmation lands as a reviewable pull request (ENG-03).

Naming rules for IDs, files and statuses: [STD-01](STD-01.md) (Confirmed) and [NAMING.md](../standards/NAMING.md).

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
| [PLAN-13](PLAN-13.md) | Foundation workspace layout (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-14](PLAN-14.md) | Encrypted store access (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-15](PLAN-15.md) | Key hierarchy (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-16](PLAN-16.md) | Crypto-shredding and proof of destruction (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-17](PLAN-17.md) | Change-log entry and hash chain (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-18](PLAN-18.md) | All-or-nothing writes and crash safety (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-19](PLAN-19.md) | One running copy (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-20](PLAN-20.md) | Damaged-store handling (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-21](PLAN-21.md) | Owner backup archive (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-22](PLAN-22.md) | Published format description and CI check (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-23](PLAN-23.md) | Restore rules (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-24](PLAN-24.md) | Address checker, identifiers and time (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-25](PLAN-25.md) | Loopback server and anti-forgery (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-26](PLAN-26.md) | Owner-facing messages and Fluent (foundation plan) | Confirmed | 2026-11-19 |
| [PLAN-27](PLAN-27.md) | Test strategy (foundation plan) | Confirmed | 2026-11-19 |
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
| [FOUND-10](FOUND-10.md) | Anchor the change-log head outside the store: recorded in owner backups and shown in History (option B) | Confirmed | 2026-11-20 |
| [FOUND-08](FOUND-08.md) | Feature events travel with record writes (save, update, delete), validated by the foundation | Confirmed | 2026-11-20 |
| [FOUND-11](FOUND-11.md) | Launch-token exchange for local access; amends FOUND-02 item 2 | Confirmed | 2026-11-20 |
| [ACCESS-02](ACCESS-02.md) | Access layers are separate; emergency access is never a backdoor | Confirmed | 2026-11-20 |
| [CONST-28](CONST-28.md) | Proposed principle 28: separate access layers | Proposed | 2026-11-20 |
| [FOUND-12](FOUND-12.md) | Key wrapping: one data key, one wrapped copy per unlock method | Confirmed | 2026-11-20 |
| [FOUND-14](FOUND-14.md) | Owner-backup encryption (v1.1) | Confirmed (vNext) | 2026-11-22 |
| [FOUND-13](FOUND-13.md) | Registered addresses | Confirmed | 2026-11-22 |
| [LOG-02](LOG-02.md) | Log content and erasure policy | Confirmed (D, E pending) | 2026-11-22 |
| [PLAN-29](PLAN-29.md) | Recovery key format: digits, groups, check digits, key ID | Confirmed | 2026-11-20 |
| [PLAN-30](PLAN-30.md) | Dependency license allow-list: Zlib; blessing and WTFPL for sqlcipher-src only | Confirmed | 2026-11-20 |
| [AUTH-01](AUTH-01.md) | vNext sign-in: SSO enrollment, passkey unlock, 2FA | Confirmed (vNext) | 2026-11-20 |
| [USER-01](USER-01.md) | vNext user model, close-out, persisted session | Confirmed (vNext) | 2026-11-20 |
| [LOCK-01](LOCK-01.md) | vNext app auto-lock and idle | Confirmed (vNext) | 2026-11-20 |
| [DRAFT-01](DRAFT-01.md) | vNext drafts on lock and close-out | Confirmed (vNext) | 2026-11-20 |
| [FOUND-07](FOUND-07.md) | Test-only features (plaintext store, in-memory key store) never ship: CI check, start-up guard | Confirmed | 2026-11-19 |
| [FOUND-06](FOUND-06.md) | Backups while saving: as-of snapshot, nothing delayed, verified before naming, last-backup shown | Confirmed | 2026-11-19 |
| [FOUND-05](FOUND-05.md) | Owner backup: zip with read-me, data, schema; versioned; docs published and checked | Confirmed | 2026-11-19 |
| [FOUND-04](FOUND-04.md) | One running copy: OS-held lock, check after unexpected stop, refuse when unsure | Confirmed | 2026-11-19 |
| [FOUND-03](FOUND-03.md) | Damaged store: read-only open, rescue backup, fresh chain, new recovery key | Confirmed | 2026-11-19 |
| [FOUND-02](FOUND-02.md) | Foundation: plain open backup with a warning; one-person computer; recovery key not replaceable in v1.0 | Confirmed | 2026-11-19 |
| [VZ-02](VZ-02.md) | AI review of every pull request: Claude Code Action then Codex; CodeQL; grouped Dependabot | Confirmed | 2026-11-20 |
| [VZ-01](VZ-01.md) | Release gate "vulns at zero" for the first stable long-term release | Proposed | 2026-11-19 |
| [PLAN-28](PLAN-28.md) | Logging design: change log fields, encrypted diagnostic file, alias alphabet, typed log API | Confirmed | 2026-11-20 |
| [LOG-01](LOG-01.md) | Logging: History log and Diagnostic log, content-free, FieldRef and record alias | Confirmed | 2026-11-20 |
| [SEC-05](SEC-05.md) | Input validation and injection: static SQL, autoescape, localhost hardening, name input rules, hostile-input tests | Confirmed | 2026-11-20 |
| [LANG-02](LANG-02.md) | UI language: Language setting from v1.0, English only until LTS; never affects names; English fallback; test-only pseudo-locale | Confirmed | 2026-11-21 |
| [VAL-01](VAL-01.md) | Shared field validation: server is the authority, all failures at once, input kept | Confirmed | 2026-11-21 |
| [STD-01](STD-01.md) | Meep naming standards (docs/standards/NAMING.md) | Confirmed | 2026-11-21 |
| [STD-02](STD-02.md) | Directive and conformance SOP (docs/standards/SOP-DIRECTIVES.md) | Confirmed | 2026-11-22 |
| [LANG-01](LANG-01.md) | Implementation language is Rust | Confirmed | not stated in the master spec |
| [ENG-03](ENG-03.md) | Pull requests only, enforced by branch protection on main | Confirmed | 2026-11-19 |
| [ARCH-02](ARCH-02.md) | Domain list and namespace boundaries (open question in the master spec) | Proposed | 2026-11-21 |
| [ERR-01](ERR-01.md) | Error and panic handling: no panics, Display is a diagnostic code, content-free cause | Confirmed | 2026-11-21 |
| [BUILD-01](BUILD-01.md) | v1.0 build order: spec 002, then 010, then names screens; no throwaway placeholders | Confirmed | 2026-11-19 |
| [NAMES-05](NAMES-05.md) | Checklist resolutions (CHK001 to CHK039) | Confirmed | 2026-11-19 |
| [NAMES-07](NAMES-07.md) | Keystone changes: plain or typed confirmation, a toast and a tray entry | Confirmed | 2026-11-20 |
| [UI-01](UI-01.md) | Light/dark toggle button with a sun/moon icon in the app header in v1.0, plus a theme setting | Confirmed | 2026-11-20 |
| [UI-02](UI-02.md) | Toasts are transient notices | Confirmed | 2026-11-20 |
| [UI-03](UI-03.md) | Notifications tray: timestamped, dismissible, dismissals in the History log | Confirmed | 2026-11-20 |
| [NAMES-06](NAMES-06.md) | Reactivating a retired name | Confirmed | 2026-11-19 |
| [FOUND-01](FOUND-01.md) | Spec 002 ownership of key, storage, log-format and export-envelope primitives | Confirmed | 2026-11-19 |
| [CONST-27](CONST-27.md) | Principle 27 (no lock-in) confirmed | Confirmed | 2026-11-19 |
| [PRON-01a](PRON-01a.md) | Pronoun visibility in v1.0 (Only me / Included in exports; JSON backup always includes) | Confirmed | 2026-11-19 |
| [R5](R5.md) | Sign-in in v1.0 | Confirmed | 2026-11-19 |
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

## Prefix registry (STD-01, from NAMING.md section 1)

New prefixes need owner approval and are added here in the same PR. `R` (R1 to R17) is a closed series.

| Prefix | Covers |
|---|---|
| ACCESS | Access layers: custody, local access, identity, authorization |
| AUTH | Sign-in and identity (vNext) |
| BUILD | Build order and release sequencing |
| COMMIT | Commit identity (existing file `COMMIT-ID.md` keeps its name) |
| CONST | Constitution principles |
| DRAFT | Unsaved-change drafts (vNext) |
| ENG | Engineering process rules |
| ERR | Error and panic handling |
| FOUND | Foundation (spec 002) |
| LANG | Language: implementation language (LANG-01) and UI language and localization (LANG-02+) |
| LOCK | App auto-lock (vNext) |
| LOG | Logging |
| NAMES | Names feature (spec 001) |
| PLAN | Implementation-plan choices |
| PRON | Pronouns |
| R | v1.0 readiness, closed series |
| SEC | Security controls |
| STD | Standards, including this one |
| UI | Shared interface behavior (spec 010) |
| USER | User model (vNext) |
| VAL | Field validation pattern |
| VZ | Vulnerability gate and AI review |
