# FOUND-05: Owner backup contents and documentation (spec 002)

- **ID:** FOUND-05
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 clarification, question 3)

## Decision

1. An owner backup is one zip archive containing READ-ME-FIRST.txt (plain text, plain language), the data, and a machine-readable schema of the data format.
2. The read-me is complete on its own: what Meep is; what each part holds; how fields map to the standard name formats; how to restore; that the file is not encrypted (FOUND-02); and the format version.
3. A rescue backup's read-me also lists what could not be read and explains the unverified history archive (FOUND-03).
4. The format carries a version. A Meep that meets a newer incompatible version refuses with a plain message and does not guess.
5. The same full description is published in the project documentation; CI checks that the in-backup description and the published one match.
