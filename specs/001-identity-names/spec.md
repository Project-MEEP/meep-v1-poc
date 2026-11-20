# Feature Specification: Identity and Names (v1.0)

**Feature Branch**: `001-identity-names`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "Feature 001, Meep v1.0 Identity and names. Source: Meep Spec v1.0 (2026-11-19 20:52 CT), section Identity and names (v1.0), the decision records in docs/decisions/, and the constitution. Names and pronouns only: ID-01 to ID-05, ID-07 to ID-10, PRON-01, R4, R12 to R16. Out of scope: ID-06, AT-24, AT-25, AT-32, insurance, login, delegates, any AI or cloud feature."

**Binding rule**: only items the Meep spec marks Confirmed are binding here. Anything Proposed or Open appears as a clarification question, not a decision.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Add my name in three familiar boxes (Priority: P1)

The owner adds a name by typing First name, Middle name and Last name, like any
common form. Behind the scenes Meep keeps the name as labeled parts that rebuild
the full text exactly, so nothing downstream ever has to guess or re-split a
name (ID-04, ID-05).

**Why this priority**: it is the smallest slice that delivers the feature: a
person can record their name correctly. Everything else builds on it.

**Independent Test**: enter the Legal name "Jordan Lee Rivera Okafor" in the
three boxes, save it, and confirm it is listed exactly as entered and exports
unchanged.

**Acceptance Scenarios**:

1. **Given** an empty names list, **When** the owner enters First "Jordan", Middle "Lee", Last "Rivera Okafor" and saves, **Then** the name is stored as one Legal name whose Last name is a single compound surname (ID-08) and whose full text reads "Jordan Lee Rivera Okafor".
2. **Given** the three-field form, **When** the owner enters only a First name, or only a Last name, **Then** the name saves; with neither, saving is refused with a clear message (ID-05).
3. **Given** the three-field form, **When** the owner leaves Middle blank, **Then** nothing is stored for Middle and no filler such as "NMN" appears (ID-05).
4. **Given** First name "José Luis" entered in the First box, **When** the owner saves without changing anything in any menu, **Then** it stays one given name and is not split (AT-31).
5. **Given** any saved name, **When** its parts are joined in order with a single space and no added punctuation, **Then** the result equals the full text exactly; if it would not, the save is refused as an integrity error (ID-04, R14, AT-28).
6. **Given** a name typed with extra spaces such as "  Jordan   Rivera ", **When** it is saved, **Then** leading and trailing spaces are trimmed, runs of spaces collapse to one, and apostrophes and hyphens stay exactly as typed (ID-09).

---

### User Story 2 - Keep several names and say which are in use (Priority: P1)

The owner keeps more than one name: a Legal name, a Preferred name, Also known
as names and Nicknames. Each is either Active (in use now) or No longer used.
Retired names stay visible, greyed out, so old records still match (ID-01, ID-02).

**Why this priority**: real people use more than one name, and changes of name
must never lose history.

**Independent Test**: create the five synthetic names from the fixtures list,
retire one, and confirm the list, statuses and rules behave as described.

**Acceptance Scenarios**:

1. **Given** a Legal name exists, **When** the owner tries to add a second Legal name, **Then** it is refused; exactly one Legal name exists at any time (R12).
2. **Given** a Preferred name exists, **When** the owner tries to add another Preferred name, **Then** it is refused; at most one exists (R12). Any number of Also known as and Nickname entries are allowed (R12).
3. **Given** a Preferred name is blank, **When** the owner views or exports names, **Then** the Legal name is used as the Preferred name (ID-01).
4. **Given** a Legal and a Preferred name, **When** the owner tries to switch either to No longer used, **Then** it is refused, because Legal and Preferred are always Active (ID-01, ID-02).
5. **Given** an Also known as name that is Active, **When** the owner marks it No longer used, **Then** it stays in the list greyed out, and the owner may add an optional end date (ID-01, ID-02).
6. **Given** a Preferred name "Jordan Rivera", **When** the owner replaces it with a new Preferred name, **Then** the old one becomes an Active Also known as that the owner may later retire (R13).
7. **Given** an Also known as entry, **When** the owner chooses "Make this my legal name", **Then** it becomes the Legal name and the former Legal name becomes an Active Also known as that the owner may retire (ID-01).
8. **Given** several names, **When** more than one is Active, **Then** this is allowed (ID-02).

---

### User Story 3 - Handle names that do not fit three boxes (Priority: P2)

The owner can describe names with compound given names, two surnames, particles,
prefixes, suffixes and initials, using a small menu beside each field, a "More
name options" link, or a full-name box for names that do not fit at all
(ID-05, ID-07, ID-08).

**Why this priority**: the three-box path covers most people; this makes the
rest work without forcing a wrong shape on anyone.

