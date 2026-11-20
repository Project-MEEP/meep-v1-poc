# NAMES-05: Checklist resolutions for spec 001

- **ID:** NAMES-05
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (feature 001 checklist review)

## Decision

Resolutions to the requirements-quality checklist (CHK001 to CHK039), written into spec 001:

**Privacy and export**
- CHK001: outbound export = any export made for someone other than the owner (US form, FHIR); owner backup = the owner's full JSON export; these two terms are used everywhere.
- CHK002: retired-name rules apply to every outbound format, current and future.
- CHK003: a retired name's end date goes out where the format can hold it (FHIR period); otherwise omitted and logged (FR-017).
- CHK004, CHK010: the owner backup includes everything (all names, short forms, sharing flags, pronoun settings); short forms never go outbound.
- CHK005: deleted names never appear in exports made after deletion; files made earlier cannot be recalled (stated plainly).
- CHK006: exports are snapshots.
- CHK007: "Included in exports" covers the two named outbound formats only; a new format needs an explicit owner decision.
- CHK008: no v1.0 path sets `share_outbound` to false; importing an owner backup preserves its value.
- CHK009: adding a "Delegates" visibility value requires no change to existing stored records (tested).
- CHK011: resolved by CONST-27 (principle 27 Confirmed).
- CHK012: both entry conditions (NAMES-01 control before v2; NAMES-02 edits by others visible) are copied into the spec.

**Data integrity**
- CHK013: order is NFC normalize, trim, collapse spaces, count.
- CHK014: "characters a person sees" = extended grapheme clusters per Unicode UAX #29.
- CHK015: no separate combined limit for the three boxes.
- CHK016: the whole save is refused; the message sits on the offending field; everything typed stays.
- CHK017: all non-deleted names count toward 50, retired included.
- CHK018: collapsing spaces is intentional; rebuild equality is checked against the normalized text.
- CHK019: the owner chooses "Fix a typo" or "Add a new name"; Meep never guesses.
- CHK020: "which parts changed" = the role and position of each changed part.
- CHK021: after delete, the wrapped key must be absent from the database file, the write-ahead log and free pages (secure delete, checkpoint and truncate the log, raw-byte scan test); backups made before the deletion still contain the name.
- CHK022: each save, correction or delete is one all-or-nothing unit; on failure nothing changes and a clear message says so.
- CHK023: two windows: saving a name changed since it was opened shows "Keep mine" or "Reload"; never a silent overwrite.
- CHK024, CHK025: import restores into an empty store; a non-empty store warns and requires "Replace everything"; an import that breaks a limit or the one-Legal rule is refused whole with every problem listed; nothing is partly imported.
- CHK026: each blocking rule (rebuild, limits, one Legal and one Preferred) is classified as an integrity rule under principle 26, stated where it is introduced.

**Accessibility**
- CHK027: menu label pattern "More options for <field name>".
- CHK028: tap-to-label is keyboard-operable (Tab, Space or Enter).
- CHK029: errors tied to their field, announced to screen readers, focus moves to the first error.
- CHK030: retired names show the text "No longer used" plus muted styling that meets 4.5:1 contrast.
- CHK031: target size 44 by 44 pixels for the ⋯ menus and the theme toggle (the toggle is owned by spec 010).
- CHK032: focus order follows visual order; visible focus in both themes; reflow at 320 CSS px width and 400% zoom.
- CHK033: SC-009 = WCAG 2.2 AA checked with axe-core through Playwright on every v1.0 screen in both themes, plus a manual keyboard-only pass.
- CHK034: plain language: short sentences, no jargon, each error says what to do next.

**Boundaries**
- CHK035, CHK036: spec 002 owns the change-log entry format and the JSON export envelope; names owns only its payloads and the names fragment.
- CHK037: assumptions about spec 002 are listed and checked when it lands; if it differs, names adapts through its ports and the spec is updated by pull request.
- CHK038: if a spec 010 element cannot meet a names need, names requests a change to 010 and never builds its own version; there are no throwaway placeholders, and the build order is spec 002, then spec 010, then the names screens (BUILD-01).
- CHK039: tasks cite FR numbers; the rule is stated in the spec.

Notes: the 44 by 44 pixel target is stricter than the 24 by 24 pixel minimum in the owner's master spec; the owner confirmed keeping it (2026-11-19), and the master spec is updated to match.
