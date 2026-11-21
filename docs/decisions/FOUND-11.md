# FOUND-11: Launch-token exchange for local access (amends FOUND-02 item 2)

- **ID:** FOUND-11
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

Spec 002 FR-052. The cookie and the anti-forgery token are issued only through a one-time launch-token exchange, never to an ordinary page load. The token expires 60 seconds after launch; the HttpOnly, SameSite=Strict cookie is the session credential afterwards. The token reaches the browser through a launch file readable only by the current operating-system user (Windows: under `%LOCALAPPDATA%\Meep`; Linux and macOS: mode 0600 in a 0700 directory; never a shared temporary folder), never on a command line, never left in a URL. The file is deleted after the exchange or on expiry, and stale files at start-up. The exchange is a GET from a file page (Origin `null`) and has a narrow SEC-05 exemption: GET only, changes no data, accepts only a valid unused token, redirects to a clean URL; it applies to no other route.

## Why

local-access.md previously handed the cookie and anti-forgery token to any page load, so another local account could fetch them. The file is encrypted and the keychain is per user, so the port was the only path between local accounts.

## Update 2026-11-20: exchange route exemption (owner, T153)

The launch file is a page opened from disk, so the exchange request is cross-site, has no cookie yet and may carry `Origin: null`. The owner chose to exempt the exchange route, and only that route, from the cookie check, the `Origin: null` refusal and the `Sec-Fetch-Site: cross-site` refusal. Fences that stay: GET only, no data change, one valid unused token within 60 seconds, a redirect to a clean URL, and the `Host` check. Rejected: starting the browser at a URL carrying the token, which would put the token in a URL or on a command line (FR-052).

## Amendment to FOUND-02 item 2

v1.0 blocks other local operating-system accounts at the port. The same account and administrator or root access remain out of scope and the documentation says so (one person, one Meep account, one operating-system account; see the access decisions).
