# Quickstart: Validating Identity and Names (v1.0)

A validation guide, not implementation. The workspace does not exist yet (it is created by the implementation tasks), so the commands below describe what will run once it does. Details live in [data-model.md](data-model.md) and [contracts/](contracts/). Fixtures only: no real personal data, no email addresses.

## Prerequisites

- Rust stable toolchain (installed: rustc 1.100.0 with rustfmt and clippy).
- A C toolchain for the bundled SQLCipher build (Microsoft C++ Build Tools are installed).
- No network is needed to run anything; the app makes no outside calls.

## Commands (once the workspace exists)

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                        # unit and property tests
cargo test --workspace --test acceptance      # Given/When/Then, star-gated tests
cargo run -p meep-web                         # serves on the loopback address only
```

CI runs the same checks on every push and pull request; `main` merges only when `all-green` passes (ENG-02, ENG-03).

## End-to-end validation scenarios

Run each against a fresh database. Expected outcomes are in the spec; this table is the map.

| # | Scenario (fixture) | Expected outcome | Spec ref |
|---|---|---|---|
| 1 | Add Legal name First "Jordan", Middle "Lee", Last "Rivera Okafor", no menu changes | One compound surname; text "Jordan Lee Rivera Okafor"; exports to First, Middle, Last unchanged | ID-05, ID-08, AT-30 |
| 2 | Parts joined in order with one space | Equal to the stored text; a mismatching assembly is refused and nothing is written | ID-04, R14, AT-28 |
| 3 | First "José Luis", save with no menu changes | One given name, not split | AT-31 |
| 4 | Full-name box and tap-to-label for "María José García López" | Same labeled parts as three-field entry | AT-27 |
| 5 | Save, display, export and re-import "María José García López" | Every accent unchanged | AT-26 |
| 6 | Export any name to each outside format | Built from parts; anything a target cannot hold is logged as an Also known as rendering | AT-29 |
| 7 | Fresh install: visit every screen and export | No pronoun appears anywhere | AT-23 |
| 8 | Enable pronouns; check default visibility | "Only me"; only two visibility choices shown | PRON-01a |
| 9 | Outbound export versus owner's own JSON backup | Pronouns appear in the backup always; in outbound exports only if "Included in exports" | FR-024a |
| 10 | Retire "Jordan Lee Pak" | Greyed out in the list; "old" in FHIR and Previous Name in the US form on outbound export; present in the backup | ID-02, NAMES-01 |
| 11 | Add a second Legal name | Refused with an explanation | R12 |
| 12 | Replace Preferred "Jordan Rivera" | Old becomes an Active Also known as | R13 |
| 13 | Type 129 characters in a box; add a 51st name | Clear message; nothing saved; nothing cut off | NAMES-03 |
| 14 | "María" with a combining accent near the limit | Counted as 5 characters | NAMES-03 |
| 15 | Correct a typo in place | Only the new wording kept; the log shows "name edited", date, parts changed, fingerprint, no old text | NAMES-02 |
| 16 | Delete a name | Content and key unrecoverable from the database file and log; log shows only "a name was deleted" and the date | R8 |
| 17 | Set a custom short name | Shown in the list; absent from outbound exports; never used for matching | NAMES-04 |
| 18 | Send a state-changing request from a non-local origin or without the token | Rejected | PLAN-07 |

## Done when

- All unit tests in the spec's v1.0 list pass.
- The star-gated acceptance tests (AT-23, AT-26 to AT-31) pass as Given/When/Then.
- Scenarios 1 to 18 behave as stated.
- Names screens pass the spec 010 accessibility checks (AT-41, AT-42) in both themes.
- `all-green` passes in CI.
