# Contract: Screens (names)

What the owner sees and does. Server-rendered pages, minimal JavaScript. Shared elements (text field, ⋯ menu, button, toggle, list row, status label, Settings page, theme toggle) come from spec 010; names only composes them. Every visible string is a message ID in the Fluent catalog (R10); nothing below is literal text except as an example of intent.

## Access rules (all pages)

- Served only on the loopback address; no sign-in (R5).
- Every request that changes data carries a per-session anti-forgery token; requests with a non-local `Host` or `Origin` are rejected.
- Each page works without JavaScript for its core action; JavaScript only improves convenience.
- WCAG 2.2 AA in both themes (spec 010 owns the checks); ⋯ menus are labeled for screen readers and at least 44 x 44 pixels (spec Identity and names).

## Screen 1: Names (list)

- Shows each name with its use, status (Active or No longer used) and the short form where space calls for it.
- Retired names are shown greyed out, never hidden (ID-01).
- Status is shown by text as well as colour.
- Actions per name: edit, retire, "Mark as in use again" on a retired name's menu (FR-028), "Make this my legal name" on Also known as entries, delete.
- Empty state: a prompt to add the first name.
- Over 50 names: the add action explains the limit (NAMES-03).

## Screen 2: Add or edit a name

Fields and controls:

1. **First name**, **Middle name**, **Last name**: each accepts several words, 1 to 128 user-perceived characters; only one of First or Last is required (ID-05).
2. A ⋯ menu beside each field offering: change a word's type; split or keep together; two separate last names (Last only) with optional father's and mother's labels; "This is the name I go by"; initial only; optional origin, owner-set: birth or from a spouse (ID-05, ID-07, ID-08).
3. **Use** selector: Legal, Preferred, Also known as, Nickname (ID-01). Selecting Legal or Preferred when one exists explains the rule (R12, R13).
4. **More name options** (text-labeled link): prefix, suffix, particle, the **Short name** box (labeled by purpose, placeholder shows the computed default, NAMES-04), and "My name doesn't fit these boxes" (ID-05).
5. **Full-name box** (up to 384 characters) with tap-to-label; produces the same labeled parts as the three boxes (AT-27).
6. Optional end date when marking a name No longer used (ID-01).

Behaviour:

- Entered text is normalized and trimmed as in data-model; the rebuild check runs on save and a failure shows a clear error with nothing written (ID-04).
- Over a limit: a clear message, text kept in the box, nothing saved, nothing cut off (NAMES-03).
- The three boxes never split "José Luis" unless the owner chooses to (AT-31).

## Screen 3: Settings, pronouns section

- Off by default; no pronoun appears anywhere until the owner turns it on (PRON-01, AT-23).
- When on: options she/her, he/him, they/them, use my name, custom (PRON-01).
- Visibility shows exactly two choices, "Only me" and "Included in exports", defaulting to "Only me"; no "Delegates" choice in v1.0 (PRON-01a).
- Turning pronouns off again hides them everywhere and in outbound exports (spec Story 5, scenario 8).

## Behaviour added by the checklist review (NAMES-05)

- ⋯ menus are labeled "More options for <field name>" and are at least 44 x 44 pixels; tap-to-label words are buttons reached with Tab and toggled with Space or Enter (FR-013, FR-014).
- Errors are tied to their field, announced to screen readers, and focus moves to the first error; everything typed stays on screen (FR-001a, FR-032).
- Retired names show the text "No longer used" plus muted styling that meets 4.5:1 contrast (FR-032).
- Editing a name asks "Fix a typo" or "Add a new name"; Meep never guesses (FR-002a).
- Saving a name that changed since it was opened shows "Keep mine" or "Reload", never a silent overwrite (FR-031).
- Backup and restore: the owner backup downloads everything; restoring works only into an empty store, otherwise it warns and needs "Replace everything"; an import that breaks a limit or the one-Legal rule is refused whole with every problem listed (FR-029).
- The focus order follows the visual order, focus is visible in both themes, and the page reflows at 320 CSS pixels and 400% zoom (FR-032).

## Errors and messages

All error and limit messages are plain language, say what to change, and use Fluent message IDs. Integrity errors (rebuild mismatch) and limit errors block the save; nothing else in names blocks the owner (principle 26).
