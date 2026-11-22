# LOG-02: Log content and erasure policy

- **ID:** LOG-02
- **Status:** Confirmed, except items D and E, which are Pending owner
- **Date:** 2026-11-22
- **Owner:** Owner
- **Source:** `docs/directives/Meep-Directive-ORC_2026-11-22_1215-CT.md` (S7), from the review of the typed log details in PR #44
- **Related:** LOG-01, FOUND-02, FOUND-10, FR-010, FR-011, FR-027, FR-048, R8

## Decision

| Item | Status | Rule |
|---|---|---|
| B | Confirmed | Integrity checks and schema checks are separate. An unknown but well-formed code means newer data, never tampering (`StoredValueError::UnknownCode`; task T155; `E-HISTORY-UNVERIFIED` stays reserved for integrity failures). |
| A | Confirmed (documentation rule) | Deleting a record destroys its content and key, and the deletion entry holds only the fact and the date (FR-011). Earlier change-log entries for that record survive by design, because the hash chain cannot be rewritten. What survives: its alias, address, event kinds, changed part roles and positions, and dates. The log stores the alias, never the record id. Owner documents must list this. Part roles such as `particle` can hint at a naming tradition; the documents disclose that. Recorded in task T070. |
| G | Confirmed (design constraint) | v1.0 makes no outbound network requests (FR-027, enforced by the `deny.toml` bans). Every future outbound request (updates in spec 015, SSO in AUTH-01, sync, AI) must be designed against the FTC position that using a health app can itself be health information: minimal identifiers, documented, opt-in where it reveals use. Noted against REG-01 in the master spec; task T156 holds the future `deny.toml` follow-up. |
| P | Confirmed | Property tests over stored detail text (`crates/meep-core/tests/detail_props.rs`). |
| D | Pending owner | Chain-head anchoring is FOUND-10 (Proposed). Cross-reference only. |
| E | Pending owner | Owner backups are plaintext (FOUND-02). Open question added to the master spec: revisit owner-backup encryption before v1.1 insurance intake. |
| F | Met | ERR-01 covers diagnostic output (CWE-209, CWE-532). |
| H | Already planned | Deletion mechanics are planned (research D4; T022, T023, T028, T032, T065, T069). The behavior spike must prove that `secure_delete` zeroes freed content. |

## Notes

The honest limit behind item A is the same as FR-010's: a tamper-evident log cannot also forget. The design keeps content out of the log (LOG-01) so that what survives is metadata about a record, not the record.
