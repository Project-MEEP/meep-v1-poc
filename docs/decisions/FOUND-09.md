# FOUND-09: Multi-part changes are applied as one atomic batch

- **ID:** FOUND-09
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

Changes that must stay consistent across more than one record (for example R12, exactly one Legal name, when "Make this my legal name" retires the old one and promotes another) are applied as one atomic batch operation of the foundation's record store: all the record changes and their events land in one transaction or none do. No intermediate state is ever stored.

## Notes

This decides the "suggested later extension" left open in FOUND-08 (known limit: one record per call). Reflected 2026-11-20 in contracts/ports.md (`apply`), spec 002 FR-058 and tasks T019, T020, T057, T062, and spec 001 T048, T053, T057. The exact signature of `apply` is a contract-level design choice, not an owner decision.
