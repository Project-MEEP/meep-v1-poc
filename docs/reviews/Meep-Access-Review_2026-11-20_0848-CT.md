# Meep access, identity and keys: design review

Drafted 2026-11-20 08:48 CT.

**Owner decisions, 2026-11-20 09:46 CT:**
- Confirmed as recommended: D1, D2 (option B: launch token in v1.0), D4, D5, D6, D9, D10, and the D11 proposals.
- D7: Confirmed for now.
- D8: Deferred.
- D3 and D11: decided as written.

Labels: **Fact** = checked in the repo or a standard. **Verify** = believed true, must be checked before it becomes binding. **Hypothesis** = reasoning, not measured.

## 1. What is already decided

| ID | What it says today | Source |
|---|---|---|
| R5 | v1.0 has no sign-in; Meep is reachable only from the owner's machine | Master spec Readiness table. **No `docs/decisions/R5.md` exists** |
| R7 | OS keychain holds the normal key; a one-time recovery key is printed or kept offline; no backdoor (SEC-03). Revisit before v1.1: recovery drill, hardware key as a second unlock | `docs/decisions/R7.md`. It does not say how the store key is derived or wrapped |
| FOUND-02 #2 | v1.0 does not protect against other OS accounts on the same computer | `docs/decisions/FOUND-02.md` |
| FOUND-02 #3 | Recovery key cannot be replaced after setup in v1.0 | same |
| FR-032 | Restoring into a new store issues a new recovery key | spec 002 |
| local-access.md | Loopback only. In-memory 256-bit session secret; any page load gets an HttpOnly SameSite=Strict cookie plus an anti-forgery token; Host and Origin checks | spec 002 contract |
| AUTH (whiteboard) | **SSO login with authenticator 2FA: Confirmed.** Not in v1.0 (R5); passkeys and recovery under SEC-03 | Master spec v1 features table |
| Principles 2, 4 | Every person owns their own data; the owner grants and revokes all delegation; an in-app role is not proof of legal POA | Constitution |
| ACCESS-01 | Emergency access to DNR and POA: later | Roadmap |

**Correction to my earlier answers:** SSO is not a new idea. It is a Confirmed whiteboard decision (the AUTH row), deferred past v1.0. My "AUTH-01" should refine that row, not create a parallel one.

## 2. Separate the four questions

Most access designs go wrong by letting one mechanism answer several of these questions. Meep should keep them apart.

| Layer | Question | Mechanism | Never does |
|---|---|---|---|
| Key custody | What can decrypt the store? | One random data key, wrapped separately by each "protector" (keychain, recovery key, later a passkey or hardware key) | Never depends on any online provider |
| Local access | Is this request coming from the person who launched Meep on this device? | Launch token, then session cookie (FR-052) | Not a login and doesn't identify anyone |
| Identity | Who is this person? | Nothing in v1.0. Later: SSO federation (AUTH) for the owner and for caregivers | Never holds or unwraps the store key |
| Authorization | What may this person do? | Owner-granted roles and delegation (spec 007) | A role is never proof of legal authority (principle 4) |

Rule that falls out of this: **the owner always has a path into their own data that involves no provider** (keychain or recovery key). SSO suspension, no internet, or Project-MEEP disappearing must never lock an owner out.

## 3. Who we are defending against

| Actor | v1.0 today | v1.0 with this review | Later |
|---|---|---|---|
| Malicious website in the same browser | Blocked (Host/Origin checks, SameSite cookie) | Same | Same |
| Another OS account on the same computer | **Not blocked.** Any page load hands out the cookie and anti-forgery token, so another account can just load the page (Fact, from local-access.md) | Blocked: the cookie is issued only to whoever holds the launch token (FR-052) | Same |
| Someone using the **same** OS account (a shared family login) | Not supported; documented | Not supported by design: the end user is 1:1 (D3) | Caregiver release: optional app lock (D3) |
| Other apps on an Android phone | n/a (no Android yet) | n/a | Launch token covers it, if the Tauri app talks to a local port (Hypothesis) |
| Thief with the powered-off device or a copied file | Encrypted (SQLCipher); keychain is tied to the OS user | Same | Hardware key optional (R7) |
| Caregiver exceeding their grant | n/a | n/a | Authorization layer (spec 007), audited (AT-05) |
| Identity provider suspends the account | n/a | n/a | Owner still gets in through keychain or recovery key |
| Coercion (someone forces the owner to unlock) | Not addressed | Not addressed | Open question (D10) |

