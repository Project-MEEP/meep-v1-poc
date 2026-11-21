# Contract: Local access rules

R5, FR-020, FR-021, FOUND-02, PLAN-25, SEC-05. Meep is reachable only from the owner's own machine, with no sign-in in v1.0.

## Binding

- The server listens on the loopback address only: `127.0.0.1`, and `::1` where IPv6 loopback is available. It never binds `0.0.0.0` or any other interface, so another machine on the network cannot connect (FR-020).
- The port is chosen by the operating system at start (ephemeral) and shown to the owner in the address Meep opens; a fixed port can be added later without changing these rules.
- Binding to a non-loopback address is a startup error, not a setting.

## Accepted requests

A request is accepted only if all of these hold; otherwise it is refused with status 403 and the owner-visible message `request-refused` (FR-021):

| Check | Rule |
|---|---|
| Host | The `Host` header names the loopback address or `localhost` with the server's own port. This also blocks web pages that point another domain name at the loopback address (DNS rebinding). |
| Origin and fetch metadata | For any request that reads sensitive data through a script or changes state: if `Origin` is present it equals the server's own origin; if `Sec-Fetch-Site` is present it is `same-origin` or `none`. A missing `Origin` on a state-changing request is refused. |
| Session token | State-changing requests carry the per-session anti-forgery token (below). |
| Session cookie | Every request carries the cookie issued by the launch-token exchange; without it the request is refused (FR-052). |

**One narrow exemption (SEC-05, FR-052):** the exchange route accepts a request whose `Origin` is `null` (a page opened from a file). It is GET only, changes no data, accepts only a valid, unused, unexpired token, and redirects to a clean URL. No other route has this exemption.

## Session token

- At start Meep creates a random 256-bit session secret kept only in memory.
- The cookie (`HttpOnly`, `SameSite=Strict`, no `Domain`) and the anti-forgery token are issued only through the launch-token exchange below, never to an ordinary page load. The token is embedded in each form or sent in a request header; a state-changing request must present both and they must match (double submit).
- The token is not a sign-in and does not identify the owner. It only stops other web pages from making Meep act (FR-021).

## Launch token (FR-052, FOUND-11)

- Each launch creates a cryptographically random token, **one-time**, that **expires 60 seconds after launch**. It is never passed on a command line and never left in a URL.
- Meep writes a launch file readable only by the current operating-system user that redirects into Meep. Windows: under `%LOCALAPPDATA%\Meep`, which inherits the per-user permissions. Linux and macOS: a file with mode 0600 in a directory with mode 0700. Never a shared temporary folder.
- The server exchanges a valid, unused token for the cookie and redirects to a clean URL. The cookie is the session credential afterwards. A request without a valid cookie is refused with `request-refused`.
- The file is deleted after the exchange or on expiry; stale launch files are deleted at start-up.
- The token is never logged. This is not a sign-in (R5).
- Residual risk, stated in the documentation: the same operating-system account, and an administrator or root, can still reach Meep; other operating-system accounts are blocked at the port.

## Responses

All responses, in particular every response that contains record data (FR-057), send `Cache-Control: no-store`, `X-Content-Type-Options: nosniff` and the content security policy `default-src 'self'` with no inline script and no inline style needing `unsafe-inline`, so other sites cannot embed or load Meep content. The server sends **no CORS headers** at all (SEC-05). Templates escape output by default; `|safe` is forbidden by CI.

## Not covered in v1.0 (FOUND-02, FOUND-11)

Other operating-system accounts are blocked at the port by the launch-token exchange above. The same account, and administrator or root access, are out of scope. The documentation says anyone using the owner's operating-system account can see everything.

## Tests (FR-020, FR-021, SC-006)

Also (FR-052): a reused token, an expired token and a missing or wrong token are refused; the exemption applies to no other route; the token is absent from logs and from the final URL.

- A connection attempt to the machine's network address is refused.
- A request with a wrong `Host`, a foreign `Origin`, `Sec-Fetch-Site: cross-site`, or no token on a state-changing request gets 403.
- A normal same-origin request with the token succeeds with no sign-in.
- These are Rust integration tests against the router itself using tower `oneshot` with forged `Host`, `Origin` and `Sec-Fetch-Site` headers; no browser is needed (SEC-05). Browser-level tests stay with spec 010.
