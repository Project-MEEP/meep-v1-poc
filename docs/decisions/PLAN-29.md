# PLAN-29: Recovery key format: digits, fixed groups, per-group check digit, key ID

- **ID:** PLAN-29
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** [Meep Access Review 2026-11-20 08:48 CT](../reviews/Meep-Access-Review_2026-11-20_0848-CT.md) (the owner's decisions of 09:46 CT)

## Decision

**Confirmed by the owner on 2026-11-20 as proposed below (length 91 printed digits, Damm check digit per group, 8-digit key ID).** Confirmed parts (owner, D5): a short key ID is printed with the key and shown at every recovery prompt; the key is digits only in fixed groups with a per-group checksum so a mistyped group is flagged. Proposed exact scheme:

- **Length:** the 256-bit key as a 78-digit decimal number (10^78 is larger than 2^256), written as 13 groups of 6 digits.
- **Checksum:** one check digit after each group, computed with the Damm algorithm (detects every single-digit error and every swap of two adjacent digits within a group). Printed as 13 groups of 7 digits, 91 digits in all.
- **Key ID:** 8 digits, from HKDF-SHA-256 of the key under its own label (separate from every wrap key), printed as two groups of 4 (for example `4821 7305`). It is also stored, non-secret, in the recovery slot, so every recovery prompt can list the IDs of the keys that belong to this store. FR-032 means an owner can hold several keys.
- **Why not shorter:** a shorter key weakens the wrap; 256 bits is kept from PLAN-15.
- **Alternatives considered:** base32 groups (PLAN-15, as proposed before); a word list (not digits only); one checksum for the whole key (flags an error without saying which group); a 128-bit key (39 digits, easier to type, weaker).
- A mistyped key that passes every check digit still fails to unwrap and is refused.

## Notes

Amends the encoding in PLAN-15 (base32 replaced). Nothing is open.
