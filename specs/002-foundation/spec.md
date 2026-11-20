# Feature Specification: Foundation (v1.0)

**Feature Branch**: `002-foundation-spec`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "Feature 002, Meep v1.0 Foundation. Source: Meep Spec v1.0 (2026-11-19 22:22 CT), the v1.0 minimum foundation, Engineering rules, R2, R5, R6 to R9, R16, principles 16, 26 and 27, the Verification and Risks tables, the decision records in docs/decisions/ (FOUND-01, BUILD-01, R2, R5, R7, R8, R9, R16, NAMES-01, NAMES-02, NAMES-05) and the interfaces feature 001 needs. Out of scope: names (spec 001), shared UI elements and theme (spec 010), distribution, signing and updates (spec 015), login, delegates, caregivers, insurance, any AI or cloud feature, v1.1 and later."

**Binding rule**: only items the Meep spec marks Confirmed are binding here. Anything Proposed or Open appears as a clarification question, not a decision.

**Build order (BUILD-01)**: this feature comes first, then spec 010 (shared UI elements), then spec 001 (names) screens. Features reach the foundation only through the capabilities described here.

## Definitions

- **Record**: one piece of the owner's information that a feature saves (for example a name). Every record has the same outer wrapper, the envelope.
- **Envelope**: the set of standard fields every record carries: its identifier, the account ID, its address, its provenance, the statement that Meep claims no authority over it, and its timestamps.
- **Store**: the owner's encrypted local collection of all records and the change log.
- **Change log**: the append-only history of what happened to records, which never contains a record's content.
- **Owner backup**: the owner's own full export of everything Meep holds, in an open, documented format, made for the owner and never for anyone else.
- **Fragment**: the part of the owner backup that one feature contributes (for example the names fragment from spec 001).
- **Recovery key**: a one-time secret shown to the owner when the store is created, which they print or keep offline, and which can reopen the store if the normal key is lost.

## Clarifications

### Session 2026-11-19

- Q: Is the owner backup readable plain text or protected by a passphrase? → A: Plain, open, documented file; Meep warns the owner when making it that anyone who has it can read everything (FOUND-02).
- Q: Should the foundation guard against other user accounts on the same computer? → A: No. v1.0 assumes the computer is used by one person and says so (FOUND-02).
- Q: Can the owner replace the recovery key after setup? → A: No. It is shown once and cannot be replaced in v1.0; the owner backup is the protection (FOUND-02; R7 revisit before v1.1).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - My information is kept in an encrypted store only I can open (Priority: P1)

The first time Meep runs on the owner's machine, it creates an encrypted store and shows a one-time recovery key. From then on the store opens automatically for the owner on their own machine and cannot be opened without a key (R2, R7).

**Why this priority**: every other feature depends on a safe place to keep information. Without it nothing else can be trusted.

**Independent Test**: create a store, close it, then try to open it without the key, with a wrong key, with the recovery key, and with the normal key; only the last two succeed.

**Acceptance Scenarios**:

1. **Given** a machine where Meep has never run, **When** the owner starts it for the first time, **Then** an encrypted store is created and a recovery key is shown once, with a clear instruction to print it or keep it offline (R7).
2. **Given** an existing store, **When** it is opened with no key or a wrong key, **Then** it refuses to open and reveals nothing about its contents (R2).
3. **Given** an existing store and the owner's normal key, **When** Meep starts, **Then** the store opens without asking the owner to type anything (R5, R7).
4. **Given** the normal key is no longer available, **When** the owner supplies the recovery key, **Then** the store opens and the owner is told how to set up normal opening again (R7).
5. **Given** the normal key and the recovery key are both lost, **When** the owner tries to open the store, **Then** it stays closed and Meep says plainly that the data cannot be recovered and that a backup is the only way to restore it; there is no backdoor (R7).
6. **Given** a copy of the store file taken to another machine, **When** someone opens it without the key, **Then** it cannot be read (R2).

---

### User Story 2 - Every record carries the same standard envelope (Priority: P1)

Any feature that saves a record gets the same envelope fields automatically, and every record lives at a valid address (R6, R9, R16).

**Why this priority**: a common envelope lets every feature, and later the peer-to-peer and export features, treat records the same way.