**Independent Test**: enter "María José García López" as a two-surname name and
"Jordan Lee Pak" with a birth-origin label, and confirm each stores the right
labeled parts.

**Acceptance Scenarios**:

1. **Given** the Last field, **When** the owner chooses two separate last names and optionally labels them father's and mother's, **Then** the name stores two ordered family-name parts with those labels (ID-07).
2. **Given** a Last field with several words, **When** the owner changes nothing, **Then** it is stored as one compound surname (ID-08); the owner may split it from the field menu.
3. **Given** a field menu, **When** the owner chooses "This is the name I go by", **Then** that word is recorded as the name the owner goes by.
4. **Given** a field menu, **When** the owner chooses "Initial only", **Then** the initial is kept with its period, for example "L." (ID-05).
5. **Given** a field menu, **When** the owner sets an origin of birth or from a spouse, **Then** the part carries that label; origin is always owner-set (ID-04, ID-05).
6. **Given** "More name options", **When** the owner adds a prefix, suffix or particle such as "de", "del" or "van", **Then** each is stored in its own labeled role and placed in the correct order when the name is rebuilt (ID-05).
7. **Given** "My name doesn't fit these boxes", **When** the owner types one full name and tap-labels each word, **Then** the stored labeled parts are identical to those produced by three-field entry for the same name (AT-27).
8. **Given** a field menu, **When** the owner changes a word's type or chooses to split or keep words together, **Then** the stored parts reflect that choice and still rebuild the full text exactly (ID-04).
9. **Given** the v1.0 conventions, **When** names are entered, **Then** only the US form and the Spanish two-surname form are supported; other conventions are not offered (ID-07).

---

### User Story 4 - Names survive accents, saving and export (Priority: P2)

Names with accents and special characters are saved, shown and exported without
changing a single character. Exports to the outside formats are built from the
stored labeled parts, never by re-guessing a name from text (ID-04, ID-10).

**Why this priority**: a name that changes on the way out defeats the purpose.

**Independent Test**: save "María José García López", export it in each format,
re-import, and compare character by character.

**Acceptance Scenarios**:

1. **Given** the name "María José García López", **When** it is saved, displayed and exported, **Then** every accent is preserved exactly (AT-26).
2. **Given** a name entered with three fields and no menu changes, **When** it is exported to the US health-data format (First, Middle, Last), **Then** the three values are unchanged (AT-30).
3. **Given** any name, **When** it is exported to any outside format, **Then** the export is built from the labeled parts, and anything the target format cannot hold is recorded as an Also known as rendering (AT-29).
4. **Given** names with use types, **When** exported to the standard healthcare name format, **Then** Legal maps to official, Preferred to usual, Nickname to nickname, and Also known as to usual while Active and old once No longer used (ID-10).
5. **Given** the owner's names, **When** they are exported and re-imported, **Then** the re-imported records equal the originals (spec 016 round trip, as it applies to names).
6. **Given** two names that differ only by accents, **When** the owner searches or compares, **Then** an accent-folded key is used for matching only and never changes what is stored (ID-04).

---

### User Story 5 - Pronouns stay hidden until I opt in (Priority: P3)

Pronouns appear nowhere in Meep until the owner turns them on in Settings. When
off, the interface uses the name or "you" and never guesses (PRON-01, R4).

**Why this priority**: it is a comfort and respect feature rather than a
dependency for names.

**Independent Test**: start a fresh install, confirm no pronoun appears anywhere,
enable pronouns in Settings, and confirm they then appear as configured.

**Acceptance Scenarios**:

1. **Given** a new install, **When** the owner browses every screen and export, **Then** no pronoun appears anywhere (AT-23).
2. **Given** pronouns are off, **When** Meep refers to the owner, **Then** it uses the owner's name or "you" and never guesses a pronoun (PRON-01).
3. **Given** Settings, **When** the owner enables pronouns, **Then** the options she/her, he/him, they/them, use my name, and custom are offered (PRON-01).
4. **Given** the owner enables pronouns for the first time, **When** the visibility setting first appears, **Then** it defaults to "Only me" (owner clarification, 2026-11-19).
5. **Given** pronouns are enabled, **When** the owner opens the visibility setting, **Then** exactly two choices are shown, "Only me" and "Included in exports"; "Delegates" is not shown in v1.0 (owner clarification, 2026-11-19; PRON-01).
6. **Given** visibility is "Only me", **When** the owner produces an outbound name export (US health-data form or the standard healthcare name format), **Then** pronouns are not included; **Given** visibility is "Included in exports", **Then** they are included (PRON-01).
7. **Given** pronouns are enabled or disabled and any visibility choice, **When** the owner makes their own full JSON export or backup of all records, **Then** pronouns are always included, regardless of the visibility setting, so the owner never loses their own data (owner clarification, 2026-11-19; principle 27, no lock-in).
8. **Given** pronouns are turned off again, **When** the owner views any screen or makes an outbound export, **Then** pronouns are hidden again and not exported (PRON-01, AT-23).

