# Research: Identity and Names (v1.0)

Phase 0 output. Every technology choice below is **Proposed** until the owner confirms its decision record (`docs/decisions/PLAN-nn.md`). Choices that merely implement a Confirmed decision say which one. Nothing here is left as NEEDS CLARIFICATION.

## Verification log (2026-11-19)

| Item | Source checked | Result |
|---|---|---|
| Crate names, current versions, licenses, last update | crates.io API | All found; see table in plan.md. Versions: rusqlite 0.43.0, keyring 4.2.0, unicode-segmentation 1.13.3, unicode-normalization 0.1.26, chacha20poly1305 0.11.0, hmac 0.13.0, sha2 0.11.1, hkdf 0.13.0, zeroize 1.9.2, rand 0.10.4, uuid 1.29.3, serde 1.0.229, serde_json 1.0.154, thiserror 2.0.21, axum 0.8.9, tokio 1.54.1, askama 0.16.1, fluent-templates 0.15.1, cucumber 0.23.0, proptest 1.12.0 |
| `rusqlite` SQLCipher support | crates.io feature list for 0.43.0 | Features `bundled-sqlcipher`, `bundled-sqlcipher-vendored-openssl` and `sqlcipher` exist |
| `keyring` platform support | crates.io feature list for 4.2.0 | Present; the exact per-platform backend features (Windows, macOS, Linux secret service) were **not** confirmed from metadata: confirm in the first scaffold task |
| FHIR HumanName `use` codes (R4) | hl7.org R4 `codesystem-name-use` | Codes `usual`, `official`, `temp`, `nickname`, `anonymous`, `old` confirmed. `old` = "no longer in use (or was never correct, but retained for records)" |
| Later FHIR releases | hl7.org directory | R6 listed as stable (Sept 2026). **Not verified:** whether HumanName changed in R6, and which release US Core currently requires. Plan targets R4 and re-checks before any R6 target |
| 128 characters per field | designsystem.digital.gov name pattern (fetched) | Page text: "Allow users to enter up to 128 characters per field" |
| USCDI Previous Name element | isp.healthit.gov (the URL the owner gave) | The site refuses automated requests (HTTP 403), so this was **not machine-checked**; it stands on the owner's verification recorded in NAMES-01 |
| Rust toolchain | local | rustc 1.100.0, rustfmt and clippy present; a test program compiles and runs |
| License fit with MIT (principle 18) | crate licenses | All MIT or Apache-2.0 (`unicode-normalization` and `icu_segmenter` add Unicode-3.0). The `cargo-deny` allow-list must include Unicode-3.0 |

## Decisions

### D1. Workspace layout: three crates (PLAN-01)

- **Decision**: `meep-core` (pure domain), `meep-storage` (encryption, envelope, change log, key custody), `meep-web` (server-rendered UI).
- **Rationale**: matches the spec's proposed first-session scaffold; keeps the domain testable with no database or browser; lets a later Tauri Android build reuse core and storage (R3). `meep-storage` is shared with spec 002, so names adds a repository to it rather than owning it.
- **Alternatives**: one crate (fastest to start, but mixes web types into the domain and blocks reuse); five or more crates (more ceremony than one owner and one feature justify).

### D2. Encrypted database access: `rusqlite` with `bundled-sqlcipher` (PLAN-02, implements R2)

- **Decision**: `rusqlite` 0.43.0 with the `bundled-sqlcipher` feature (the `-vendored-openssl` variant if the build needs it on a platform).
- **Rationale**: the owner confirmed SQLCipher (R2). `rusqlite` is the mainstream synchronous SQLite crate (about 160 million downloads), exposes SQLCipher through a feature, and a synchronous API suits a one-owner local app.
- **Alternatives**: `sqlx` (async and compile-time checked queries, but SQLCipher use is less direct and async adds weight with no benefit here); plain SQLite plus application-level encryption (loses whole-file encryption, contradicting R2); an embedded key-value store (no SQL, reinvents queries).
- **Risk**: the bundled SQLCipher build on Windows needs a working C toolchain (already installed) and may need the vendored OpenSSL variant. Confirm in the scaffold task.

### D3. Key custody: `keyring` plus a one-time recovery key (PLAN-03, implements R7)

- **Decision**: store the master key in the operating-system keychain through `keyring` 4.2.0, and at first run show a one-time recovery key for the owner to print or store offline. No backdoor (R7).
- **Rationale**: `keyring` is the cross-platform keychain crate; it implements exactly R7. The recovery key is shown once and never stored in plain text by Meep.
- **Alternatives**: calling the Windows credential APIs directly (platform-specific, blocks macOS and Linux and the Android reuse); a passphrase-derived key (needs a login-like prompt, but R5 says no sign-in); keeping the key in a file next to the database (defeats encryption).
- **Open for the scaffold task** (not decided here): which exact backends and features `keyring` needs per platform; and the recovery-key format. The v1.1 revisit already recorded in R7 (recovery drill, hardware key) still applies.

