# SEC-05: Input validation and injection

- **ID:** SEC-05
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner
- **Update 2026-11-20:** the full-name box byte cap is **1,536 bytes** (384 graphemes x 4, the same ratio as the 512-byte name-part cap), Confirmed by the owner.
- **Note:** The owner called this SEC-01; SEC-01 is already the master spec's open question on plaintext secrets, so it is SEC-05.

## Decision

1. **SQL:** the storage wrapper accepts SQL only as `&'static str` with bound parameters. CI fails on `Box::leak` and on rusqlite `execute` or `prepare` calls outside `meep-storage`.
2. **HTML:** askama autoescape stays on; CI fails on `|safe`.
3. **Localhost web hardening:** Host allow-list (`127.0.0.1:<port>`, `localhost:<port>`), otherwise reject; Origin and `Sec-Fetch-Site` checks on all state-changing requests; per-session CSRF token; `SameSite=Strict` cookies; no CORS headers; CSP `default-src 'self'` with no inline script.
4. **Name input:** NFC normalize; at most 128 graphemes **and** at most 512 bytes per name part (the owner's figure); reject control characters (Cc), null, bidi overrides and isolates (U+202A to U+202E, U+2066 to U+2069) and noncharacters; allow ZWJ and ZWNJ; never reject apostrophes, hyphens, spaces, accents or non-Latin scripts. Rejections are friendly, cite the rule ID, and never block saving other fields.
5. Fluent bidi isolation stays enabled.
6. **Tests:** a hostile-input corpus (Bobby Tables, O'Brien, `<script>`, U+202E, Zalgo, null byte, 10,000 characters, NFD "José", emoji ZWJ, Turkish İ and ı); acceptance tests assert valid ones round-trip byte-identical and invalid ones are rejected with the right rule ID; property tests on the validators; `cargo-fuzz` targets for the normalizer and validator.
7. **Localhost-hardening tests** are Rust integration tests against the axum router (tower `oneshot`) with forged Host, Origin and `Sec-Fetch-Site` headers. No Playwright for SEC-05; Playwright and axe stay with spec 010's accessibility work.

## Placement

- **Spec 002 (foundation):** items 1, 2, 3, 5 and 7 (amends FR-021 and R5's contract, plan, tasks).
- **Spec 001 (names):** item 4, the NAMEVAL rule IDs, the grapheme and byte caps, the character rules, the hostile-input corpus and round-trip acceptance tests; item 6 for the validators.

## Later, not v1.0

The 4-bytes-per-grapheme ratio suits Latin script. Some scripts pack more bytes into one visible character (a Devanagari conjunct can be 9 to 12 bytes, an emoji family sequence 25), so long names in them would hit the byte cap before the grapheme cap. When non-Latin scripts get real support, guard against Zalgo with a cap on code points per grapheme and a generous byte cap behind it.
