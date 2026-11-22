# Feature Specification: Shared UI Elements (v1.0)

**Feature Branch**: `feat/010-shared-ui-spec`

**Created**: 2026-11-21

**Status**: Draft

**Input**: User description: "Spec 010: shared UI elements for v1.0 (BUILD-01: after spec 002, before the names screens in spec 001), from the master spec rows for 010 plus the theme toggle, toasts, the notification tray, NAMES-07 confirmation dialogs and the shared form-error pattern."

**Binding rule**: only items marked Confirmed are binding here (CLAUDE.md rule 2). Q1 and Q2 under "Decisions confirmed by the owner" were decided by the owner on 2026-11-22 and Q3 was confirmed on 2026-11-21.

**Build order (BUILD-01, Confirmed)**: spec 002 (foundation) first, then this spec, then the spec 001 screens on the real elements from this spec. No throwaway placeholders. This spec describes what the owner sees and can do; it names no technology (technology choices belong to the plan).

## Definitions

- **Shared element**: a visual and behavioral building block used by more than one screen, defined once here so every screen looks and acts the same.
- **Header**: the strip at the top of every screen holding the app name, navigation and the theme toggle.
- **Theme**: Light or Dark. The owner's *setting* is Light, Dark or Match my device.
- **Toast**: a short, transient notice that appears, then goes away by itself (UI-02, Confirmed).
- **Notification tray**: the list that keeps timestamped, dismissible entries (UI-03, Confirmed).
- **Notice**: one entry shown as a toast and kept in the tray.
- **Plain confirmation / typed confirmation**: the two confirmation dialogs of NAMES-07 (Confirmed). A confirmation is an extra step, never a refusal.
- **Rule ID**: the stable ID of a validation rule (`NAMEVAL-nn`, spec 001 FR-001d, SEC-05, LOG-01). Spec 001 owns the rules; this spec owns how a failure is shown.
- **History log**: the foundation's append-only change log (spec 002). This spec writes to it only through the foundation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Pick a theme and have it stay (Priority: P1)

The owner presses the sun/moon button in the header to switch between light and dark, or chooses Light, Dark or Match my device in Settings. The choice is remembered and the page appears in that theme from the first moment.

**Why this priority**: Sensory comfort is the reason UI-01 exists, and AT-41 and AT-42 are star acceptance tests. Every other element is styled in both themes, so this comes first.

**Independent Test**: Open any screen, press the toggle, reload; the same theme shows with no flash of the other. Run the accessibility check in both themes.

**Acceptance Scenarios**:

1. **Given** the Light theme, **When** the owner looks at the header, **Then** the button shows a moon and its spoken and hover label reads "Switch to dark mode".
2. **Given** the Dark theme, **When** the owner looks at the header, **Then** the button shows a sun and its label reads "Switch to light mode".
3. **Given** the owner chose Dark, **When** they close and reopen Meep, **Then** the first paint is already dark; no white frame appears (AT-41).
4. **Given** the setting is Match my device and the device changes theme, **When** Meep is open, **Then** Meep follows the device.
5. **Given** the header button, **When** its target size is measured, **Then** it is at least 44 by 44 pixels (NAMES-05).
6. **Given** every v1.0 screen in both themes, **When** the automated WCAG 2.2 AA check runs, **Then** it passes (AT-42).

---

### User Story 2 - The same elements on every screen (Priority: P1)

Every v1.0 screen is built from one set of elements: text field, ⋯ menu, button, toggle, list row, status label and the Settings page. Each looks and behaves the same everywhere, in both themes.

**Why this priority**: BUILD-01 puts the names screens on these elements; without them spec 001's screens cannot start.

**Independent Test**: A gallery of every element in every state (normal, focused, disabled, error), in both themes, passes the accessibility check and the keyboard walk-through.

**Acceptance Scenarios**:

1. **Given** any element, **When** the owner uses only the keyboard, **Then** it can be reached, operated and left, and the focus position is always visible.
2. **Given** a status label (for example "in use" or "no longer used"), **When** it is shown, **Then** its meaning is carried by words or a shape as well as color, and the palette stays distinguishable for color-vision deficiency.
3. **Given** a ⋯ menu, **When** it is shown, **Then** it has a screen-reader label and a target of at least 44 by 44 pixels (NAMES-05).
4. **Given** the device's reduced-motion setting is on, **When** any element animates, **Then** the motion is removed or reduced; nothing flashes or plays automatically.

