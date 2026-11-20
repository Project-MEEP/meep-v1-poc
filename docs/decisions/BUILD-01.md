# BUILD-01: v1.0 build order

- **ID:** BUILD-01
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (PR review of feature 001)

## Decision

1. Build order for v1.0: spec 002 (foundation), then spec 010 (shared UI elements), then spec 001 screens on the real 010 elements.
2. Names domain and core tasks may proceed now against the port fakes.
3. No throwaway placeholders (APPROACH: nothing is disposed). Task T016 and the phase dependencies in tasks.md are updated: every screen and the web shell tasks are marked DEP-010.
4. The 44 by 44 pixel target for menus and the theme toggle is kept (NAMES-05).