**Independent Test**: save a synthetic record and confirm every envelope field is present and correct; save one at an invalid address and confirm it is refused.

**Acceptance Scenarios**:

1. **Given** a feature saves a record, **When** it is stored, **Then** the record has an identifier, the owner's account ID, an address, a provenance, the statement that Meep claims no authority over it, and created and last-changed timestamps (R9).
2. **Given** a feature saves a record, **When** it is read back, **Then** its envelope is exactly what was written and its content is unchanged.
3. **Given** the address `Alpha.Meep.Personal.Identity.Names`, **When** it is checked, **Then** it is accepted: exactly five dot-separated segments, each starting with a letter and containing only letters, digits and hyphens (R16).
4. **Given** addresses with four or six segments, an empty segment, a segment starting with a digit, a space, or an underscore, **When** they are checked, **Then** each is refused with a message saying why (R16).
5. **Given** two addresses that differ only in letter case, **When** they are compared, **Then** they are treated as the same address, while the case as written is kept for display (R16).
6. **Given** the account ID, **When** records are created over time, **Then** every record carries the same random account ID, and the display label "Alpha" is not used as an identifier (R9).

---

### User Story 3 - Deleting a record destroys it for good (Priority: P1)

When a record is deleted, its content and the key that protects it are destroyed, so it cannot be read again, while the history records only that a record was deleted and the date (R8, principle 26 integrity).

**Why this priority**: it is the promise that "delete means gone" for every feature built on the foundation.

**Independent Test**: save a synthetic record, delete it, then search all of Meep's storage for its content and key and confirm nothing is found.

**Acceptance Scenarios**:

1. **Given** a saved record, **When** the owner deletes it, **Then** its content can no longer be read by any feature (R8).
2. **Given** a deleted record, **When** the store, its history working files and its freed space are searched, **Then** neither the record's content nor its key is found anywhere in what Meep manages (R8, spec Risks table).
3. **Given** a deleted record, **When** the change log is read, **Then** it shows only that a record was deleted and the date, never the content (R8).
4. **Given** a feature needs a purpose-specific protected value for a record (for example a tamper-evident fingerprint of a prior value for corrections), **When** it asks for one, **Then** the foundation provides a value tied to that record's own key and that purpose, which becomes meaningless once the record's key is destroyed (NAMES-02, FOUND-01).
5. **Given** backups or exports made before the deletion, **When** the record is deleted, **Then** the foundation does not claim to change them, and the interface and documentation say so plainly (R8, spec 001 FR-033).

---

### User Story 4 - Every change is recorded in a tamper-evident history (Priority: P2)

Each time a feature adds, changes or deletes a record, an entry is appended to an append-only history in which each entry is cryptographically linked to the one before it, so any alteration or removal is detectable (spec v1.0 foundation, principle 16, FOUND-01).

**Why this priority**: it gives the owner an audit trail they can trust without storing any record's content in it.

**Independent Test**: append synthetic entries, verify the chain, then alter, remove or reorder one entry and confirm verification fails and points at the problem.

**Acceptance Scenarios**:

1. **Given** a feature performs an action on a record, **When** it appends an entry, **Then** the entry is added at the end with its time, the record it concerns, the kind of event and only the details the feature is allowed to log (FOUND-01).
2. **Given** a log of several entries, **When** it is verified, **Then** verification passes and reports the number of entries checked.
3. **Given** a log where one entry was edited, removed or moved, **When** it is verified, **Then** verification fails and says which entry first breaks the chain.
4. **Given** any attempt by a feature to change or delete an earlier entry, **When** it is made, **Then** it is refused; entries can only be added at the end (spec v1.0 foundation).
5. **Given** a feature tries to put a record's content or key into an entry, **When** it appends, **Then** the entry is refused (FOUND-01, R8).
6. **Given** the entry format, **When** a new feature appends its own kind of event, **Then** the format is the same for every feature; each feature defines only the details of its own payload (FOUND-01).

---

### User Story 5 - I can take a full backup and restore it (Priority: P2)

The owner can export everything Meep holds as an owner backup in an open, documented format, and restore it later. Each feature contributes its own fragment to the same backup (principle 27, FOUND-01).

