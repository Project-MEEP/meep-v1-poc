# PLAN-07: Web server and templates

- **ID:** PLAN-07
- **Status:** Proposed (planning choice; the owner promotes it to Confirmed)
- **Date:** 2026-11-19
- **Owner:** Owner (proposed in the plan for spec 001, names)

## Decision

`axum` 0.8.9 on `tokio` 1.54.1 with `askama` 0.16.1 templates, minimal JavaScript, loopback address only, per-session anti-forgery token and `Host`/`Origin` checks on state-changing requests (implements R3, R5).

## Alternatives considered

`maud` 0.27.0; `minijinja` 3.1.0 (the stated fallback if the Fluent helper does not fit `askama`); a single-page JavaScript app (contradicts R3).

## Notes

Compile-time checked server-rendered HTML; the anti-forgery rule closes the hole where a web page the owner visits could send requests to a local server.
