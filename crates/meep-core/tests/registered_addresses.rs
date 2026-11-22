//! Registered addresses (S2, FR-008, R6, R16): a record is saved only at an address a feature
//! registered, compared ignoring ASCII case. Needs the `testing` feature.

#![cfg(feature = "testing")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::address::Address;
use meep_core::clock::FixedClock;
use meep_core::detail::DetailValue;
use meep_core::errors::{AddressReason, Error};
use meep_core::ports::{ChangeLog, Event, EventKind, Provenance, RecordStore};
use meep_core::testing::FakeStore;
use std::sync::Arc;
use time::macros::datetime;

const NAMES: &str = "Alpha.Meep.Personal.Identity.Names";
const SURNAME: &str = "Alpha.Meep.Personal.Okafor.Names";

fn empty_store() -> FakeStore {
    FakeStore::new(Arc::new(FixedClock::new(
        datetime!(2026-11-22 08:00:00 UTC),
    )))
}

fn unregistered() -> Error {
    Error::AddressInvalid {
        reason: AddressReason::Unregistered,
    }
}

#[test]
fn save_refuses_unregistered_address() {
    let s = empty_store();
    s.register_address(Address::parse(NAMES).unwrap()).unwrap();
    let err = s
        .save(SURNAME, vec![1], Provenance::Entered, vec![])
        .unwrap_err();
    assert_eq!(err, unregistered());
    assert_eq!(err.to_string(), "E-ADDRESS-UNREGISTERED");
    assert!(s.list(None).is_empty());
    assert!(s.read_log(0..100).is_empty());
}

#[test]
fn save_accepts_registered_address_any_ascii_case() {
    let s = empty_store();
    s.register_address(Address::parse(NAMES).unwrap()).unwrap();
    let a = s.save(NAMES, vec![1], Provenance::Entered, vec![]).unwrap();
    let b = s
        .save(
            &NAMES.to_ascii_uppercase(),
            vec![2],
            Provenance::Entered,
            vec![],
        )
        .unwrap();
    // The original case is kept for display.
    assert_eq!(a.address.as_str(), NAMES);
    assert_eq!(b.address.as_str(), NAMES.to_ascii_uppercase());
}

#[test]
fn register_address_twice_is_refused() {
    let s = empty_store();
    s.register_address(Address::parse(NAMES).unwrap()).unwrap();
    let again = s.register_address(Address::parse(&NAMES.to_ascii_lowercase()).unwrap());
    assert_eq!(
        again,
        Err(Error::AddressInvalid {
            reason: AddressReason::AlreadyRegistered
        })
    );
}

#[test]
fn event_with_unregistered_address_detail_refuses_whole_call() {
    let s = empty_store();
    s.register_address(Address::parse(NAMES).unwrap()).unwrap();
    s.register_kind(
        EventKind::new("name_moved"),
        &[meep_core::detail::DetailKey::Address],
    )
    .unwrap();
    let event = Event {
        kind: EventKind::new("name_moved"),
        details: vec![DetailValue::Address(Address::parse(SURNAME).unwrap())],
    };
    let err = s
        .save(NAMES, vec![1], Provenance::Entered, vec![event])
        .unwrap_err();
    assert_eq!(err, unregistered());
    assert!(s.list(None).is_empty());
    assert!(s.read_log(0..100).is_empty());
}