---

### User Story 3 - Toasts and the notification tray (Priority: P2)

When something notable happens, a toast appears briefly. The same notice is kept in the notification tray with the time it happened, until the owner dismisses it. Every dismissal is recorded in the History log.

**Why this priority**: NAMES-07 requires a toast and a tray entry for keystone changes, so names depends on it, but it follows the elements it is built from.

**Independent Test**: Raise a test notice; the toast appears and goes away on its own, the tray still lists it with a timestamp, dismissing it removes it and the History log gains one `notice_dismissed` entry.

**Acceptance Scenarios**:

1. **Given** a notice is raised, **When** it appears, **Then** a toast shows and is announced to screen-reader users without taking focus.
2. **Given** an info or success toast is showing, **When** the owner does nothing, **Then** it goes away by itself after 8 seconds (never under 5) and the tray entry remains (UI-02, UI-03).
2a. **Given** an error toast is showing, **When** the owner does nothing, **Then** it stays until the owner closes it, and the tray entry remains.
3. **Given** a toast with a button or link, **When** the owner moves the pointer or keyboard focus onto it, **Then** it stays until they leave it, so no one is timed out mid-action.
4. **Given** the tray, **When** it is opened, **Then** each entry shows its message and its date and time.
5. **Given** a tray entry, **When** the owner dismisses it, **Then** it disappears from the tray and one `notice_dismissed` entry (carrying only the notice's enumerated ID) is appended through the foundation (UI-03, LOG-01 item 7, spec 002 T136).
6. **Given** the store is read-only (FOUND-03), **When** the owner dismisses an entry, **Then** the entry is hidden for the session and the screen says the dismissal could not be recorded; the dismissal is never silently lost.

---

### User Story 4 - Confirmation dialogs (Priority: P2)

Before a keystone or irreversible action, Meep shows an extra confirmation step. A plain confirmation asks "are you sure". A typed confirmation, for irreversible loss, makes the owner type a confirming word and offers to make a backup first.

**Why this priority**: NAMES-07 (Confirmed) and spec 002 FR-017 (Replace everything) need them, and they protect the owner's data.

**Independent Test**: Trigger each dialog with a synthetic action; confirm that Cancel changes nothing, that the typed dialog's confirm button stays unavailable until the exact word is typed, and that the backup offer is present.

**Acceptance Scenarios**:

1. **Given** a plain-confirmation action (changing the Legal name's text, making another name Legal, marking the Legal name no longer used, deleting any name), **When** the owner starts it, **Then** a dialog names the action and its effect and offers Confirm and Cancel.
2. **Given** a typed-confirmation action (Replace everything, permanent delete, restoring over an existing store), **When** the owner starts it, **Then** the dialog states in plain words that the loss cannot be undone, offers "Make a backup first", and keeps Confirm unavailable until the owner types the confirming word.
3. **Given** any confirmation, **When** the owner confirms, **Then** the action proceeds and raises a toast and a tray entry (NAMES-07); **When** they cancel or press Escape, **Then** nothing changes.
4. **Given** an action NAMES-07 lists as not keystone (editing Preferred, Nickname or Also known as entries, adding names, the pronoun opt-in, display settings), **When** the owner does it, **Then** no confirmation appears.
5. **Given** a confirmation is open, **When** it is used with a keyboard or screen reader, **Then** focus moves into it, stays inside until it closes, returns to where it was, and the typed field is labeled with the word to type.

---

### User Story 5 - Form errors that say what to do (Priority: P2)

When a field fails a validation rule, the same pattern appears on every form: a message next to the field that says what is wrong and what to do next, tied to the field for screen readers, with the rule's ID available for support and logs.

**Why this priority**: SEC-05 and LOG-01 require one rule ID across spec, message, log and tests; spec 001's forms need one place to show it.

**Independent Test**: Feed each synthetic rule failure through the pattern; the message, the field link and the rule ID all appear, and nothing the owner typed is copied into a log line.

**Acceptance Scenarios**:

1. **Given** a field that fails a rule, **When** the error is shown, **Then** the catalog message for that rule appears beside the field, says what to do next (spec 001 FR-034), and is announced to screen-reader users.
2. **Given** the error, **When** it is shown, **Then** the field is marked invalid by more than color, and the message is tied to the field.
3. **Given** a form with several failures, **When** the owner submits, **Then** a summary lists them all, each linking to its field, and focus moves to the summary.
4. **Given** a failure, **When** it is logged, **Then** the log carries the rule ID and the field reference only, never the typed text (SEC-05, LOG-01, spec 002 FR-049).
5. **Given** the typed text contains characters that look like markup or catalog placeholders, **When** the error is shown, **Then** the text is never interpreted, only displayed or omitted.
6. **Given** a rule ID with no catalog message, **When** the build checks messages, **Then** the build fails (R10).

---

### Edge Cases

- Several notices arrive at once: toasts queue, never stack off screen; every one still reaches the tray.
- The tray is empty: it says so in plain words.
- Many entries: the tray stays usable by keyboard and screen reader, with the newest first.
- A toast is raised while a confirmation dialog is open: it waits until the dialog closes.
- The theme setting cannot be saved: the theme still applies for the session and the owner is told once.
- Text is enlarged by the browser to 200 percent: no element loses content or function (WCAG 2.2 AA).
- A notice's message is not in the catalog: the build fails; a toast never shows a raw ID.
- The owner types the confirming word with different capitalization or extra spaces: it still matches (Q2). If the action has no verb in the chosen language's catalog, the English verb shows (FR-036).

## Requirements *(mandatory)*

### Functional Requirements

**Elements (UI-01)**

- **FR-001**: The system MUST provide these shared elements and use them on every v1.0 screen: text field, ⋯ menu, button, toggle, list row, status label and a Settings page (UI-01).
- **FR-002**: Every element MUST be operable with the keyboard alone, show a visible focus position, and carry a screen-reader name and role.
- **FR-003**: Targets for menus and the theme toggle MUST be at least 44 by 44 pixels (NAMES-05, BUILD-01 item 4).
- **FR-004**: Status MUST never be shown by color alone, and the palette MUST be checked for color-vision deficiency.
- **FR-005**: Motion MUST respect the device's reduced-motion setting; nothing flashes or plays automatically.
- **FR-006**: All owner-visible text MUST come from Fluent message files, English only in v1.0 (R10); none appears in code. Messages are plain language and say what to do next (spec 001 FR-034).

**Theme (UI-01, AT-41, AT-42)**

- **FR-007**: The header MUST hold an icon button that switches between light and dark on every screen. It shows a moon in light mode and a sun in dark mode (the mode it will switch to) and carries a spoken and hover label such as "Switch to dark mode".
- **FR-008**: Settings MUST offer Light, Dark and Match my device. The choice is remembered for this owner and applied before the page first draws, so the other theme never flashes (AT-41).
- **FR-009**: Both themes MUST meet WCAG 2.2 AA contrast, and an automated check of every v1.0 screen in both themes MUST pass (AT-42).

**Toasts (UI-02)**

- **FR-010**: A toast MUST be a notice that is announced to assistive technology without taking focus. An info or success toast goes away by itself after 8 seconds, never under 5, and stays while the pointer or keyboard focus is on it. An error toast never goes away by itself: the owner closes it (UI-02, Q1).
- **FR-011**: A toast that holds an action MUST NOT go away while the pointer or keyboard focus is on it, whatever its kind.
- **FR-012**: Every toast MUST also create a tray entry, so a missed toast is never lost (NAMES-07, UI-03).

**Notification tray (UI-03)**

- **FR-013**: The tray MUST keep timestamped entries, newest first, until the owner dismisses them (UI-03).
- **FR-014**: Dismissing an entry MUST record one `notice_dismissed` entry in the History log through the foundation, carrying only the notice's enumerated ID, never its text (UI-03, LOG-01 item 7, spec 002 T136).
- **FR-015**: If the dismissal cannot be recorded, the owner MUST be told and the entry MUST NOT be silently lost (spec 002 FR-015 all-or-nothing writes).
- **FR-016**: Tray entries survive closing and reopening Meep until dismissed. Whether they are stored in the foundation or elsewhere is a planning matter; Q3 (Confirmed) puts them in the foundation store.

**Confirmations (NAMES-07)**

- **FR-017**: Plain-confirmation actions MUST show a dialog that names the action and its effect, with Confirm and Cancel. The actions are: changing the Legal name's text, making a different name Legal, marking the Legal name no longer used, deleting any name (NAMES-07).
- **FR-018**: Typed-confirmation actions MUST show a dialog that warns, in plain words, that the loss cannot be undone, offers to make a backup first, and enables Confirm only after the owner types the confirming word. The actions are: Replace everything, permanent delete, restoring over an existing store (NAMES-07, spec 002 FR-017).
- **FR-019**: A confirmation MUST be an extra step, never a refusal: after Confirm the action proceeds, and Cancel or Escape leaves everything unchanged (NAMES-07).
- **FR-020**: A confirmed keystone action MUST raise a toast and a tray entry (NAMES-07, UI-02, UI-03).
- **FR-021**: Dialogs MUST trap focus while open, return focus when closed, and be operable by keyboard and screen reader.
- **FR-022**: The dialog elements MUST NOT decide which actions need confirmation; the calling feature names the action and its level. NAMES-07's not-keystone list (editing Preferred, Nickname or Also known as entries, adding names, the pronoun opt-in, display settings) shows no confirmation.

**Form errors (SEC-05, LOG-01)**

- **FR-023**: A field failure MUST show the catalog message for its rule ID beside the field, state what to do next, be linked to the field for screen readers, and mark the field invalid by more than color.
- **FR-024**: A form with several failures MUST show a summary of all of them, each linking to its field, with focus moved to the summary on submit.
- **FR-025**: The rule ID shared by the spec, the Fluent message, the log and the tests MUST be available with the error for support and logging; logs carry the rule ID and the field reference only, never the typed text (SEC-05, LOG-01, spec 002 FR-049).
- **FR-026**: Typed text MUST never be interpreted as markup or as a message placeholder when it is displayed.
- **FR-027**: A build check MUST fail when a rule ID or a notice has no catalog message (R10).

**Shared form-error pattern (VAL-01, Confirmed)**

- **FR-028**: Validation on the server is the authority; any check in the browser is a hint only (VAL-01 item 1).
- **FR-029**: Every failure of a form MUST come back at once: an error summary at the top that links to each field, plus an inline message per field linked to it with `aria-describedby` (WCAG 3.3.1, 3.3.3) (VAL-01 item 3). FR-023 and FR-024 are this pattern.
- **FR-030**: The owner's input MUST be kept on error and never cleared (VAL-01 item 4); this includes the stale-save case, where the message `record-changed` is shown with the typed input still on screen so the owner can copy it before reloading (spec 002 contracts/owner-messages.md).
- **FR-031**: Validation runs on save, not on a draft. Future field types (dates, phone numbers, member IDs, v1.1) register their rules in this pattern; no feature invents its own error display (VAL-01 items 5 and 6).

**Error surfaces and severity (ERR-01 item 7, Confirmed)**

- **FR-032**: Each failure appears on the surface that fits it: validation failures inline per field, all at once, with their rule IDs; a transient save or backup failure as a toast plus a tray entry; a store-level failure (wrong key, damaged store, read-only store) as a blocking screen.
- **FR-033**: Every catalog message carries a severity: info, warning, error or blocking. The severity decides the surface, the announcement to assistive technology (blocking and error are announced assertively) and the styling, and is never shown by color alone (FR-004).

**Language (LANG-02, Confirmed)**

- **FR-034**: Settings MUST offer a Language choice from v1.0. Only English ships until the first stable long-term release is accepted; more languages come after that (Tsalagi, English, Spanish, DOD-16) (LANG-02 item 1).
- **FR-035**: The UI language MUST NOT change how names are entered, split, stored or displayed. Format locale (dates, numbers) is a separate future setting, and neither implies the other (LANG-02 item 2).
- **FR-036**: If a message is missing in the chosen language, the English text MUST show, never a raw message ID. A build check MUST fail if any English ID is missing (LANG-02 item 3, R10).
- **FR-037**: A test-only pseudo-locale (code `en-XA`; accented characters and about 40 percent longer text) MUST exist, MUST NOT be offered in release builds, and the browser accessibility suite MUST run once under it to catch hard-coded text and broken layout (LANG-02 item 4).
- **FR-038**: The language choice is stored per owner and logged only as a fixed code (LANG-02 item 5, LOG-01).

### Key Entities

- **Theme setting**: the owner's choice (Light, Dark, Match my device), kept per owner.
- **Notice**: an enumerated ID, a catalog message, the time it was raised, and a dismissed state. Notice IDs are enumerations, never free text (spec 002 FR-049).
- **Confirmation request**: the action name, its level (plain or typed) and its effect text, supplied by the calling feature.
- **Field error**: the rule ID, the field reference and the catalog message.
- **Message severity**: info, warning, error or blocking, a property of each catalog message (FR-033).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Across 100 reloads in each theme setting, the page's first paint is in the chosen theme in 100 of 100 (AT-41).
- **SC-002**: The automated WCAG 2.2 AA check reports zero failures on every v1.0 screen in both themes (AT-42).
- **SC-003**: Every element in the gallery can be reached and operated with the keyboard alone, with the focus position visible in every state.
- **SC-004**: Every notice raised in a test creates exactly one tray entry with a timestamp, and every dismissal creates exactly one `notice_dismissed` History entry.
- **SC-005**: Every plain and typed confirmation can be cancelled with no change in data, verified by comparing the store before and after.
- **SC-006**: For every rule ID in the test table, the message, field link and rule ID are all present, and a search of the logs finds none of the typed text.
- **SC-007**: A check over the message files finds no rule ID and no notice without a message, and no owner-visible text outside the catalog.
- **SC-008**: Menus and the theme toggle measure at least 44 by 44 pixels on every screen.

## Out of scope

- **App auto-lock warning** (the WCAG 2.2.1 "extend" option, 30-second default) and the **resume/discard draft diff**: both vNext by owner decision (2026-11-20). See `docs/reviews/Meep-Access-Review_2026-11-20_0848-CT.md` (D3 and the auto-lock and unsaved-changes follow-ons). Nothing here may assume them.
- Login, sign-in screens, users and delegates (R5; AUTH-01 is vNext).
- AI and cloud features.
- Languages other than English; font size, menu size, text reader and haptics settings; Android (the DOD-16 items beyond v1.0).
- The names screens, name rules and name validation (spec 001); this spec only shows their failures.
- The foundation's store, log and keys (spec 002); this spec uses them through their capabilities.
- Distribution and updates (spec 015).
- Everything labeled v1.1 or later.

## Assumptions

- Spec 002 supplies the History log, the `notice_dismissed` kind (T136) and the message catalog; this spec adds no storage of its own beyond what Open question Q3 settles.
- The theme setting and any tray state are kept for the owner on this computer only, never sent anywhere (R5).
- Interface language is English only (R10).
- Examples in tests and documents use synthetic data only (CLAUDE.md rule 4).
- Dependencies: spec 002 (BUILD-01); spec 001 FR-001d for rule IDs and FR-035 for the keystone list.

## Decisions confirmed by the owner (2026-11-21 and 2026-11-22)

- **Q1 (toast timing, hybrid)**: **Confirmed (owner, 2026-11-22; supersedes the 2026-11-21 default and the directive's)**: info and success toasts stay 8 seconds, never under 5, paused while hovered or focused; error toasts never auto-dismiss and the owner closes them; every toast is also recorded in the notification tray (UI-03).
- **Q2 (typed word)**: **Confirmed (owner, 2026-11-22; replaces "DELETE")**: the owner types the action's own verb (for example "delete", "replace", "restore"), taken from the Fluent catalog in the UI language; matching ignores capitalization and surrounding spaces.
- **Q3 (tray storage)**: Do tray entries live in the foundation's store (so they are encrypted and travel with a backup) or only on this computer's browser? **Confirmed**: the foundation's store, so nothing about the owner's activity sits outside the encrypted store (FOUND-02). Why it matters: it decides whether spec 002 needs one more record kind and whether a backup includes the tray.