**Why this priority**: it is the owner's protection against losing the key and the proof that there is no lock-in.

**Independent Test**: save synthetic records from two features, make an owner backup, restore it into an empty store, and confirm every record matches; then try restoring into a non-empty store and a damaged backup.

**Acceptance Scenarios**:

1. **Given** records from several features, **When** the owner makes an owner backup, **Then** it contains every record, each feature's fragment, and enough standard information to be read without Meep running (principle 27).
2. **Given** an owner backup and an empty store, **When** it is restored, **Then** every record is back exactly as it was and re-exporting gives an equal backup (spec v1.0 unit tests).
3. **Given** an owner backup and a store that already holds records, **When** the owner tries to restore, **Then** Meep warns and requires an explicit "Replace everything" choice before changing anything (NAMES-05).
4. **Given** a backup that breaks a rule (for example a record at an invalid address, or content a feature refuses), **When** it is restored, **Then** the whole restore is refused with a message listing every problem, and nothing is partly restored (NAMES-05).
5. **Given** a restore that fails partway for any reason, **When** it ends, **Then** the store is exactly as it was before and a clear message says so (NAMES-05, all-or-nothing).
6. **Given** a backup, **When** a feature's fragment is present but that feature is not installed, **Then** the restore says which fragment cannot be read and does not drop it silently.
7. **Given** the owner makes an owner backup, **When** the file is created, **Then** it is a plain, open, documented file that anyone who has it can read, and Meep warns the owner of this at that moment and suggests keeping it somewhere safe (owner decision FOUND-02).

---

### User Story 6 - Meep is reachable only from my own machine (Priority: P2)

In v1.0 there is no sign-in. Meep is therefore reachable only from the owner's own machine, and other web pages the owner visits cannot send it requests (R5).

**Why this priority**: with no login, location and request protection are the only barriers.

**Independent Test**: try to reach Meep from another machine and from a web page on another site; both must fail, while the owner's own use works.

**Acceptance Scenarios**:

1. **Given** Meep is running, **When** a request arrives from another machine on the network, **Then** it cannot connect (R5).
2. **Given** the owner has Meep open and visits another web page, **When** that page tries to send Meep a request that would read or change data, **Then** the request is refused (R5).
3. **Given** a request that claims a name or origin that is not the owner's local one, **When** it arrives, **Then** it is refused (R5).
4. **Given** the owner's own use on their own machine, **When** they open Meep, **Then** it works with no sign-in (R5).
5. **Given** a computer with other user accounts, **When** one of them is considered, **Then** v1.0 does not guard against it: Meep assumes the computer is used by one person, and says so in its documentation (owner decision FOUND-02).

---

### Edge Cases

