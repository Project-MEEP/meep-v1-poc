# PLAN-09: Export formats

- **ID:** PLAN-09
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

A `NameExporter` trait with FHIR R4 HumanName, US health-data (USCDI) name and JSON backup implementations, built only from stored parts; outbound exports respect the sharing flag, omit short forms, and apply pronoun visibility; the JSON backup is always complete (implements ID-10, NAMES-01, NAMES-04, PRON-01a).

## Alternatives considered

One monolithic exporter (harder to add formats); building exports from `text` (breaks ID-04).

## Notes

FHIR R4 `use` codes verified. Open: whether R6 changes HumanName, the exact USCDI Previous Name layout, and the JSON field names.
