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
- **Owner backup**: the owner's own full export of everything Meep holds, made for the owner and never for anyone else. It is one zip archive holding a plain-language read-me (READ-ME-FIRST.txt), the data, and a machine-readable schema of the data format, and it carries a format version.
- **Rescue backup**: an owner backup made from a damaged store; it holds only records that read and validate fully, is labeled as coming from a damaged store, lists what could not be read, and carries the old change log only as a separate, clearly unverified archive.
- **Fragment**: the part of the owner backup that one feature contributes (for example the names fragment from spec 001).
- **Recovery key**: a one-time secret shown to the owner when the store is created, which they print or keep offline, and which can reopen the store if the normal key is lost.

## Clarifications

### Session 2026-11-19

- Q: Is the owner backup readable plain text or protected by a passphrase? → A: Plain, open, documented file; Meep warns the owner when making it that anyone who has it can read everything (FOUND-02).
- Q: Should the foundation guard against other user accounts on the same computer? → A: No. v1.0 assumes the computer is used by one person and says so (FOUND-02).
- Q: If the store is partly damaged or the change log fails its check, what should Meep do? → A: Open it read-only with a clear warning and offer only an owner backup (a rescue backup) of what can be read; refuse a store that cannot be read at all, pointing to the latest owner backup. Meep never modifies, repairs or deletes the damaged file. The rescue backup holds only records that read and validate fully, is labeled and lists what could not be read, and keeps the old log only as an unverified archive; the new store starts a fresh chain. Restoring adopts the backup's account ID and issues a new recovery key (FOUND-03).
- Q: If Meep stops unexpectedly while the store is open, how does the next start deal with the one-copy-at-a-time rule? → A: The rule is enforced by a claim the operating system holds for the running Meep and releases automatically when it stops, including after a crash or power loss, so there is no leftover marker whose owner has to be guessed. After an unexpected stop the next start checks that the store is intact and the change log verifies before opening normally; if either fails, damaged-store handling applies (FOUND-03). If Meep cannot tell whether another copy holds the store, it does not ask the owner to confirm; it refuses with the message "Meep may already be open. If it isn't, restart your computer and try again." "Copy" means a running Meep program, not a browser tab or window; several tabs on one running Meep are allowed (FOUND-04).
- Q: Should every owner backup carry its own description of its format? → A: Yes. An owner backup is one zip archive containing READ-ME-FIRST.txt (plain text, plain language), the data, and a machine-readable schema. The read-me is complete on its own: what Meep is, what each part holds, how fields map to the standard name formats, how to restore, that the file is not encrypted (FOUND-02), and the format version. A rescue backup's read-me also lists what could not be read and explains the unverified history archive (FOUND-03). The format carries a version; a Meep that meets a newer incompatible version refuses with a plain message and does not guess. The same full description is published in the project documentation, and a check on every change confirms the two match (FOUND-05).
- Q: While an owner backup is being made, should changes wait or carry on? → A: Everything carries on as normal; the backup shows the store as of the moment it started and says so in the read-me and on screen. Changes made during a backup are saved to the store normally and a backup never delays or holds them. When a backup finishes, if any change was made after its "as of" moment, Meep says "Changes you made after <time> aren't in this backup" with a "Make another backup" action. A backup is written under a temporary name, fully verified (archive readable, every record and the read-me present), and only then given its final name; an interrupted or failed backup leaves no file that looks like a complete backup, and Meep says plainly that it failed. Settings shows "Last successful backup: <date and time>" (FOUND-06).
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
7. **Given** a store that is partly damaged or whose change log fails its check but can still be read in part, **When** the owner starts Meep, **Then** it opens read-only with a clear warning, offers only an owner backup of what can be read, and does not change the store file in any way (FOUND-03).
8. **Given** a store that cannot be read at all, **When** the owner starts Meep, **Then** it refuses to open, changes nothing, and points to the latest owner backup (FOUND-03).
9. **Given** Meep stopped unexpectedly (a crash or power loss) while the store was open, **When** the owner starts Meep again, **Then** the earlier copy no longer holds the store, and Meep checks that the store is intact and the change log verifies before opening normally (FOUND-04).
10. **Given** that check fails, **When** Meep starts, **Then** damaged-store handling applies: read-only with a warning, and a rescue backup is offered (FOUND-04, FOUND-03).
11. **Given** one Meep program is running and holds the store, **When** a second Meep program is started, **Then** it is refused with a plain message and the store is untouched (FOUND-04).
12. **Given** Meep cannot determine whether another copy holds the store, **When** it starts, **Then** it does not ask the owner to confirm anything; it refuses and says "Meep may already be open. If it isn't, restart your computer and try again." (FOUND-04)
13. **Given** one running Meep, **When** the owner has it open in several browser tabs or windows, **Then** all of them work; a stale save from a second tab shows the stale-save warning from spec 001, never a silent overwrite (FOUND-04, NAMES-05).

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
7. **Given** a chain check that fails, **When** Meep tells the owner, **Then** the message says the history does not verify, names the first entry that breaks the chain, and says the likely causes are a software fault or editing the store outside Meep; it never implies an intruder (FOUND-03).
8. **Given** data that cannot be read, **When** Meep tells the owner, **Then** the message says some data could not be read, which is different from the history not verifying (FOUND-03).

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
8. **Given** a partly damaged store opened read-only, **When** the owner makes a rescue backup, **Then** it contains only records that read and validate fully, is labeled as coming from a damaged store, lists what could not be read, and always restores cleanly (FOUND-03).
9. **Given** a rescue backup, **When** it is imported, **Then** the warning that it came from a damaged store and the list of what could not be read are shown again (FOUND-03).
10. **Given** a rescue backup, **When** it is restored, **Then** the old change log is not restored as live history; it is kept as a separate, clearly unverified archive, and the new store starts a fresh chain whose first entry records "restored from a rescue backup of a damaged store" with the date (FOUND-03).
11. **Given** a backup restored into a new store, **When** the restore finishes, **Then** the new store uses the backup's account ID (R9), a new recovery key is shown, and Meep tells the owner plainly that the old recovery key does not open the new store (FOUND-03, R7).
12. **Given** the owner makes an owner backup, **When** the file is created, **Then** it is one zip archive containing READ-ME-FIRST.txt, the data and a machine-readable schema, and it carries a format version (FOUND-05).
13. **Given** an owner backup, **When** someone with no access to Meep or the project's website opens READ-ME-FIRST.txt, **Then** it is complete on its own: it says what Meep is, what each part holds, how fields map to the standard name formats, how to restore, that the file is not encrypted, and the format version (FOUND-05, FOUND-02).
14. **Given** a rescue backup, **When** its read-me is opened, **Then** it also lists what could not be read and explains that the old history is an unverified archive (FOUND-05, FOUND-03).
15. **Given** a backup whose format version is newer than and incompatible with the Meep reading it, **When** the owner tries to restore it, **Then** Meep refuses with a plain message and does not guess (FOUND-05).
16. **Given** the owner makes an owner backup while changes are being saved, **When** the backup finishes, **Then** every change was saved to the store normally and none was delayed or held by the backup, and the backup shows the store as of the moment it started, stated in the read-me and on screen (FOUND-06, FR-015).
17. **Given** a change was made after the backup's "as of" moment, **When** the backup finishes, **Then** Meep says "Changes you made after <time> aren't in this backup" and offers "Make another backup" (FOUND-06).
18. **Given** a backup is interrupted or fails at any point, **When** the owner looks at the backup location, **Then** no file there looks like a complete backup, and Meep has said plainly that the backup failed (FOUND-06).
19. **Given** at least one successful backup, **When** the owner opens Settings, **Then** it shows "Last successful backup: <date and time>" so the owner knows what an emergency restore would lose (FOUND-06).

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

