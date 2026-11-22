# LANG-02: UI language

- **ID:** LANG-02
- **Status:** Confirmed
- **Date:** 2026-11-21
- **Owner:** Owner (ORC directive 2026-11-21 20:50 CT, Part D; ID assigned 2026-11-22 because LANG-01 is the implementation language)

## Decision

1. Settings has a **Language** choice from v1.0. Only English ships until the first stable long-term release (LTS) is accepted. More languages come after that, from the DOD-16 list (Tsalagi, English, Spanish). DOD-16's timing in the master spec copy is updated to say so.
2. **UI language never changes how names are entered, split, stored or displayed.** There is no profile-level naming-convention setting: spec 001 has the owner choose the convention per name, in the Last field menu (FR-012, FR-016), and that stays. Format locale (dates, numbers) is a separate future setting. None of the three implies another.
3. **Fallback:** if a message is missing in the chosen language, show English, never a raw ID. CI fails if any `en` ID is missing.
4. **Pseudo-locale:** test-only, never offered in release builds. Accented characters plus about 40 percent text expansion (code `en-XA`, NAMING.md section 6). The Playwright and axe suite runs once under it to catch hard-coded text and layout breakage. This is part of spec 010's test plan, not v1.0 shipping scope.
5. The language choice is stored per user and logged only as a fixed code.

## Notes

- Implementation goes in spec 010 (language setting, fallback and pseudo-locale requirements).
- Not the same as `LANG-01` (master spec: implementation language is Rust, Confirmed). Both belong to the `LANG` prefix; see the registry.
- Related: R10 (Fluent message files, English only in v1.0), DOD-16.