- The store file is damaged or truncated: Meep refuses to open it, says so, and points the owner to the owner backup (principle 16, R7).
- Disk full or the machine is interrupted during any save, delete, append or restore: nothing changes and a clear message says so (all-or-nothing; spec 001 FR-030).
- Two features append to the log at the same moment: both entries are added in a single, consistent order and the chain stays valid.
- The change-log entry for a deleted record is kept, but it never contains the record's content or key (R8).
- A record's address is changed to an invalid one by a feature: the save is refused (R16).
- The clock is wrong or goes backwards: entries keep their order in the chain regardless of the timestamp; the timestamp is informational.
- A second copy of Meep is started on the same store: only one may open it at a time; the second is refused with a message [ASSUMPTION: one at a time, since there is a single owner and no sync in v1.0].
- A key is in use when a record is deleted: the delete still destroys that record's key and leaves other records untouched.
- A backup is made while records are being saved: the backup is a consistent snapshot of one moment.
- The recovery key is shown only once and cannot be replaced after setup in v1.0: if the owner dismisses it unread or loses the printed copy, Meep makes clear that the owner backup is the way to protect against losing the normal key as well (R7, owner decision FOUND-02; the v1.1 revisit of key custody may reopen this).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST keep all records and the change log in an encrypted store that cannot be read without a key, including when a copy of the store file is taken elsewhere (R2, principle 16).
- **FR-002**: On first run the system MUST create the store and show a one-time recovery key, instructing the owner to print or keep it offline, and MUST NOT keep the recovery key readable anywhere in Meep afterwards (R7).
- **FR-003**: The store MUST open without the owner typing anything while the normal key is available on the owner's own machine, and MUST refuse to open with no key or a wrong key (R2, R5, R7).
- **FR-004**: The system MUST allow the recovery key to open the store when the normal key is lost, and MUST state plainly that when both keys are lost the data cannot be recovered, with no backdoor and no master override (R7).
- **FR-005**: The system MUST protect the store key using the owner's own computer, so it stays on the owner's machine and is never sent anywhere (R7, EGRESS-01).
- **FR-006**: Every record MUST carry the envelope: identifier, account ID, address, provenance, the statement `authority: none`, and created and last-changed timestamps (R9, spec v1.0 foundation).
- **FR-007**: The account ID MUST be a random value, the same for every record of the owner, and the display label "Alpha" MUST NOT be used as an identifier (R9).
- **FR-008**: The system MUST provide one address checker shared by all features that accepts exactly five dot-separated segments, each starting with a letter and containing only letters, digits and hyphens, compares addresses without regard to letter case, keeps the case as written for display, and refuses anything else with a message saying why (R6, R16).
- **FR-009**: The system MUST protect each record's content with a key that belongs to that record alone, so one record can be destroyed without touching any other (R8).
- **FR-010**: When a record is deleted, the system MUST destroy its content and its key so neither can be read, and MUST ensure no copy of either remains anywhere Meep manages, including the store, its history working files and freed space (R8, spec Risks table).
- **FR-011**: When a record is deleted, the change log MUST show only that a record was deleted and the date, never its content or key (R8).
- **FR-012**: The system MUST let a feature ask for a protected value tied to one record's own key and a named purpose (for example a tamper-evident fingerprint of a prior value for corrections); the value MUST be non-reversible and MUST become meaningless when the record's key is destroyed (NAMES-02, FOUND-01).
- **FR-013**: The system MUST keep an append-only change log in which each entry is cryptographically linked to the one before it, so alteration, removal or reordering of any entry is detectable; verification MUST report the first entry that breaks the chain (spec v1.0 foundation, principle 16).
- **FR-014**: The system MUST use one change-log entry format for every feature, containing the time, the record concerned, the kind of event and the feature-defined details, and MUST refuse any attempt to change or delete an earlier entry or to put a record's content or key into an entry (FOUND-01, R8).
- **FR-015**: The system MUST make each save, correction, deletion, log append and restore all-or-nothing: on failure or interruption nothing changes and a clear message says so (NAMES-05, principle 26 integrity).
- **FR-016**: The system MUST let the owner make an owner backup of everything Meep holds in an open, documented format that can be read without Meep running, with an envelope that each feature contributes its fragment to (principle 27, FOUND-01).
- **FR-017**: A restore MUST work only into an empty store unless the owner explicitly chooses "Replace everything", and MUST refuse the whole restore, listing every problem, when the backup breaks any rule; nothing is partly restored (NAMES-05).
- **FR-018**: Restoring then re-exporting MUST give an equal backup, and the owner backup MUST be a consistent snapshot of one moment (spec v1.0 unit tests).
- **FR-019**: A backup fragment that belongs to a feature that is not available MUST be reported by name and MUST NOT be dropped silently.
- **FR-020**: The system MUST be reachable only from the owner's own machine, MUST require no sign-in in v1.0, and MUST refuse requests from other machines (R5).
- **FR-021**: The system MUST refuse requests from other web pages that would read or change data and requests that claim a name or origin that is not the owner's local one (R5).
- **FR-022**: The system does not guard against other user accounts on the same computer in v1.0; it assumes one person uses the computer and MUST say so in its documentation (owner decision FOUND-02).
- **FR-023**: The owner backup MUST be a plain, open, documented file, and the system MUST warn the owner when making one that anyone who has the file can read everything in it and suggest keeping it somewhere safe (owner decision FOUND-02; principle 27).
- **FR-024**: The system MUST allow only one running copy to open a store at a time and refuse a second with a clear message (assumption, single owner, no sync in v1.0).
- **FR-025**: The system MUST provide to features, as capabilities and not as implementation, a record store (save, read, delete records with their envelope), a change log (append, verify) and key custody (obtain a record key, obtain a purpose-specific value, destroy a record key), so features never touch encryption or storage directly (FOUND-01).
- **FR-026**: The system MUST NOT include names, shared UI elements, distribution and updates, login, delegates, caregivers, insurance, any AI or cloud feature, or any v1.1 or later item (spec Out of scope).
- **FR-027**: The system MUST make no network request to any outside service of any kind (EGRESS-01, principle 12).

