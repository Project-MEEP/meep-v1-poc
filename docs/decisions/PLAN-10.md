# PLAN-10: Design patterns in Rust

- **ID:** PLAN-10
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Builder for name assembly (rebuild check at build); State for status as an enum with typestate only where it prevents an invalid record; Strategy and Adapter for exports; Observer for domain events feeding the change log (implements ENG-01).

## Alternatives considered

A general event-bus crate (more machinery than a handful of events needs).

## Notes

These are the patterns the spec lists, expressed with traits and enums.
