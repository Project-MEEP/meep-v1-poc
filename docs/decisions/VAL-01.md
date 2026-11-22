# VAL-01: Shared field validation

- **ID:** VAL-01
- **Status:** Confirmed
- **Date:** 2026-11-21
- **Owner:** Owner (ORC directive 2026-11-21 20:50 CT, Part E)

## Decision

1. **Server-side validation is the authority.** Any browser-side check is a hint only.
2. **Every rule** has a stable rule ID and a Fluent message (R10, ERR-01).
3. **All failures come back at once:** an error summary at the top that links to each field, plus an inline message per field linked with `aria-describedby` (WCAG 3.3.1, 3.3.3).
4. **The owner's input is kept on error,** never cleared.
5. **Validation runs on save,** not on draft (access review D11). NFC normalization first, then the caps (SEC-05).
6. **Future field types** (dates, phone numbers, member IDs in v1.1) register their rules in this pattern. No feature invents its own error display.

## Notes

- Reconciliation with spec 001 (proposed in the directive, pending owner review): one name saves atomically, all of its boxes or none (R12, FOUND-09); independent items (another name, a setting) save independently; all failures in the form are shown at once. Spec 001 FR-001a, FR-001c and task T104 were reworded to match in the same PR.
- The error component itself belongs to spec 010; the rule IDs (`NAMEVAL-nn`) belong to spec 001 (FR-001d, SEC-05).
- Display of a failure never logs the typed text (LOG-01, spec 002 FR-049).
