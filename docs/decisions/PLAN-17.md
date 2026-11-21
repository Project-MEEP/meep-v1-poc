# PLAN-17: Change-log entry and hash chain

- **ID:** PLAN-17
- **Status:** Confirmed (owner, 2026-11-20, as proposed in the plan for spec 002)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

Entry body in RFC 8785 canonical JSON (`serde_json_canonicalizer` 0.4.1); `hash` = SHA-256 of the body; `prev_hash` inside the body; genesis entry at seq 0; kinds and detail keys registered per feature; triggers refuse update and delete; verify returns the first broken seq.

## Alternatives considered

A keyed (HMAC) chain; a Merkle tree; ordering by timestamp.

## Notes

Unkeyed so a rescue reader can verify without the key; tamper-evident, not protection against the key holder.