### Key Entities

- **Record**: one stored piece of information with its envelope and its content; the content is protected by its own key.
- **Envelope**: identifier, account ID, address, provenance, `authority: none`, created and last-changed timestamps.
- **Record key**: the secret that protects one record's content; destroying it destroys the record.
- **Store key and recovery key**: the normal key kept on the owner's own computer, and the one-time recovery key the owner keeps offline.
- **Change-log entry**: time, record concerned, kind of event, feature-defined details, and the link to the previous entry; never any content.
- **Owner backup and fragment**: the full export for the owner, built from one fragment per feature.
- **Test fixtures (synthetic)**: two sample records at the addresses `Alpha.Meep.Personal.Identity.Names` and `Alpha.Meep.Personal.Notes.Sample` with made-up content, a made-up account ID, and invalid addresses (four segments, a segment starting with a digit, an underscore). No real personal data and no email addresses anywhere.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of attempts, a store cannot be opened or read with no key or a wrong key, and opens with the normal key or the recovery key (R2, R7).
- **SC-002**: After a record is deleted, a search of everything Meep manages finds neither its content nor its key in 100% of tested cases (R8).
- **SC-003**: In 100% of tests that alter, remove or reorder a change-log entry, verification fails and names the first broken entry (spec v1.0 foundation).
- **SC-004**: An owner backup restored into an empty store reproduces every record exactly, and re-exporting gives an equal backup, for every synthetic fixture (principle 27).
- **SC-005**: In 100% of tested failures or interruptions during save, delete, append or restore, the store is unchanged afterwards (NAMES-05).
- **SC-006**: In 100% of tests, requests from another machine and from another web page are refused, while the owner's own use works with no sign-in (R5).
- **SC-007**: A new owner can reach the point of having saved their recovery key in under five minutes from first launch [ASSUMPTION: measured with the owner as the test subject].
- **SC-008**: In 100% of address-checker tests, every valid and invalid address in the fixtures is accepted or refused as specified, with a reason for each refusal (R16).
- **SC-009**: No record, log entry or backup in the fixtures contains real personal data or an email address (constitution).

## Assumptions

- A single owner uses Meep on their own computer, and the computer's protected key storage is available (R5, R7).
- Which encryption, storage and key-protection technology is used is decided in planning; the planning choices PLAN-02 to PLAN-05 are Proposed and not binding here.
- The change log is never shortened or compacted in v1.0; how large it may grow is a planning matter (DATA-01 is Open).
- The owner backup format is open and documented; its exact field names are fixed at implementation.
- The shared address syntax is R16; the first address is `Alpha.Meep.Personal.Identity.Names` for names (R6).
- Spec 001 (names), spec 010 (shared UI) and spec 015 (distribution and updates) are separate; AT-35 and AT-36 (unsigned update refused, failed migration restores a snapshot) belong to spec 015 and are only referenced here.
- The v1.1 revisit of key custody (a recovery drill from the printed key and a hardware key as a second unlock) is outside v1.0 (R7).
- Whether Meep is run by each owner on their own machine is the HOST-01 default and is assumed here; other operators are out of scope.
- A single person uses the computer; other user accounts on it are not guarded against in v1.0 (FOUND-02).
- The recovery key is shown once and cannot be replaced after setup in v1.0 (FOUND-02, R7).
- The owner backup is a plain, open file; the owner is warned when making it (FOUND-02).

## Out of Scope (v1.0)

Names and pronouns (spec 001); shared UI elements, theme and screens (spec 010); distribution, signing, updates and snapshots before migration (spec 015); sign-in, SSO, 2FA, delegates and caregivers; insurance; any AI or cloud feature; peer-to-peer; optional cloud backup (v2); anything labeled v1.1 or later.
