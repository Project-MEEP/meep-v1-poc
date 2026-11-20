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
