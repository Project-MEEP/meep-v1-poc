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

## Session token

- At start Meep creates a random 256-bit session secret kept only in memory.
- The page the owner opens receives a cookie (`HttpOnly`, `SameSite=Strict`, no `Domain`) and an anti-forgery token embedded in each form or sent in a request header; a state-changing request must present both and they must match (double submit).
- The token is not a sign-in and does not identify the owner. It only stops other web pages from making Meep act (FR-021).

## Launch token (FR-052)

Each launch creates a cryptographically random token. It is never passed on a command line and never left in a URL: Meep writes a launch file readable only by the current operating-system user that redirects into Meep; the server swaps the token for a `SameSite=Strict`, `HttpOnly` cookie and redirects to a clean URL. A request without a valid cookie is refused with `request-refused`. The token is never logged. This is not a sign-in (R5). Residual risk, stated in the documentation: code running as the same operating-system user can still read the launch file or reach the port.

## Responses

All responses send `Cache-Control: no-store`, `X-Content-Type-Options: nosniff` and the content security policy `default-src 'self'` with no inline script and no inline style needing `unsafe-inline`, so other sites cannot embed or load Meep content. The server sends **no CORS headers** at all (SEC-05). Templates escape output by default; `|safe` is forbidden by CI.

## Not covered in v1.0 (FOUND-02)

Other user accounts on the same computer are not guarded against: a different operating-system account could reach the loopback port. The documentation says v1.0 assumes one person uses the computer.

## Tests (FR-020, FR-021, SC-006)

- A connection attempt to the machine's network address is refused.
- A request with a wrong `Host`, a foreign `Origin`, `Sec-Fetch-Site: cross-site`, or no token on a state-changing request gets 403.
- A normal same-origin request with the token succeeds with no sign-in.
- These are Rust integration tests against the router itself using tower `oneshot` with forged `Host`, `Origin` and `Sec-Fetch-Site` headers; no browser is needed (SEC-05). Browser-level tests stay with spec 010.
