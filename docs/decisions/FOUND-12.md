# FOUND-12: Key wrapping: one data key, one wrapped copy per unlock method (D4)

- **ID:** FOUND-12
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** Meep Access Review 2026-11-20 08:48 CT (the owner's decisions of 09:46 CT)

## Decision

The store has one random data key. Each unlock method (the operating-system keychain, the recovery key, later a passkey or hardware key) holds its own wrapped copy of it. Adding or replacing a method rewraps the data key only; no record and no database page is re-encrypted.

## Notes

Report on the plan before this decision: the plan already had a random store key and a recovery key that unwraps a stored copy of it, but the keychain held the key itself, not a wrapped copy, and there was no slot per method. Spec 002 now has a `key_slots` table (data-model), FR-055 and tasks T148 and T149. In this document the existing term "store key" means the data key. FOUND-02 item 3 is unchanged: v1.0 exposes no way to replace the recovery key; the mechanism only makes that possible later.