---

### Edge Cases

- A name with only one word: it saves if it is a First or a Last name; Middle alone is refused (ID-05).
- Names containing apostrophes or hyphens (for example a double-barrelled surname): kept exactly as typed (ID-09).
- A name that cannot be rebuilt exactly from its parts: save fails and nothing is written (ID-04, principle 26 data-integrity block).
- Typing a stray comma before a suffix: Meep never inserts punctuation, so "Smith Jr." exports as "Smith Jr." and "Smith, Jr." appears only if the owner typed the comma (R14).
- Correcting a typo in an existing name: allowed, and recorded as an edit in the change history (spec 002 foundation).
- Deleting a name: the content is destroyed beyond recovery while the change history records only that a name was deleted, with the date (R8).
- The owner marks every Also known as and Nickname No longer used: allowed; Legal and Preferred remain Active (ID-02).
- Two surnames versus one compound surname: the default is one compound surname; the owner must choose two (ID-07, ID-08).
- Names in a convention other than US or Spanish two-surname: not specially supported in v1.0, and the full-name box is the escape hatch (ID-07).
- Very long names: [ASSUMPTION: accepted up to a generous limit with a clear message beyond it; the limit is a planning decision].

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST let the owner add a name with First, Middle and Last fields, each accepting several words, with only one of First or Last required and Middle never required (ID-05).
- **FR-002**: The system MUST store each name as exactly-as-entered text (with accents preserved) plus an ordered list of labeled parts, and MUST refuse to save any name whose parts do not rebuild the text exactly (ID-04, R14, AT-28).
- **FR-003**: The system MUST rebuild full text by joining parts in order with one space and MUST NOT insert punctuation (R14).
- **FR-004**: The system MUST trim leading and trailing spaces, collapse runs of spaces to one, and keep apostrophes and hyphens as typed (ID-09).
- **FR-005**: The system MUST support name types Legal name, Preferred name, Also known as and Nickname (ID-01).
- **FR-006**: The system MUST mark every name Active or No longer used, allow several Active names, keep Legal and Preferred always Active, and show retired names greyed out (ID-01, ID-02).
- **FR-007**: The system MUST allow exactly one Legal name, at most one Preferred name, and any number of Also known as and Nickname names (R12).
- **FR-008**: The system MUST fall back to the Legal name when the Preferred name is blank (ID-01).
- **FR-009**: When the Preferred name is replaced, the system MUST turn the old one into an Active Also known as that the owner may retire (R13).
- **FR-010**: The system MUST offer "Make this my legal name" to promote another name, turning the former Legal name into an Active Also known as (ID-01).
- **FR-011**: The system MUST offer an optional end date when a name is switched to No longer used (ID-01).
- **FR-012**: The system MUST treat several words in the Last field as one compound surname by default and let the owner split it, or choose two separate last names with optional father's and mother's labels (ID-07, ID-08).
- **FR-013**: The system MUST provide, beside each field, a labeled menu offering: change a word's type, split or keep together, two separate last names, "This is the name I go by", initial only, and an owner-set origin of birth or from a spouse (ID-05).
- **FR-014**: The system MUST provide "More name options" with prefix, suffix, particle and a full-name escape hatch with tap-to-label whose stored result equals three-field entry (ID-05, AT-27).
- **FR-015**: The system MUST keep initials with their period (ID-05).
- **FR-016**: The system MUST support only the US form and the Spanish two-surname form in v1.0 (ID-07).
- **FR-017**: The system MUST export names to the US health-data name form (First, Middle, Last, with prefix and suffix) and to the standard healthcare name format, build every export from the labeled parts, and log anything a target cannot hold as an Also known as rendering (ID-04, ID-10, AT-29, AT-30).
- **FR-018**: The system MUST map name types to the standard healthcare use values as ID-10 states (Legal to official, Preferred to usual, Nickname to nickname, Also known as to usual while Active and old when No longer used).
- **FR-019**: The system MUST export and re-import all names as an open, documented format with equal results (principle 27 is Proposed, so this rests on the Confirmed JSON export and import in the v1.0 foundation).
- **FR-020**: The system MUST keep a separate accent-folded key for matching only and MUST NOT merge or change records based on it (ID-04).
- **FR-021**: The system MUST compute a default short form of the owner's first given name plus the initial of the first surname and a period (for example "Jordan R."), using the Preferred name or the Legal name if none, and an owner-set override MUST always win (R15).
- **FR-022**: The system MUST hide pronouns everywhere until the owner enables them in Settings, MUST use the name or "you" instead, and MUST never guess (PRON-01, R4, AT-23).
- **FR-023**: When pronouns are enabled, the system MUST offer she/her, he/him, they/them, use my name, and custom (PRON-01).
- **FR-024**: The system MUST let the owner choose pronoun visibility from exactly two choices in v1.0, "Only me" and "Included in exports", defaulting to "Only me" when pronouns are first enabled. "Delegates" MUST NOT be shown in v1.0, but the stored setting MUST be able to hold it later without a data migration of existing records (PRON-01, owner clarification 2026-11-19).
- **FR-024a**: "Exports" in the visibility setting means outbound exports only (the US health-data form and the standard healthcare name format). The owner's own full JSON export and backup MUST always include pronouns regardless of the visibility setting (owner clarification 2026-11-19; principle 27, no lock-in).
- **FR-025**: The system MUST make each name's address segment valid, with exactly five dot-separated segments starting with a letter and containing only letters, digits and hyphens, compared case-insensitively (R6, R16).
- **FR-026**: The system MUST make every interface string available in a message catalog so more languages can be added without rewriting screens; v1.0 ships English only (R10).
- **FR-027**: The system MUST NOT include any delegate, login, insurance, outside-record linking (ID-06) or AI feature in this feature.

