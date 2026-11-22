//! The closed set of event detail keys (ERR-01 item 4, contracts/change-log-entry.md).

use meep_core::detail::DetailKey;
use meep_core::errors::Error;
use std::collections::BTreeSet;

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

/// R8, FR-014: a detail value must fit the grammar of its own key, so a name or a key cannot
/// pass as a detail just by being short or free of spaces.
#[test]
fn each_key_accepts_only_values_of_its_own_shape() {
    let cases: &[(DetailKey, &[&str], &[&str])] = &[
        (
            DetailKey::FormatVersion,
            &["1", "12"],
            &["", "v1", "1.0", "1234567"],
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
            &["family[2]", "given[1],family[1]", "middle[10]"],
            &[
                "Jordan",
                "okafor",
                "family",
                "family[x]",
                "Family[2]",
                "family[2],",
                "Jordan Lee",
            ],
        ),
        (
            DetailKey::AsOf,
            &["2026-11-22T08:00:00Z"],
            &["yesterday", "2026-11-22", "Jordan"],
        ),
        (
            DetailKey::RecordCount,
            &["0", "42"],
            &["", "-1", "4.2", "1234567890"],
        ),
        (
            DetailKey::Date,
            &["2026-11-22"],
            &["2026-13-01", "2026-11-31", "11/22/2026", "Okafor"],
        ),
        (
            DetailKey::NoticeId,
            &["backup-failed", "name-changed"],
            &["", "Backup Failed", "1abc", &"a".repeat(41)],
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
