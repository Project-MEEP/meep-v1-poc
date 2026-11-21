# PLAN-30: Dependency license allow-list additions (Zlib; blessing and WTFPL for sqlcipher-src only)

- **ID:** PLAN-30
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

The `deny.toml` license allow-list (principle 18) gains one license and one reviewed exception:

- **`Zlib`** is allowed for every crate. It is a permissive, OSI-approved license and is carried by `foldhash` (reached through `rusqlite`, so it cannot be avoided) and by `zlib-rs`.
- **`blessing` and `WTFPL`** are allowed **only** for the crate `sqlcipher-src`, as a per-crate exception. `blessing` is the SQLite public-domain notice and `WTFPL` is a permissive grant; both are carried by the SQLCipher source the owner chose in R2. No other crate may use them.

## Alternatives considered

- Allow nothing and keep the dependency check red: rejected; `foldhash` and `sqlcipher-src` cannot be avoided without dropping the R2 database choice.
- Allow `blessing` and `WTFPL` globally: rejected; a per-crate exception keeps any new crate under those terms visible.

## Also done in the same change

`jsonschema` (dev) has its default features off, because they pull in the banned `reqwest` (FR-027); `zip` has its default features off and keeps only deflate, which drops the bzip2 and ppmd licenses.

## Closes

Spec 002 T037 (research.md open item 3). Task T007 verifies the resulting `deny.toml`.
