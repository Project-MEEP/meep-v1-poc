//! The shared address checker (FR-008, R16, SC-008), driven by the synthetic fixtures.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use meep_core::address::Address;
use meep_core::errors::{AddressReason, Error};
use serde_json::Value;

fn fixtures() -> Value {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/addresses.json"
    ))
    .expect("fixture");
    serde_json::from_str(&text).expect("json")
}

#[test]
fn every_valid_address_is_accepted_and_keeps_its_written_case() {
    for a in fixtures()["valid"].as_array().unwrap() {
        let text = a.as_str().unwrap();
        let parsed = Address::parse(text).unwrap();
        assert_eq!(parsed.as_str(), text);
    }
}

#[test]
fn every_invalid_address_is_refused_with_its_reason() {
    for case in fixtures()["invalid"].as_array().unwrap() {
        let text = case["address"].as_str().unwrap();
        let want = match case["reason"].as_str().unwrap() {
            "address-invalid-segments" => AddressReason::WrongSegmentCount,
            "address-invalid-segment" => AddressReason::BadSegment,
            other => panic!("unknown reason {other}"),
        };
        assert_eq!(
            Address::parse(text),
            Err(Error::AddressInvalid { reason: want }),
            "{text}"
        );
    }
}

#[test]
fn addresses_differing_only_in_case_are_equal_but_keep_their_case() {
    for pair in fixtures()["case_insensitive_pairs"].as_array().unwrap() {
        let a = Address::parse(pair[0].as_str().unwrap()).unwrap();
        let b = Address::parse(pair[1].as_str().unwrap()).unwrap();
        assert_eq!(a, b);
        assert_ne!(a.as_str(), b.as_str());
    }
}

#[test]
fn a_prefix_matches_whole_segments_ignoring_case() {
    let a = Address::parse("Alpha.Meep.Personal.Identity.Names").unwrap();
    assert!(a.has_prefix("alpha.meep.personal"));
    assert!(a.has_prefix("Alpha.Meep.Personal.Identity.Names"));
    assert!(!a.has_prefix("Alpha.Mee"));
    assert!(!a.has_prefix("Alpha.Meep.Personal.Identity.Names.Extra"));
}
