# PLAN-06: Text normalization and character counting

- **ID:** PLAN-06
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

Normalize to NFC with `unicode-normalization` 0.1.26, then count user-perceived characters as extended grapheme clusters with `unicode-segmentation` 1.13.3; limits per NAMES-03; an accent-folded match key is stored separately.

## Alternatives considered

`chars().count()` or byte length (miscounts combined characters); `icu_segmenter` 2.4.0 (larger, locale-aware; revisit for later languages).

## Notes

Makes "María" count as 5 either way and keeps other scripts from hitting limits early. Open: the accent-folding algorithm and test cases.
