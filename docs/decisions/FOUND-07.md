# FOUND-07: Test-only features never ship (spec 002)

- **ID:** FOUND-07
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 002 analysis fixes, findings I2 and I4)

## Decision

1. `plaintext-test-store` and the in-memory key store (`in-memory-key-store`) are test-only cargo features, never enabled by default.
2. Release builds must not compile them in. A CI check (`.github/scripts/check-release-features.sh`, job `release-features`, part of `all-green`) fails if a release or signed artifact is built with either feature, if either is in a default feature set, or if the marker string either feature embeds appears in a built artifact.
3. Code using them refuses to start outside tests: a start-up check aborts if either feature is compiled into a release build, or into a debug build without the explicit test variable `MEEP_ALLOW_TEST_FEATURES=1`.
4. Spec 015's release and signed artifacts must run the same check.

Note: the owner asked for this to be recorded as FOUND-06, but FOUND-06 is already the "backups while changes are being saved" decision, so it is recorded as FOUND-07.
