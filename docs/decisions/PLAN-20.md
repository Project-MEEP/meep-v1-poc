# PLAN-20: Damaged-store handling

- **ID:** PLAN-20
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Never open the damaged file in place: copy `store.db` and `store.db-wal` byte-for-byte into a private temporary folder, hash the originals before and after, work on the copy, run key check, integrity checks, chain verification and a read-and-validate of every record; outcomes normal, read-only damaged, refused; rescue backup of fully valid records only; delete the copy afterwards.

## Alternatives considered

Open in place with an immutable flag (ignores log content); ask the owner to copy files by hand.

## Notes

The database engine can write when opening a log-mode file, so only a copy makes 'never modifies' provable (FR-029, SC-012).

Analysis fix (2026-11-19): the working copy is private (owner-only), holds only the encrypted store files, is deleted at the end of the session, and a leftover is deleted at the next start (FR-045, SC-017). A behavior spike proves that in-place open can change the file before the copy method is relied on.