**Scope rule (owner, 2026-11-20):** the primary end user is 1:1 with their OS account. A shared family login is outside v1's supported use. The docs say so and say what it exposes.

## 4. Decisions for the owner

Each has my recommendation. Reply by number.

**D1. Separate the layers as in section 2, and the owner always has a provider-free way in.** Recommend: Confirm as a foundation principle.

**D2. FR-052 launch token (v1.0).**
- The cookie and anti-forgery token are issued only through a one-time launch token. The token expires 60 seconds after launch and is delivered by a launch file only the current OS user can read. It is never put on a command line or left in a URL.
- This amends FOUND-02 #2.
- Recommend: Confirm for v1.0.
- Fallback: make it a hard gate before the first public alpha tag or v1.1.

**D3. App lock for shared-login households. Owner decision 2026-11-20: vNext.**
- The primary end user is always 1:1: one person, one OS account, one Meep store.
- Several people behind one account is supported only for a caregiver, and that is vNext (caregiver workspaces, CARE-03).
- v1 documents that anyone using the owner's OS account can see everything in Meep. That is the residual risk.
- The optional app lock (Windows Hello, PIN or biometrics through a passkey) moves to the caregiver release.
- Open for HOST-01: 1:1 means person to store, not person to device. One person on a phone plus a PC is a sync question, not a sharing one.

**D4. Key structure: one data key, one wrapped copy per protector (envelope encryption).** This decides the store format, so it is cheapest on day one. It lets protectors be added or replaced (recovery key, passkey, hardware key) by rewrapping one small key instead of re-encrypting the store. Recommend: Confirm for v1.0, and have Claude Code report whether the plan already does this.

**D5. Recovery key format (v1.0).**
- A short key ID is printed and shown at every recovery prompt. FR-032 means owners will end up holding several keys.
- The key is digits only, in groups, each group with its own checksum, so a typo is caught per group.
- This follows BitLocker's model (Verify: 48 digits in 8 checksummed groups).
- Recommend: Confirm. Exact length and checksum go in a PLAN record.

**D6. SSO (the AUTH row): what it does.**
- It covers enrollment and identity for the owner and, later, for caregivers.
- Daily unlock is delegated to device policy (Windows Hello, PIN, biometrics) through a passkey bound at enrollment.
- SSO never gates the store key (D1).
- Recommend: record as AUTH-01, refining the existing AUTH row. Target: alongside caregivers / HOST-01 / P2P, not v1.0.

**D7. SSO's second factor.** The whiteboard says "authenticator 2FA". With device-bound passkeys, the passkey already supplies the possession factor, and the device check supplies the biometric or PIN factor. Recommend: treat passkey plus device check as satisfying 2FA, and keep a TOTP authenticator only as a fallback. Owner's call.

**D8. Who registers the SSO apps.** Options:
- (a) Project-MEEP registers one shared public client for everyone (PKCE, no secret). Easiest for lay users, but makes you a single point of failure if a provider pulls the registration.
- (b) Each self-hoster registers their own. Too much friction for lay users.
- (c) (a) by default, with (b) allowed.

Recommend: decide when AUTH is specced. Leaning (c).

**D9. Recovery key custody options (later).** The owner may choose to store the recovery key with a provider they control (password manager, their own cloud). It is opt-in only, and Project-MEEP never holds it. Recommend: Confirm the principle now. Build it with AUTH.

**D10. Emergency access (ACCESS-01) and coercion.** Recommend: record two constraints now and design later.
- Emergency access must be an owner-granted protector or delegation, never a backdoor (SEC-03).
- Coercion is a named open question, not silently out of scope.

**D11. One device, several Meep instances (owner direction, 2026-11-20).** A user is always one person with one Meep account. One device may sign in to different instances, one at a time. Switching closes out the prior instance and shows the new user nothing of it.
- **Switching is real only if each instance has its own unlock.** If several instances' keys sit in the same OS keychain with no login, the next user can simply reopen the prior instance. So multi-instance switching depends on per-instance sign-in (D6) and ships with it, in vNext. v1.0 supports one instance per OS account.
- **Close-out on switch:**
  - end the server session and invalidate the cookie
  - zero the keys held in memory and close the store
  - send `Clear-Site-Data: "cache", "cookies", "storage"`
  - delete working copies
  - start the new session with an empty notifications tray and no toasts
