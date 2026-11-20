# NAMES-03: Name length and count limits

- **ID:** NAMES-03
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 001 clarification)

## Decision

1. Up to 128 characters per name box (US Web Design System name pattern: https://designsystem.digital.gov/patterns/create-a-user-profile/name/), counted as characters a person sees, minimum 1.
2. The full-name box allows up to 384 characters.
3. Up to 50 names in total.
4. Over a limit: a clear message; stored text is never truncated. Shorter limits in outside systems are handled at export (logged as an Also known as rendering).
