# PLAN-26: Owner-facing messages and Fluent

- **ID:** PLAN-26
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Every foundation message is a Fluent ID in `crates/meep-web/locales/en/foundation.ftl` with the exact wording in contracts/owner-messages.md; typed errors map to IDs; a lint test fails if owner-visible text bypasses the catalog; the two owner-specified texts are asserted exactly.

## Alternatives considered

Hard-coded strings.

## Notes

R10 requires all text in message files. Refines PLAN-08.
