# PLAN-22: Published format description and CI check

- **ID:** PLAN-22
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

The description lives in `docs/format/` (READ-ME-FIRST.txt with placeholders plus a text file per feature); the same files are the sources the backup writer renders; a test compares the rendered fixed text byte-for-byte with the published files and runs in the existing CI unit job.

## Alternatives considered

A separate website; generate docs at release time only.

## Notes

One source removes drift by construction; the test catches edits to one side.

Analysis fix (2026-11-19): `docs/format/owner-backup-format.md` is generated from the same sources by a generator binary and compared by the format-docs test, so no published copy can drift.
