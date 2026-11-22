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
