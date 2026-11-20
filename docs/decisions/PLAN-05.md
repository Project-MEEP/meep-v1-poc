# PLAN-05: Tamper-evidence for corrections

- **ID:** PLAN-05
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

The change log records the date, the parts changed, and an HMAC-SHA-256 of the prior value keyed from the record's own key through HKDF (`hmac` 0.13.0, `sha2` 0.11.1, `hkdf` 0.13.0). Only the new wording is stored (implements NAMES-02).

## Alternatives considered

Store the old text encrypted (rejected by NAMES-02); a plain hash (guessable for short names); no fingerprint (no tamper-evidence).

## Notes

Non-reversible, and meaningless once the record key is destroyed (R8).

Ownership: subkey derivation belongs to spec 002 (`derive_subkey`); names computes the HMAC fingerprint of the prior value from that subkey.
