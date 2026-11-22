//! Detail keys and typed detail values (ERR-01 item 4, LOG-01, contracts/change-log-entry.md).
//! Rule: every detail value is a number, a time or date, or a member of a closed registered set.
//! No open vocabularies, so free text cannot be represented at all.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::address::Address;
use meep_core::detail::{DetailKey, DetailValue, NoticeId, PartRole};
use meep_core::errors::Error;
use std::collections::BTreeSet;
use time::macros::{date, datetime};

#[test]
fn detail_keys_are_copy_and_display_as_their_stable_code() {
    let k = DetailKey::NoticeId;
    let copy = k;
    assert_eq!(k, copy);
    assert_eq!(k.to_string(), "notice_id");
}

#[test]
fn every_detail_key_has_a_unique_snake_case_code() {
    let mut seen = BTreeSet::new();
    for k in DetailKey::ALL {
        let code = k.as_str();
        assert!(
            code.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "{code}"
        );
        assert!(seen.insert(code), "duplicate {code}");
        assert_eq!(k.to_string(), code);
    }
    assert_eq!(DetailKey::ALL.len(), 7);
}

#[test]
fn a_refused_detail_names_its_key_only_in_the_diagnostic_log() {
    let e = Error::DetailRefused {
        key: DetailKey::AsOf,
    };
    assert_eq!(e.to_string(), "E-DETAIL-REFUSED");
    assert_eq!(e.detail_key(), Some(DetailKey::AsOf));
    assert_eq!(Error::Stale.detail_key(), None);
}

/// Spec 001 data-model.md, part `role` (ID-04): exactly these six.
#[test]
fn part_roles_are_exactly_the_names_data_model_roles() {
    let codes: Vec<&str> = PartRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        codes,
        [
            "given",
            "given_additional",
            "family",
            "particle",
            "prefix",
            "suffix"
        ]
    );
}

#[test]
fn notice_ids_are_a_closed_set_mirroring_their_fluent_message_ids() {
    let codes: Vec<&str> = NoticeId::ALL.iter().map(|n| n.as_str()).collect();
    assert_eq!(codes, ["backup-made", "backup-failed", "save-failed"]);
}

#[test]
fn a_typed_value_knows_its_key_and_stores_as_canonical_text() {
    let cases = [
        (DetailValue::FormatVersion(1), DetailKey::FormatVersion, "1"),
        (DetailValue::RecordCount(42), DetailKey::RecordCount, "42"),
        (
            DetailValue::Address(Address::parse("Alpha.Meep.Personal.Notes.Sample").unwrap()),
            DetailKey::Address,
            "Alpha.Meep.Personal.Notes.Sample",
        ),
        (
            DetailValue::ChangedParts(vec![(PartRole::Family, 2), (PartRole::GivenAdditional, 1)]),
            DetailKey::ChangedParts,
            "family[2],given_additional[1]",
        ),
        (
            DetailValue::AsOf(datetime!(2026-11-22 08:00:00 UTC)),
            DetailKey::AsOf,
            "2026-11-22T08:00:00Z",
        ),
        (
            DetailValue::Date(date!(2026 - 11 - 22)),
            DetailKey::Date,
            "2026-11-22",
        ),
        (
            DetailValue::NoticeId(NoticeId::BackupFailed),
            DetailKey::NoticeId,
            "backup-failed",
        ),
    ];
    for (value, key, text) in cases {
        assert_eq!(value.key(), key);
        assert_eq!(value.to_stored(), text);
        // Defensive read path: the stored text parses back to the same typed value.
        assert_eq!(DetailValue::from_stored(key, text), Some(value));
        assert!(key.accepts(text));
    }
}

/// `DetailKey::accepts` is the defensive check used when reading stored entries or importing a
/// backup, where text arrives instead of typed values.
#[test]
fn each_key_accepts_only_canonical_text_of_its_own_type() {
    let cases: &[(DetailKey, &[&str], &[&str])] = &[
        (
            DetailKey::FormatVersion,
            &["1", "12"],
            &["", "v1", "1.0", "01", "4294967296"],
        ),
        (
            DetailKey::Address,
            &["Alpha.Meep.Personal.Notes.Sample"],
            &[
                "Okafor",
                "Alpha.Meep",
                "Alpha.Meep.Personal.Notes.Sample extra",
            ],
        ),
        (
            DetailKey::ChangedParts,
            &["family[2]", "given[1],family[1]", "particle[10]"],
            &[
                "Jordan",
                "okafor[1]",
                "rivera[1]",
                "family",
                "family[x]",
                "Family[2]",
                "family[2],",
                "family[02]",
                "family[65536]",
                "Jordan Lee",
            ],
        ),
        (
            DetailKey::AsOf,
            &["2026-11-22T08:00:00Z"],
            &[
                "yesterday",
                "2026-11-22",
                "Jordan",
                "2026-11-22T08:00:00+05:00",
            ],
        ),
        (
            DetailKey::RecordCount,
            &["0", "42"],
            &["", "-1", "4.2", "4294967296"],
        ),
        (
            DetailKey::Date,
            &["2026-11-22"],
            &["2026-13-01", "2026-11-31", "11/22/2026", "Okafor"],
        ),
        (
            DetailKey::NoticeId,
            &["backup-made", "save-failed"],
            &[
                "",
                "Backup Failed",
                "name-changed",
                "okafor",
                "backup-made ",
            ],
        ),
    ];
    for (key, good, bad) in cases {
        for v in *good {
            assert!(key.accepts(v), "{key} should accept {v:?}");
        }
        for v in *bad {
            assert!(!key.accepts(v), "{key} should refuse {v:?}");
        }
    }
}

#[test]
fn a_key_shaped_value_is_refused_for_every_key() {
    let hex = "a".repeat(64);
    let b64 = "QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo0MTIz";
    for key in DetailKey::ALL {
        assert!(!key.accepts(&hex), "{key}");
        assert!(!key.accepts(b64), "{key}");
    }
}

/// Canary: synthetic surnames from the spec 001 fixtures (Okafor, Rivera, Pak) are refused under
/// every key, whether bare, lowercased or dressed as another key's shape.
#[test]
fn synthetic_surnames_are_refused_under_every_detail_key() {
    for surname in ["Okafor", "Rivera", "Pak", "Garcia"] {
        let lower = surname.to_lowercase();
        let dressed = [
            surname.to_string(),
            lower.clone(),
            format!("{lower}[1]"),
            format!("{surname}[1]"),
            format!("family[1],{lower}[2]"),
            format!("{lower}-{lower}"),
        ];
        for key in DetailKey::ALL {
            for text in &dressed {
                assert!(!key.accepts(text), "{key} accepted {text:?}");
            }
        }
    }
}

/// Documents the open gap (owner item 4): today any syntactically valid address is accepted, so a
/// surname in a segment passes. It will pass once addresses are checked against registered
/// address spaces with closed segment vocabularies. Ignored until the owner confirms that rule.
#[test]
#[ignore = "pending the owner's rule on registered address spaces"]
fn a_surname_in_an_address_segment_is_refused() {
    assert!(!DetailKey::Address.accepts("Alpha.Meep.Personal.Okafor.Names"));
}
