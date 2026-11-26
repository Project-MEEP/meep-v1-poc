# FOUND-13: Registered addresses

- **ID:** FOUND-13
- **Status:** Confirmed
- **Date:** 2026-11-22
- **Owner:** Owner
- **Source:** `docs/directives/Meep-Directive-ORC_2026-11-22_1215-CT.md` (S2), from the owner's confirmation of the registered-address rule

## Decision

1. An address names one repository (`INSTANCE.NAME.Domain.Namespace.Repository`). Features register whole addresses, not segment vocabularies.
2. v1.0 fixes the instance and codename segments in code as `Alpha.Meep`. Names registers `Alpha.Meep.Personal.Identity.Names`.
3. Registry comparison ignores ASCII case (R16). The original case is kept for display.
4. `Address::parse` stays a syntax-only check. The registry check runs at the port boundary: every `save` and `update`, and every Address-type event detail passed in with them. Later, backup import too. `DetailValue::from_stored` stays pure and has no registry access.
5. A feature registers an address with `register_address(address)` on the same port as `register_kind` (`ChangeLog`), so one registry serves both checks. Registering the same address twice is an error.
6. An unregistered address is refused with `E-ADDRESS-UNREGISTERED` and nothing is written. The owner sees the generic `internal-error` message, because an unregistered address is a programmer error, not something the owner entered.

## Notes

- Why: a syntactically valid address could carry a surname in a segment, and addresses are written to the permanent change log. A closed registry keeps owner-chosen text out of addresses.
- Gap fill: the directive did not name the failure for a double registration. It uses the same family: `E-ADDRESS-ALREADY-REGISTERED`, owner message `internal-error`.
- Related: R6, R16, ARCH-02, LOG-01, FR-008.