- **No back-button leaks:** every page showing record data is sent `Cache-Control: no-store`. This is cheap enough to do in v1.0.
- **Unlock (owner decision 2026-11-20):** delegated to the device's own screen-lock settings (Windows Hello, PIN, biometrics).
- **Sign-in screen list (owner decision 2026-11-20, privacy before ease of use):**
  - The sign-in screen lists only users who turned on "persisted session" in their own Settings. It is off by default.
  - A user who hasn't turned it on is never listed and signs in from scratch every time.
- **Follow-ons (proposed):**
  - Persisted session can be turned on only if the device has a screen lock set. With no device lock there is nothing to delegate to.
  - A listed user appears by a **vanity username** (owner term, 2026-11-20). It is display only, never used for identity or sign-in, and must be unique on the device.
  - **App auto-lock (owner decision 2026-11-20):** Meep locks after 30 seconds idle by default. The owner can raise it to at most 2 minutes in app settings. Meep locks on its own even if the device never does, and also locks whenever the device locks.
  - **WCAG 2.2.1 (Level A) fit:** a 30 s to 2 min range is only 4×, short of the 10× needed for "adjust". So the lock must use the "extend" option. A warning appears at least 20 seconds before the lock, a single simple action (any key or tap) extends it, and extending works at least 10 times in a row. With the 30 s default, the warning shows at 10 s idle.
  - **Lock is not close-out:** locking hides the screen and asks for the device check to resume. Drafts and work in progress are kept, which also meets WCAG 2.2.5. Close-out (switching users) wipes everything, as above.
  - **"Idle" means** no key, pointer, touch or scroll input, and Meep's own read-aloud not playing. Proposed:
    - **Other apps' audio** (music players) never counts. A web page can't see it anyway.
    - **Meep's own read-aloud** (the DOD-16 text reader) counts as activity while it plays, because Meep knows it is playing.
    - **External screen readers** can't be reliably detected from a web page, and browsers deliberately don't expose that. Meep doesn't try. Instead the lock warning is announced as an alert dialog (`role="alertdialog"`, assertive), so it interrupts speech, and any key extends the session. That meets 2.2.1 without detection.
    - **Android (later):** the native app can check whether TalkBack is on and use that only to lengthen the warning window. It is never logged.
- **Unsaved changes (proposed):**
  - On lock or close-out, unfinished edits are saved as a draft inside the user's encrypted store. Never in browser storage.
  - On the next unlock or sign-in, Meep shows a small field-by-field diff ("Middle name: Kim → Kimberly") with **Resume** or **Discard**.
  - If the record changed after the draft was started (another device, or later a caregiver), the diff says so and shows the current value next to both.
  - Resuming never skips confirmations: a keystone change in a draft still goes through NAMES-07.
  - Drafts may be incomplete or invalid. Validation runs on save, not on draft.
  - A draft stays until the user resumes or discards it (nothing disposed). Discarding is explicit.
  - The History log records draft saved, resumed and discarded by FieldRef and alias only (LOG-01).
  - Turning persisted session off removes the user from the list and deletes their unlock material from the device.
- **Open (HOST-01):** for a user without a persisted session, the device must keep no unlock material and nothing labeled. Their data then lives on their host or on their own device, not as an unlabeled store file left behind; even an unlabeled encrypted file shows someone uses Meep here (CHK001).
- **Outside Meep's control:** files the owner exported or downloaded stay in the OS. The close-out message says so.

## 5. Things to verify before anything becomes binding

| Claim | Status |
|---|---|
| WebAuthn/passkeys work with `localhost` as the relying party (localhost is a secure context) | Verify |
| WebAuthn PRF extension (passkey derives a key) is supported on Windows Hello, Android, iCloud Keychain, Google Password Manager | Verify per platform. Needed only if a passkey becomes a key protector rather than just an app lock |
| Google and Microsoft allow loopback redirects for installed-app OAuth clients; Apple requires a paid developer membership and a registered web domain | Verify |
| A Windows user profile's `%LOCALAPPDATA%` inherits an ACL of user + SYSTEM + Administrators | Verify |
| Linux exposes every process's command line to other users | Fact (`/proc/<pid>/cmdline`), unless the system restricts it |
| SQLCipher encrypts every page, and the file starts with a random salt | Fact (default settings) |
| BitLocker recovery password format | Verify |

## 6. Repo housekeeping found during review

- `R5.md` is missing. R5 exists only in the master spec table.
- `NAMES-07`, `FOUND-09`, `FOUND-10`, `UI-02` and `UI-03` are referenced but not recorded. This is already being handled in the corrections PR.
- Once confirmed, the AUTH row, R5, R7 and FOUND-02 need cross-references to whatever D1–D10 produce.