### Key Entities

- **Name**: one name the owner uses. Has full text exactly as entered, a use (Legal, Preferred, Also known as, Nickname), an Active or No longer used status, an optional period (start and end), an optional short form, and an ordered list of parts.
- **Name part**: one labeled piece of a name, with a role (given, additional given, family, particle, prefix, suffix), its position, and optional qualifiers (father's, mother's, birth, spouse, "the name I go by", initial).
- **Pronoun setting**: the owner's on or off choice, the chosen pronouns, and a visibility value (Only me, Included in exports, and later Delegates). Only the first two are selectable in v1.0.
- **Owner**: the single person whose names these are, with no sign-in in v1.0 (R5).
- **Test fixtures (synthetic)**: Jordan Lee Rivera Okafor (Legal, compound surname); Jordan Rivera (Preferred); "Jordan L. Rivera" (Also known as, Active, as on a card); Jordan Lee Pak (Also known as, No longer used, birth-origin label); María José García López (Legal, two surnames and accents). No real personal data and no email addresses are used anywhere.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: An owner can add a Legal name using the three fields in under one minute without opening any menu.
- **SC-002**: 100% of saved names rebuild their full text exactly from their parts, with zero silent changes (AT-28).
- **SC-003**: 100% of the synthetic fixtures round-trip through save, display, export and re-import with every character, including accents, unchanged (AT-26).
- **SC-004**: A name entered through the full-name box produces the same labeled parts as three-field entry for the same name in every fixture case (AT-27).
- **SC-005**: With a fresh install, zero pronouns appear on any screen or export until the owner turns them on (AT-23); once on, they appear in outbound exports only if the owner chose "Included in exports", and always appear in the owner's own full JSON export.
- **SC-006**: 100% of three-field entries with no menu changes export to First, Middle, Last unchanged (AT-30), and "José Luis" in First stays one given name (AT-31).
- **SC-007**: Every export is built from parts, and every value a target format cannot hold is recorded as an Also known as rendering (AT-29).
- **SC-008**: In an owner usability check, at least 9 of 10 first-time users complete adding both a Legal and a Preferred name without help.
- **SC-009**: All screens in this feature meet the accessibility level stated in the v1.0 spec (WCAG 2.2 AA in both themes), including labeled, adequately sized menu controls.

## Assumptions

- The v1.0 foundation (record envelope, encrypted local storage, change history, JSON export and import, no sign-in, owner's machine only) is delivered by spec 002 and provides the saved, deleted and exported behavior this feature relies on.
- This feature uses the shared interface elements and theme toggle from spec 010 and adds no navigation of its own.
- Pronouns are included in v1.0 (R4 Confirmed), hidden by default. Visibility choices and the always-included JSON export rule come from the owner's clarification of 2026-11-19; principle 27 itself is still marked Proposed in the constitution, so this rule rests on that clarification.
- English is the only interface language in v1.0, but all text lives in a message catalog (R10); the Spanish gender-neutral copy rule is Proposed and not applied.
- Names are US or Spanish two-surname only (ID-07); Tsalagi and other conventions are later.
- Name length limits and any cap on the number of names are set during planning.
- No outside record exists in v1.0, so linking names to outside records (ID-06) and warnings when retiring a linked name (AT-32) belong to v1.1.
- Matching a document name against names on record (AT-25) and delegate visibility of the legal name (AT-24) belong to v1.1.

## Out of Scope (v1.0)

Outside-record linking (ID-06) and its tests AT-24, AT-25, AT-32; insurance cards; login, SSO and delegates; any AI or cloud feature; languages other than English; name conventions other than US and Spanish two-surname; whether a caregiver can hide a patient's pronouns (PRON-02, Open).
