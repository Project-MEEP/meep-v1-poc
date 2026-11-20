# PLAN-25: Loopback server and anti-forgery

- **ID:** PLAN-25
- **Status:** Proposed (planning choice for spec 002, the foundation; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 002)

## Decision

`axum` 0.8.9 on `tokio` 1.54.1 with `tower-http` 0.7.1 layers; loopback binding only (127.0.0.1 and ::1), ephemeral port; Host, Origin and Sec-Fetch-Site checks; a per-session random token as an HttpOnly SameSite=Strict cookie plus a matching request token (`cookie` 0.18.2); no-store and content-security headers.

## Alternatives considered

A secret only in the launch address; a sign-in (excluded by R5).

## Notes

With no sign-in these checks are the only barrier against other web pages. Other local user accounts are not covered (FOUND-02). Moves and refines the server part of PLAN-07.
