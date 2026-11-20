# PLAN-08: Interface text

- **ID:** PLAN-08
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Fluent message files through `fluent-templates` 0.15.1; English only in v1.0; a test fails the build if a template contains literal user-facing text (implements R10).

## Alternatives considered

Hard-coded strings (blocks Spanish and later languages); another i18n framework (R10 already confirmed Fluent).

## Notes

Flag: `fluent-bundle`, which it builds on, was last released May 2025; stable, but check periodically.