### D4. Per-record keys and crypto-shredding: `chacha20poly1305` with `zeroize` (PLAN-04, implements R8)

- **Decision**: each name record's content is encrypted with its own random data key using XChaCha20-Poly1305 (`chacha20poly1305` 0.11.0), the data key is stored wrapped under the master key, and deleting a name destroys its wrapped key. Keys live in memory only inside zeroizing wrappers (`zeroize` 1.9.2).
- **Rationale**: R8 requires deletion to destroy content irrecoverably while the log keeps only "a name was deleted" with the date. XChaCha20-Poly1305 is an authenticated cipher whose long random nonce removes the nonce-reuse risk when many small records are encrypted.
- **Alternatives**: AES-GCM through `aes-gcm` 0.11.1 (fine and hardware-accelerated, but a 96-bit random nonce needs more care at scale; ours is small, so this is a close call and an acceptable fallback); relying on the whole-database encryption alone (cannot destroy one record's content); overwriting the content in place (unreliable on SQLite pages).
- **Risks and required tests**: SQLite can leave old bytes in free pages and the write-ahead log. Mitigation: enable `secure_delete`, overwrite the wrapped-key row before deleting it, checkpoint and truncate the log after a delete, and add a test that a deleted name's key and ciphertext cannot be found in the database file or its log.

### D5. Tamper-evidence for corrections: keyed fingerprint (PLAN-05, implements NAMES-02)

- **Decision**: when a name is corrected in place, the change log records the date, which parts changed, and `HMAC-SHA-256(edit_subkey, canonical prior value)`, where `edit_subkey` is derived from the record's own key with HKDF (`hmac` 0.13.0, `sha2` 0.11.1, `hkdf` 0.13.0). Only the new wording is stored.
- **Rationale**: a plain hash of a short name could be recovered by guessing; a keyed fingerprint cannot, and it becomes meaningless when the record key is destroyed (R8). This is the mechanism the owner described in NAMES-02, now chosen concretely. The foundation's hash chain (spec 002) covers the log entry itself, so tampering with either the entry or the record is detectable.
- **Alternatives**: store the old text encrypted (rejected by NAMES-02: a hidden copy the owner cannot remove); a plain hash (guessable); no fingerprint (loses tamper-evidence).
- **Note**: when delegates can edit names (a later feature), edits by others keep the prior wording visible to the owner (recorded entry condition, NAMES-02).

### D6. Counting and normalizing text: `unicode-segmentation` and `unicode-normalization` (PLAN-06, implements ID-04, ID-09, NAMES-03)

- **Decision**: normalize entered text to NFC with `unicode-normalization` 0.1.26, then count user-perceived characters as extended grapheme clusters with `unicode-segmentation` 1.13.3. Limits: 128 per box, 384 in the full-name box (NAMES-03). Separately store an accent-folded matching key (ID-04) that never changes stored text.
- **Rationale**: both crates are very widely used and current. Counting graphemes after NFC means "María" counts as 5 whether the accent is precomposed or combining (spec edge case), and Cherokee syllabary and other scripts are not counted early or unfairly.
- **Alternatives**: `chars().count()` or byte length (miscounts combined characters); `icu_segmenter` 2.4.0 (more complete locale-aware segmentation, much larger dependency; revisit if a later language needs it).
- **Open for the scaffold task**: the exact accent-folding algorithm for the matching key (decomposition plus mark removal is the likely approach; list the test cases).

### D7. Web server and templates: `axum` with `askama` (PLAN-07, implements R3, R5)

- **Decision**: `axum` 0.8.9 on `tokio` 1.54.1 for the server, `askama` 0.16.1 for compile-time-checked server-rendered templates, minimal JavaScript. Bind only to the loopback address (R5). Because any web page the owner visits could try to send requests to a local server, require a per-session anti-forgery token on every state-changing request and reject requests whose `Host` or `Origin` is not the local one.
- **Rationale**: server-rendered HTML with minimal JavaScript (R3) keeps one Rust codebase and works for the accessibility goals (semantic HTML). `askama` checks templates at compile time, which suits the clean-code and test-first rules; `axum` is the dominant Rust web framework (about 600 million downloads).
- **Alternatives**: `maud` 0.27.0 (templates as Rust code; fewer designer-friendly files, and its last release is older), `minijinja` 3.1.0 (runtime templates, flexible but errors surface at run time); a single-page JavaScript app (contradicts R3).
- **Fluent integration**: `askama` has no built-in Fluent support; the plan wraps `fluent-templates` in a small `t("message-id")` helper and tests that it can be called from templates. If this proves awkward, `minijinja` is the fallback (no decision change to R3 or R10).

### D8. Interface text: Fluent through `fluent-templates` (PLAN-08, implements R10)

- **Decision**: all interface strings live in `locales/en/main.ftl`, loaded through `fluent-templates` 0.15.1 (which builds on Fluent's Rust bundle). English only in v1.0; Spanish grammatical gender is supported by the Fluent format when Spanish arrives.
- **Rationale**: R10 confirmed Fluent. A test fails the build if a template contains literal user-facing text instead of a message ID.
- **Note**: `fluent-bundle` itself was last released in May 2025; it is stable and Mozilla-maintained, but this is the least-recently-updated dependency and is flagged for a periodic check.

### D9. Exports as Strategy plus Adapter (PLAN-09, implements ID-10, NAMES-01, PRON-01a)

- **Decision**: a `NameExporter` trait (Strategy) with three implementations: the FHIR R4 HumanName exporter, the US health-data (USCDI) name exporter, and the JSON backup exporter. Each builds only from the stored parts (ID-04). Outbound exporters respect the per-name outbound-sharing flag (NAMES-01), omit short forms (NAMES-04) and apply pronoun visibility (PRON-01a); the JSON backup always includes every name and pronouns. Anything a target cannot hold is logged as an Also known as rendering (AT-29).
- **Rationale**: these are the behaviors the spec fixes; the trait keeps each format independent and lets later formats be added without touching the domain (ENG-01).
- **FHIR use mapping** (ID-10, verified against R4): Legal to `official`, Preferred to `usual`, Nickname to `nickname`, Also known as to `usual` while Active and `old` once No longer used. The R6 question is open (see verification log).
- **Open for the scaffold task**: the exact USCDI element layout for Previous Name and the exact JSON backup field names (specified in contracts/export-formats.md, to be confirmed against the standard at implementation).

### D10. Design patterns in Rust (PLAN-10, implements ENG-01)

- **Decision**: Builder for assembling a name from labeled parts (`NameBuilder`, with the rebuild-equals-text check at `build()`); State for name status as an enum (`Active` or `NoLongerUsed`) with typestate only where it prevents an invalid record (for example, Legal and Preferred cannot be retired); Strategy and Adapter for exports (D9); Observer for domain events feeding the change log (a small subscriber list or channel; no framework).
- **Rationale**: these are the patterns the owner's spec lists for this feature, translated to traits and enums rather than inheritance.
- **Alternatives**: a general-purpose event bus crate (more machinery than a handful of events needs).

### D11. Identifiers and addresses: `uuid` and a hand-written parser (PLAN-11, implements R6, R9, R16)

- **Decision**: a random UUID (version 4) as the account ID on every record (`uuid` 1.29.3 with `rand` 0.10.4 as its randomness source); the address `Alpha.Meep.Personal.Identity.Names` and exactly five dot-separated segments, each starting with a letter and holding only letters, digits and hyphens, compared case-insensitively (R16), through a small hand-written parser in `meep-core`.
- **Rationale**: the grammar is tiny and fully specified; a hand-written parser is easier to test than adding a parsing crate. The envelope itself is spec 002's; names only supplies and checks its own address.
- **Alternatives**: a regular-expression crate (adds a dependency for one rule); sequential IDs (leak ordering and break the peer-to-peer plan in R9).

### D12. Test tooling: `cargo test`, `cucumber`, `proptest` (PLAN-12, implements ENG-02)

- **Decision**: `cucumber` 0.23.0 for Given/When/Then acceptance tests the owner can read (AT-23, AT-26 to AT-31), `proptest` 1.12.0 for properties (parts always rebuild text; round trips), and plain `cargo test` for the unit list in the spec. Browser tests with accessibility checks (AT-41, AT-42) are specified in spec 010 and run against these screens.
- **Rationale**: the spec lists the `cucumber` crate as a candidate to verify at G2; it is verified and current.
- **Alternatives**: hand-written acceptance tests as plain Rust (less readable for the owner).

## Dependencies on other specs (interfaces only)

See contracts/foundation-interfaces.md. In short, names needs from spec 002: the record envelope, an append-only hash-chained change log accepting events, the encrypted store and key custody, and JSON export and import; from spec 010: the text field, menu, button, toggle, list row, status label and Settings page elements and the theme toggle.

## Remaining open items (none block planning)

1. `keyring` backend features per platform and the recovery-key format (D3).
2. Accent-folding algorithm and test cases for the matching key (D6).
3. Exact USCDI Previous Name layout and JSON backup field names (D9).
4. Whether FHIR R6 changes HumanName, and which FHIR release the target systems require (D9).
5. The `askama` and Fluent helper proving out; `minijinja` is the stated fallback (D7).
6. `cargo-deny` configuration must allow the Unicode-3.0 license (all).
