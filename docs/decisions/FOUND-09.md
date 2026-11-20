# FOUND-09: Multi-part changes are applied as one atomic batch

- **ID:** FOUND-09
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

Changes that must stay consistent across more than one record (for example R12, exactly one Legal name, when "Make this my legal name" retires the old one and promotes another) are applied as one atomic batch operation of the foundation's record store: all the record changes and their events land in one transaction or none do. No intermediate state is ever stored.

## Notes

This decides the "suggested later extension" left open in FOUND-08 (known limit: one record per call). Not yet reflected in contracts/ports.md, spec 002 or its tasks, or spec 001 T057; that alignment is a follow-up.
