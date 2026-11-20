# ACCESS-02: Access layers are separate (D1) and emergency access is never a backdoor (D10)

- **ID:** ACCESS-02
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** Meep Access Review 2026-11-20 08:48 CT (the owner's decisions of 09:46 CT)

## Decision

Key custody, local access, identity and authorization are four separate mechanisms; no mechanism answers two layers. The owner always has a provider-free way in: the operating-system keychain or the recovery key. No identity provider, passkey service or sign-in can be the only way to the store key.

D10: emergency access (the open question ACCESS-01 in the master spec) must be an owner-granted unlock method or delegation, never a backdoor (SEC-03). Coercion is recorded as a named open question and is not decided.

A proposal to add this as constitution principle 28 is CONST-28 (Proposed).

## Notes

Spec 002 FR-054. ACCESS-01 stays an open question in the master spec; this record only constrains its answer.
