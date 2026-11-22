# Verification of the "Verify" claims in the Access Review

- **Date:** 2026-11-20
- **Source:** section 5 of [Meep-Review-Access_2026-11-20_0848-CT.md](Meep-Review-Access_2026-11-20_0848-CT.md)
- **Status:** Research record, not a decision. Nothing here is binding. It records what was checked, against what, and how strong the evidence is. Three AI research agents did the reading; their sources are listed so the owner can re-check them. Pages read through a summarizing fetch are marked as such.
- **Effect on v1.0:** none of these results changes a Confirmed v1.0 decision. The Windows result matters to T139 (spec 002); the rest concern vNext sign-in (AUTH-01) and the recovery-key format (PLAN-29).

## Summary

| Claim | Result | Evidence strength |
|---|---|---|
| Windows `%LOCALAPPDATA%` is not readable by other standard users | Confirmed on one machine | Local read-only `icacls` check on one Windows 11 PC; no Microsoft page found that states the profile default |
| BitLocker recovery password: 48 digits, 8 groups of 6, checksummed | Confirmed | Secondary (libbde format document); Microsoft says only "48-digit" |
| WebAuthn works with `localhost` as the relying party | Confirmed | Primary (W3C WebAuthn Level 3); browser differences not verified |
| WebAuthn PRF on Windows Hello, Android, iCloud Keychain, Google Password Manager | Unclear | No primary per-platform matrix found; secondary sources disagree |
| Google and Microsoft allow loopback redirects; Apple needs membership and a domain | Google confirmed; Microsoft and Apple partly | Provider docs for Google and Microsoft; Apple's localhost rule only from forum and third-party posts |

## 1. Windows: `%LOCALAPPDATA%` permissions (affects T139, FR-052, FOUND-11)

**Checked:** a read-only `icacls` listing of `%LOCALAPPDATA%` and of the user profile folder on one Windows 11 machine (user name not recorded).

- The profile folder and `%LOCALAPPDATA%` list SYSTEM, Administrators and the owning user, each with full control and inheritance to children. The `Users` and `Everyone` groups are not on the profile folder.
- A new folder or file created inside inherits the same entries, so another standard user cannot read it unless someone changes the permissions. Administrators can read it.
- The machine also showed extra read entries for a development tool's sandbox accounts. Those are specific to this machine, not a Windows default.

**Limits:** one machine, no Microsoft documentation page found for the default profile ACL, and only the default Users group was checked. This supports the FR-052 launch-file design (a per-user folder, owner-only) but is not a guarantee. It does not protect against administrators or offline disk access, which FOUND-02 and FOUND-11 already put out of scope.

**Follow-up for T139 (not a new decision):** the launch-token tests should assert the launch file's permissions on a real Windows runner, not assume them.

## 2. BitLocker recovery password format (compares with PLAN-29)

- A recovery password is 48 decimal digits in 8 groups of 6, usually shown with hyphens. Each 6-digit group, read as an integer, must be divisible by 11. Each group divided by 11 gives a 16-bit value, so the 8 values form a 128-bit key.
- It is a divisibility check, not a separate check digit, and it does not catch every transposition.
- **Sources:** Microsoft Learn, BitLocker recovery overview (says only "48-digit number"); libbde, BitLocker Drive Encryption (BDE) format document, an open-source reverse-engineered specification (states the grouping and the rule; read through a summarizer, not the raw page).

**Relation to PLAN-29:** PLAN-29 uses a different scheme (a Damm check digit after each 6-digit group, 91 printed digits, 256-bit key) and does not depend on BitLocker's rule. The claim is only that the format idea (digits, fixed groups, a per-group check) exists in a widely used product. That is accurate.

## 3. WebAuthn (affects AUTH-01, vNext only)

**`localhost` as the relying party: Confirmed.** The W3C WebAuthn Level 3 text requires a secure context and makes an explicit exception for an origin whose host is `localhost` over `http`. The RP ID would be `localhost`.

- An IP address cannot be an RP ID, so a passkey flow must be reached as `localhost`, not `127.0.0.1`. The v1.0 Host rule in `contracts/local-access.md` already allows `localhost`, so this does not conflict with v1.0.
- A credential made for `localhost` is bound to that name; it does not follow the app to another host.
- **Not verified:** behavior in Chrome, Edge, Firefox and Safari. MDN says only "secure contexts".

**PRF extension: Unclear.** No primary per-platform support matrix was found.

- Windows Hello: unclear. Microsoft's passkey page does not mention PRF. One third-party source says it works from a February 2026 update; another says it is not suitable. Both unverified.
- Android and Google Password Manager: probably supported, per secondary sources whose version numbers disagree.
- iCloud Keychain: probably supported (iOS and iPadOS 18.4+, macOS 15.4+, Safari 18+ per secondary sources); not confirmed against Apple documentation.
- Security keys: PRF rides on the CTAP2 `hmac-secret` extension, which many keys support; which keys was not verified.
- Some providers accept a PRF request but return no result.

**Design implication (for the owner, when AUTH-01 is specified):** if a passkey ever becomes a key protector, detect PRF at runtime and keep a fallback. As an app lock only, PRF is not needed.

**Sources:** W3C WebAuthn Level 3 (RP ID, secure context, localhost exception; the PRF section was not read); MDN Web Authentication API; passkeys.dev device support (no PRF information); Microsoft Learn passkeys overview (no PRF mention). The Chrome PRF page returned 404. Secondary only: Corbado posts, a GitHub PRF support matrix, the Chromium intent-to-ship thread, Mozilla Bugzilla items.

## 4. OAuth loopback redirects (affects AUTH-01 D8, vNext only)

- **RFC 8252:** a native app uses `http` with a loopback IP literal (`127.0.0.1` or `[::1]`); the server must accept any port; `localhost` is not recommended. Read through a summarizing fetch.
- **Google: Confirmed.** Desktop apps use a loopback IP redirect; the copy-and-paste flow is deprecated; PKCE expected; the client secret is optional. Not checked: whether a shared client needs app verification for sensitive scopes.
- **Microsoft: Partly.** Loopback redirects work and the port is ignored when matching, but the portal rejects `http://127.0.0.1`, so it has to be set in the app manifest; `[::1]` is not supported (unlike the RFC); the public client flow must be enabled; personal accounts allow 100 redirect URIs. Page updated 2026-06-15. Not checked: publisher verification or admin consent for organization tenants.
- **Apple: Partly.** Sign in with Apple needs a Services ID, a registered web domain and HTTPS return URLs, and the Developer Program is a paid membership. That loopback and `localhost` are rejected comes from forum and third-party posts, not an Apple statement.

**Conclusion for D8:** one shared public client registered by the project looks feasible for Google and Microsoft. It does not look realistic for Apple for a self-hosted desktop app. The D8 option (c) wording stands as a leaning, not a decision.

**Sources:** RFC 8252; Google Identity, OAuth 2.0 for native apps; Microsoft Learn, redirect URI (reply URL) restrictions; Apple Developer, Sign in with Apple for the web setup; Apple Developer Program, what's included; an Apple developer forums thread and a third-party setup guide (snippet only).

## What this does not cover

The review's "Fact" rows (the Linux process command line, SQLCipher page encryption) were not re-checked here. Re-verify the Windows result on a Windows CI runner when T139 is built, and re-read the provider pages before AUTH-01 is specified, since provider rules change.