- The store file is damaged, truncated, or its change log fails its check: Meep opens it read-only with a clear warning and offers only a rescue backup of what can be read; a store that cannot be read at all is refused and the owner is pointed to the latest owner backup. Meep never modifies, repairs or deletes the damaged file, including any changes still waiting to be written (principle 16, R7, FOUND-03).
- Disk full or the machine is interrupted during any save, delete, append or restore: nothing changes and a clear message says so (all-or-nothing; spec 001 FR-030).
- Two features append to the log at the same moment: both entries are added in a single, consistent order and the chain stays valid.
- The change-log entry for a deleted record is kept, but it never contains the record's content or key (R8).
- A record's address is changed to an invalid one by a feature: the save is refused (R16).
- The clock is wrong or goes backwards: entries keep their order in the chain regardless of the timestamp; the timestamp is informational.
- A second running Meep program is started on the same store: it is refused with a plain message; several browser tabs or windows on one running Meep are allowed (FOUND-04).
- Meep stopped unexpectedly: the claim on the store is released by the operating system with no leftover marker; the next start checks the store and the change log before opening normally (FOUND-04).
- A key is in use when a record is deleted: the delete still destroys that record's key and leaves other records untouched.
- A backup is made while records are being saved: the changes are saved normally, never delayed; the backup is a consistent snapshot as of the moment it started, and Meep says so (FOUND-06).
- A damaged-store session is interrupted by a crash: the private working copy is removed at the next start and the original is untouched (FR-045, FOUND-03).
- A backup is interrupted by a crash or failure: it leaves either a verified backup or no file that looks like a complete backup, and changes saved during it survive (FOUND-06).
- The recovery key is shown only once and cannot be replaced after setup in v1.0: if the owner dismisses it unread or loses the printed copy, Meep makes clear that the owner backup is the way to protect against losing the normal key as well (R7, owner decision FOUND-02; the v1.1 revisit of key custody may reopen this).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST keep all records and the change log in an encrypted store that cannot be read without a key, including when a copy of the store file is taken elsewhere (R2, principle 16). What an observer of the store file can still learn is limited to its size, its timestamps and the existence of write-ahead-log, journal and backup files; the size roughly tracks the amount of data, and v1.0 does not pad the file. Contents and record counts are not readable.
- **FR-002**: On first run the system MUST create the store and show a one-time recovery key, instructing the owner to print or keep it offline, and MUST NOT keep the recovery key readable anywhere in Meep afterwards (R7).
- **FR-003**: The store MUST open without the owner typing anything while the normal key is available on the owner's own machine, and MUST refuse to open with no key or a wrong key (R2, R5, R7).
- **FR-004**: The system MUST allow the recovery key to open the store when the normal key is lost, and MUST state plainly that when both keys are lost the data cannot be recovered, with no backdoor and no master override (R7). Losing the normal key for any reason, including a change of operating-system account, a reinstall or a keychain reset, is the recovery-key case. After the recovery key opens the store, Meep MUST store a fresh normal key and say so. There MUST be no support tool, debug switch or environment override that opens a store without a key.
- **FR-005**: The system MUST protect the store key using the owner's own computer, so it stays on the owner's machine and is never sent anywhere (R7, EGRESS-01).
- **FR-006**: Every record MUST carry the envelope: identifier, account ID, address, provenance, the statement `authority: none`, and created and last-changed timestamps (R9, spec v1.0 foundation).
- **FR-007**: The account ID MUST be a cryptographically secure random value, the same for every record of the owner, and the display label "Alpha" MUST NOT be used as an identifier (R9).
- **FR-008**: The system MUST provide one address checker shared by all features that accepts exactly five dot-separated segments, each starting with a letter and containing only letters, digits and hyphens, compares addresses without regard to letter case, keeps the case as written for display, and refuses anything else with a message saying why (R6, R16).
- **FR-009**: The system MUST protect each record's content with a key that belongs to that record alone, so one record can be destroyed without touching any other (R8).
- **FR-010**: When a record is deleted, the system MUST destroy its content and its key so neither can be read, and MUST ensure no copy of either remains anywhere Meep manages, including the store, its history working files and freed space (R8, spec Risks table). "Cannot be read" means by features, by the owner and by anyone who later holds the store key. Backups made before the deletion still contain the record and its key until those backups are deleted, and the delete confirmation MUST say so.
- **FR-011**: When a record is deleted, the change log MUST show only that a record was deleted and the date, never its content or key (R8).
- **FR-012**: The system MUST let a feature ask for a protected value tied to one record's own key and a named purpose (for example a tamper-evident fingerprint of a prior value for corrections); the value MUST be non-reversible and MUST become meaningless when the record's key is destroyed (NAMES-02, FOUND-01).
- **FR-013**: The system MUST keep an append-only change log in which each entry is cryptographically linked to the one before it, so alteration, removal or reordering of any entry is detectable; verification MUST report the first entry that breaks the chain (spec v1.0 foundation, principle 16). The log grows with use, is never shortened and has no size limit in v1.0.
- **FR-014**: The system MUST use one change-log entry format for every feature, containing the time, the record concerned (by record alias, FR-048), the kind of event and the feature-defined details, and MUST refuse any attempt to change or delete an earlier entry or to put a record's content or key into an entry (FOUND-01, R8).
- **FR-015**: The system MUST make each save, correction, deletion, log append and restore all-or-nothing: on failure or interruption nothing changes and a clear message says so (NAMES-05, principle 26 integrity).
- **FR-016**: The system MUST let the owner make an owner backup of everything Meep holds in an open, documented format that can be read without Meep running, with an envelope that each feature contributes its fragment to (principle 27, FOUND-01).
- **FR-017**: A restore MUST work only into an empty store unless the owner explicitly chooses "Replace everything", and MUST refuse the whole restore, listing every problem, when the backup breaks any rule; nothing is partly restored (NAMES-05). "Empty" means no records and no log entries other than the first. The list of problems shows up to 100, grouped by feature, with a count of the rest. "Replace everything" MUST use the NAMES-07 confirmation (typed confirmation, an explicit warning that the loss cannot be undone, and an offer to make a backup first); it destroys the old records and keys, discards the old log and replaces the account ID, and nothing is recoverable afterwards.
- **FR-018**: Restoring then re-exporting MUST give an equal backup, and the owner backup MUST be a consistent snapshot of one moment (spec v1.0 unit tests).
- **FR-019**: A backup fragment that belongs to a feature that is not available MUST be reported by name and MUST NOT be dropped silently.
- **FR-020**: The system MUST be reachable only from the owner's own machine, MUST require no sign-in in v1.0, and MUST refuse requests from other machines (R5).
- **FR-021**: The system MUST refuse requests from other web pages that would read or change data and requests that claim a name or origin that is not the owner's local one (R5); concretely (SEC-05): a Host allow-list (`127.0.0.1:<port>`, `localhost:<port>`), Origin and `Sec-Fetch-Site` checks and a per-session token on every state-changing request, `SameSite=Strict` cookies, no CORS headers, and a content security policy of `default-src 'self'` with no inline script.
- **FR-022**: The system does not guard against other user accounts on the same computer in v1.0; it assumes one person uses the computer and MUST say so in its documentation (owner decision FOUND-02). The per-launch token of FR-052 is not a sign-in; the documentation MUST state that code running as the same operating-system user can still reach Meep.
- **FR-023**: The owner backup MUST be a plain, open, documented file, and the system MUST warn the owner when making one that anyone who has the file can read everything in it and suggest keeping it somewhere safe (owner decision FOUND-02; principle 27).
- **FR-024**: The system MUST allow only one running Meep program to hold a store at a time, enforced by a claim the operating system holds for the running program and releases automatically when it stops, including after a crash or power loss, with no marker file whose owner has to be guessed. "Copy" means a running Meep program, not a browser tab or window; several tabs on one running Meep are allowed (concurrent edits follow the stale-save warning, NAMES-05). A second program MUST be refused with a plain message (FOUND-04). A running Meep program means any program that has opened the store through the foundation's lock.
- **FR-025**: The system MUST provide to features, as capabilities and not as implementation, a record store (save, read, delete records with their envelope, each write accepting the calling feature's events, which the foundation validates and logs in the same transaction, FOUND-08), a change log (append, verify) and key custody (obtain a record key, obtain a purpose-specific value, destroy a record key), so features never touch encryption or storage directly (FOUND-01).
- **FR-026**: The system MUST NOT include names, shared UI elements, distribution and updates, login, delegates, caregivers, insurance, any AI or cloud feature, or any v1.1 or later item (spec Out of scope).
- **FR-027**: The system MUST make no network request to any outside service of any kind (EGRESS-01, principle 12).
- **FR-028**: When the store is partly damaged or its change log fails its check but can be read in part, the system MUST open it read-only with a clear warning and offer only a rescue backup of what can be read; when the store cannot be read at all, it MUST refuse to open and point to the latest owner backup (FOUND-03). In this read-only mode the owner can view records and make the rescue backup, and nothing else; if only the change log fails, the store stays read-only until a rescue restore; from a read-only damaged store the only backup is the rescue backup.
- **FR-029**: The system MUST NOT modify, repair or delete a damaged store file in any way, including any changes still waiting to be written; the plan MUST specify how the store is opened without altering it (FOUND-03).
- **FR-030**: A rescue backup MUST contain only records that read and validate fully so it always restores, MUST be labeled as coming from a damaged store and list what could not be read, and importing it MUST repeat that warning (FOUND-03). Repeated damaged starts and several rescue backups are allowed with no limit, but a rescue backup is not created when one with an identical content hash already exists.
- **FR-031**: The old change log MUST NOT be restored as live history; it MUST be kept as a separate, clearly unverified archive inside the rescue backup, and the restored store MUST start a fresh chain whose first entry records "restored from a rescue backup of a damaged store" with the date (FOUND-03). v1.0 offers no viewer for it; it travels in later owner backups.
- **FR-032**: Restoring into a new store MUST adopt the backup's account ID (R9) and issue a new recovery key, and the system MUST tell the owner plainly that the old recovery key does not open the new store (FOUND-03, R7).
- **FR-033**: Messages MUST distinguish "some data could not be read" from "the history does not verify"; the second MUST name the first broken entry and give the likely causes as a software fault or editing the store outside Meep, without implying an intruder (FOUND-03).
- **FR-034**: After an unexpected stop, the next start MUST check that the store is intact and the change log verifies before opening normally, and if either fails MUST follow damaged-store handling (FOUND-03, FOUND-04).
- **FR-035**: If Meep cannot determine whether another copy holds the store, it MUST NOT ask the owner to confirm; it MUST refuse and say "Meep may already be open. If it isn't, restart your computer and try again." (FOUND-04). The refusal message MUST also be written to standard error and the program MUST exit with a non-zero code, so it is visible when there is no window.
- **FR-036**: An owner backup MUST be one zip archive containing READ-ME-FIRST.txt (plain text, plain language), the data, and a machine-readable schema of the data format, and MUST carry a format version (FOUND-05, principle 27).
- **FR-037**: The read-me MUST be complete on its own: what Meep is, what each part holds, how fields map to the standard name formats (each feature supplies the part for its own fragment), how to restore, that the file is not encrypted (FOUND-02), the "as of" moment of the backup, and the format version (FOUND-05, FOUND-06).
- **FR-038**: A rescue backup's read-me MUST also list what could not be read and explain the unverified history archive (FOUND-05, FOUND-03).
- **FR-039**: When Meep meets a backup with a newer incompatible format version it MUST refuse with a plain message and MUST NOT guess (FOUND-05). The manifest carries a format version for each fragment; a fragment with a newer major version is refused and one with an older version is read.
- **FR-040**: The same full description of the format MUST be published in the project documentation, and a check on every change MUST confirm that the description inside a backup and the published one match (FOUND-05, ENG-02).
- **FR-041**: Making an owner backup MUST NOT delay or hold any change: saves, corrections and deletions during a backup are saved to the store normally (FR-015), and the backup shows the store as of the moment it started, stated in its read-me and on screen (FOUND-06).
- **FR-042**: When a backup finishes, if any change was made after its "as of" moment, Meep MUST say "Changes you made after <time> aren't in this backup" and offer "Make another backup" (FOUND-06).
- **FR-043**: A backup MUST be written under a temporary name, fully verified (archive readable, every record and the read-me present), and only then given its final name; an interrupted or failed backup MUST leave no file that looks like a complete backup, and Meep MUST say plainly that it failed (FOUND-06).
- **FR-044**: The system MUST record the date and time of the last successful backup and make it available to be shown as "Last successful backup: <date and time>" in Settings (the Settings screen belongs to spec 010), so the owner always knows what an emergency restore would lose (FOUND-06). A restore and a backup made on another machine do not count; the time reads "none" until the first new backup is made.
- **FR-045**: All damaged-store work MUST happen on a private working copy that only the owner's account can read, that holds nothing but the encrypted store files, that is deleted when the session ends, and any copy left behind by an interrupted session MUST be deleted at the next start; working on the copy leaves the original byte-for-byte unchanged (FR-029, FOUND-03).
- **FR-046** (LOG-01): The system MUST keep two logs. The **History log** is the change log of FR-013 and FR-014, whose entries are fixed fields only: time, event kind, record alias, actor, outcome and registered details. The **Diagnostic log** is levelled, limited in size and rotated, kept only on this computer, with no telemetry (FR-027). Both are stored at the same protection level as the record store.
- **FR-047** (LOG-01): No log MUST ever contain a credential, key, the recovery key, a token or any record content. Data is identified only by a typed field reference, the record alias and a rule ID, plus these value facts and nothing else: grapheme count, byte count, script class, and whether normalization changed the text.
- **FR-048** (LOG-01): Every record MUST have a record alias that is random, assigned at creation, stored in the encrypted store and never derived from content, drawn from a curated character set so an alias never looks like a name. After the record is deleted its alias MUST NOT resolve to anything (R8).
- **FR-049** (LOG-01): The logging interface MUST accept only field references, record aliases, rule IDs, value-fact types and enumerations as data, never free text, so no log line can carry content or be forged by input; an owner-only debug view resolves aliases to values only when it is displayed, and exported debug text stays alias-only. Dismissals of notification-tray entries are recorded in the History log (UI-02, UI-03).
- **FR-050** (LOG-01): The test suite MUST seed unique canary strings in its fixtures, and the build MUST fail if any canary appears in any log output.
- **FR-051** (SEC-05): The storage layer MUST accept SQL only as fixed program text with bound parameters, templates MUST escape output by default, and the build MUST fail on any construction that would bypass either (a leaked string used as SQL, SQL calls outside the storage crate, `|safe` in a template). Fluent bidi isolation stays enabled.
- **FR-052** (CHK050, R5): Each launch MUST create a cryptographically random token. The token MUST NOT be passed on a command line or left in a URL. Meep MUST write a launch file readable only by the current operating-system user that redirects into Meep; the server MUST swap the token for a `SameSite=Strict`, `HttpOnly` cookie and redirect to a clean URL; a request without a valid cookie MUST be refused. The token MUST never be logged. This is not a sign-in (R5): it ties requests to this launch and does not identify a person.
- **FR-053** (CHK022): A stored value that decrypts but fails the current validation rules MUST be shown normally with a warning that names the rule ID, MUST be kept as stored and rendered safely (escaped and isolated), MUST NOT be hidden and MUST NOT block other edits; the current rules apply to a field only when the owner edits that field. A record that fails to decrypt or fails its integrity check is unread and is listed with the reason.

### Key Entities

- **Record**: one stored piece of information with its envelope and its content; the content is protected by its own key.
- **Envelope**: identifier, account ID, address, provenance, `authority: none`, created and last-changed timestamps.
- **Record key**: the secret that protects one record's content; destroying it destroys the record.
- **Store key and recovery key**: the normal key kept on the owner's own computer, and the one-time recovery key the owner keeps offline.
- **Change-log entry**: time, record concerned, kind of event, feature-defined details, and the link to the previous entry; never any content.
- **Rescue backup and archive**: the owner backup made from a damaged store, listing what could not be read, with the old change log kept only as a separate unverified archive.
- **Owner backup and fragment**: the full export for the owner, one zip archive with a plain-language read-me, the data and a machine-readable schema, built from one fragment per feature, with a format version.
- **Test fixtures (synthetic)**: two sample records at the addresses `Alpha.Meep.Personal.Identity.Names` and `Alpha.Meep.Personal.Notes.Sample` with made-up content, a made-up account ID, and invalid addresses (four segments, a segment starting with a digit, an underscore). No real personal data and no email addresses anywhere.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of attempts, a store cannot be opened or read with no key or a wrong key, and opens with the normal key or the recovery key (R2, R7).
- **SC-002**: After a record is deleted, a search of everything Meep manages finds neither its content nor its key in 100% of tested cases (R8).
- **SC-003**: In 100% of tests that alter, remove or reorder a change-log entry, verification fails and names the first broken entry (spec v1.0 foundation).
- **SC-004**: An owner backup restored into an empty store reproduces every record exactly, and re-exporting gives an equal backup, for every synthetic fixture (principle 27).
- **SC-005**: In 100% of tested failures or interruptions during save, delete, append or restore, the store is unchanged afterwards (NAMES-05).
- **SC-006**: In 100% of tests, requests from another machine and from another web page are refused, while the owner's own use works with no sign-in (R5).
- **SC-007**: A new owner can reach the point of having saved their recovery key in under five minutes from first launch, measured by the owner on a fresh install once for each release candidate; it passes if the owner gets there within five minutes with no help.
- **SC-008**: In 100% of address-checker tests, every valid and invalid address in the fixtures is accepted or refused as specified, with a reason for each refusal (R16).
- **SC-009**: No record, log entry or backup in the fixtures contains real personal data or an email address (constitution).
- **SC-010**: A rescue backup made from a synthetic partly damaged store restores cleanly in 100% of tested cases, and the restored store verifies (FOUND-03).
- **SC-011**: The old recovery key fails to open a store restored from a backup, and the new recovery key opens it, in 100% of tested cases (FOUND-03, R7).
- **SC-012**: Opening a damaged store leaves its file byte-for-byte identical in 100% of tested cases, including when changes were waiting to be written (FOUND-03).
- **SC-013**: After a simulated crash while the store is open, the next start opens the store in 100% of tested cases when it is intact, never needs a marker removed by hand, and refuses a truly second running program in 100% of tested cases (FOUND-04).
- **SC-014**: In 100% of tested backups the archive contains the read-me, the data and the schema; the read-me includes every required topic; a newer incompatible version is refused without guessing; and the check that the in-backup and published descriptions match passes on every change (FOUND-05).
- **SC-015**: In 100% of tested crashes at any point during a backup, the result is either a verified backup or no file that looks like a complete backup, and changes saved during the backup survive the crash (FOUND-06).
- **SC-016**: In 100% of tested backups made while changes are saved, no change is delayed, the backup matches the store as of its start, and the "changes after <time>" message appears exactly when a later change exists (FOUND-06).
- **SC-017**: After any damaged-store session, including one interrupted by a crash, no working copy remains once Meep has started again, in 100% of tested cases (FOUND-03, FR-045).

## Assumptions

- A single owner uses Meep on their own computer, and the computer's protected key storage is available (R5, R7).
- Which encryption, storage and key-protection technology is used is decided in planning; the planning choices PLAN-02 to PLAN-05 are Proposed and not binding here.
- The change log is never shortened or compacted in v1.0; it grows with use and has no size limit in v1.0 (DATA-01 stays Open for a later decision).
- The owner backup format is open and documented; its exact field names are fixed at implementation.
- The shared address syntax is R16; the first address is `Alpha.Meep.Personal.Identity.Names` for names (R6).
- Spec 001 (names), spec 010 (shared UI) and spec 015 (distribution and updates) are separate; AT-35 and AT-36 (unsigned update refused, failed migration restores a snapshot) belong to spec 015 and are only referenced here.
- The v1.1 revisit of key custody (a recovery drill from the printed key and a hardware key as a second unlock) is outside v1.0 (R7).
- Whether Meep is run by each owner on their own machine is the HOST-01 default and is assumed here; other operators are out of scope.
- A single person uses the computer; other user accounts on it are not guarded against in v1.0 (FOUND-02); the launch file of FR-052 is readable only by the current operating-system user, but code running as that user can still reach Meep.
- R5 (no sign-in, recorded in the Readiness table of [the master spec](../../docs/spec/Meep-Spec-v1.0_2026-11-20_0200-CT.md)) depends on HOST-01 and SEC-03 staying Open and on the R7 revisit before v1.1; the FR-052 token is not a sign-in and does not conflict with R5.
- The recovery key is shown once and cannot be replaced after setup in v1.0 (FOUND-02, R7).
- The owner backup is a plain, open file; the owner is warned when making it (FOUND-02).

- The plan specifies how a damaged store is opened without altering it, including changes not yet written (FOUND-03).

- Principle 26 (warn, never adjudicate) allows blocking only for data integrity and for authorization by location. The foundation's refusals are of those two kinds and are set by the owner: a damaged or failed-check store opens read-only (integrity, FOUND-03); a restore that breaks a rule is refused whole (integrity, FR-017); a second running copy is refused (integrity of the single store, FOUND-04); a request from another machine or web page is refused (location, R5). Nothing else is blocked.

## Out of Scope (v1.0)

- Screens for the recovery key, backup, restore and "Last successful backup" belong to spec 010; the foundation supplies the operations, messages and the recorded time (CHK053). Signing, updates and migration snapshots belong to spec 015; the foundation owns the format-version refusal (CHK054).

Names and pronouns (spec 001); shared UI elements, theme and screens (spec 010); distribution, signing, updates and snapshots before migration (spec 015); sign-in, SSO, 2FA, delegates and caregivers; insurance; any AI or cloud feature; peer-to-peer; optional cloud backup (v2); anything labeled v1.1 or later.
