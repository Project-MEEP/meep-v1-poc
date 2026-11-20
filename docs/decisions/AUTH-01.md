# AUTH-01: vNext sign-in: SSO enrollment, passkey unlock, 2FA (D6 to D9)

- **ID:** AUTH-01
- **Status:** Confirmed (direction, vNext)
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** [Meep Access Review 2026-11-20 08:48 CT](../reviews/Meep-Access-Review_2026-11-20_0848-CT.md) (the owner's decisions of 09:46 CT)

## Decision

Refines the whiteboard AUTH row (SSO login with authenticator 2FA) and does not replace it. Not v1.0; reverses R5 when it ships.

- **D6:** SSO federation (Microsoft, Google, Apple) for enrollment and identity. Daily unlock is delegated to the device's screen-lock policy (Windows Hello, PIN, biometrics) through a passkey bound at enrollment. SSO never gates the store key. Caregivers federate; the owner grants access.
- **D7:** a passkey plus the device check satisfies two-factor for now; an authenticator app is the fallback. Revisit when AUTH is specified.
- **D8 (Deferred):** a shared Project-MEEP public OAuth client versus per-install registration; Apple's requirements.
- **D9:** the owner may opt in to storing the recovery key with a provider they control; Project-MEEP never holds it.

## Notes

Build only after v1.0 (CLAUDE.md rule 1). D9 is an opt-in exception to "the recovery key is never stored" (FR-002, principle 15) and needs that wording reconciled when it is specified. ACCESS-02 (layers) and FOUND-12 (wrapped copies) are what make it possible without a backdoor.
