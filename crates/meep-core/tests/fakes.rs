//! The in-memory fakes are held to the same rules as the real store (BUILD-01, FR-006, FR-008,
//! FR-014, FR-015, FOUND-08, FOUND-09, FR-058). Needs the `testing` feature.

#![cfg(feature = "testing")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::address::Address;
use meep_core::clock::{Clock, FixedClock};
use meep_core::detail::{DetailKey, DetailValue, PartRole};
use meep_core::errors::{AddressReason, Error};
use meep_core::ids::RecordId;
use meep_core::ports::{
    Change, ChangeLog, Envelope, Event, EventKind, FoundationLog, KeyCustody, Provenance,
    RecordStore,
};
use meep_core::testing::FakeStore;
use std::sync::Arc;
use time::macros::datetime;

const ADDR: &str = "Alpha.Meep.Personal.Notes.Sample";

fn store() -> (FakeStore, Arc<FixedClock>) {
    let clock = Arc::new(FixedClock::new(datetime!(2026-11-22 08:00:00 UTC)));
    let s = FakeStore::new(clock.clone());
    for a in [ADDR, "Alpha.Meep.Personal.Other.Thing"] {
        s.register_address(Address::parse(a).unwrap()).unwrap();
    }
    (s, clock)
}

fn kind(s: &str) -> EventKind {
    EventKind::new(s)
}

fn event(k: &str, details: &[DetailValue]) -> Event {
    Event {
        kind: kind(k),
        details: details.to_vec(),
    }
}

fn save(s: &FakeStore, content: &[u8]) -> Envelope {
    s.save(ADDR, content.to_vec(), Provenance::Entered, vec![])
        .unwrap()
}

#[test]
fn every_saved_record_has_a_full_envelope() {
    let (s, clock) = store();
    let e = save(&s, b"hello");
    assert_eq!(e.address.as_str(), ADDR);
    assert_eq!(e.provenance, Provenance::Entered);
    assert_eq!(e.authority(), "none");
    assert_eq!(e.created_at, clock.now());
    assert_eq!(e.updated_at, e.created_at);
    assert_eq!(e.account_id, s.account_id());
    let again = save(&s, b"other");
    assert_ne!(e.id, again.id);
    assert_eq!(e.account_id, again.account_id);
}

#[test]
fn a_bad_address_is_refused_and_nothing_is_stored() {
    let (s, _) = store();
    let err = s
        .save("Alpha.Meep.Personal", vec![1], Provenance::Entered, vec![])
        .unwrap_err();
    assert_eq!(
        err,
        Error::AddressInvalid {
            reason: AddressReason::WrongSegmentCount
        }
    );
    assert!(s.list(None).is_empty());
    assert_eq!(s.read_log(0..100).len(), 0);
}

#[test]
fn read_returns_the_content_and_list_filters_by_address_prefix() {
    let (s, _) = store();
    let a = save(&s, b"one");
    s.save(
        "Alpha.Meep.Personal.Other.Thing",
        b"two".to_vec(),
        Provenance::Imported,
        vec![],
    )
    .unwrap();
    assert_eq!(s.read(a.id).unwrap().content, b"one");
    assert_eq!(s.list(None).len(), 2);
    assert_eq!(s.list(Some("alpha.meep.personal.notes")).len(), 1);
}

#[test]
fn a_stale_update_is_refused_and_a_fresh_one_lands() {
    let (s, clock) = store();
    let e = save(&s, b"v1");
    clock.set(datetime!(2026-11-22 08:05:00 UTC));
    let e2 = s
        .update(e.id, e.updated_at, b"v2".to_vec(), vec![])
        .unwrap();
    assert!(e2.updated_at > e.updated_at);
    // The first reader still holds the old timestamp.
    assert_eq!(
        s.update(e.id, e.updated_at, b"v3".to_vec(), vec![]),
        Err(Error::Stale)
    );
    assert_eq!(s.read(e.id).unwrap().content, b"v2");
}

#[test]
fn two_updates_in_the_same_instant_still_count_as_stale() {
    let (s, _) = store();
    let e = save(&s, b"v1");
    s.update(e.id, e.updated_at, b"v2".to_vec(), vec![])
        .unwrap();
    assert_eq!(
        s.update(e.id, e.updated_at, b"v3".to_vec(), vec![]),
        Err(Error::Stale)
    );
}

#[test]
fn update_and_delete_of_a_missing_record_are_not_found() {
    let (s, clock) = store();
    let id = RecordId::new();
    assert_eq!(
        s.update(id, clock.now(), vec![], vec![]),
        Err(Error::NotFound)
    );
    assert_eq!(s.delete(id, vec![]), Err(Error::NotFound));
    assert_eq!(s.read(id).unwrap_err(), Error::NotFound);
}

#[test]
fn delete_makes_the_content_unreadable_and_destroys_the_key() {
    let (s, _) = store();
    let e = save(&s, b"secret-sentinel");
    s.purpose_value(e.id, "fingerprint", b"x").unwrap();
    s.delete(e.id, vec![]).unwrap();
    assert_eq!(s.read(e.id).unwrap_err(), Error::NotFound);
    assert_eq!(
        s.purpose_value(e.id, "fingerprint", b"x"),
        Err(Error::RecordKeyDestroyed)
    );
    assert!(!s.holds_bytes(b"secret-sentinel"));
}

#[test]
fn a_destroyed_key_without_a_delete_makes_the_record_unreadable() {
    let (s, _) = store();
    let e = save(&s, b"content");
    s.destroy_record_key(e.id).unwrap();
    assert_eq!(s.read(e.id).unwrap_err(), Error::Unreadable { id: e.id });
}

#[test]
fn purpose_values_are_keyed_per_record_and_per_purpose() {
    let (s, _) = store();
    let a = save(&s, b"same");
    let b = save(&s, b"same");
    let va = s.purpose_value(a.id, "fingerprint", b"input").unwrap();
    assert_eq!(va, s.purpose_value(a.id, "fingerprint", b"input").unwrap());
    assert_ne!(va, s.purpose_value(b.id, "fingerprint", b"input").unwrap());
    assert_ne!(va, s.purpose_value(a.id, "other", b"input").unwrap());
    assert_eq!(
        s.purpose_value(RecordId::new(), "fingerprint", b"x"),
        Err(Error::NotFound)
    );
}

#[test]
fn the_log_is_append_only_and_holds_no_content() {
    let (s, _) = store();
    let e = save(&s, b"never-in-the-log");
    s.update(e.id, e.updated_at, b"also-never".to_vec(), vec![])
        .unwrap();
    s.delete(e.id, vec![]).unwrap();
    let log = s.read_log(0..100);
    let kinds: Vec<&str> = log.iter().map(|l| l.kind.as_str()).collect();
    assert_eq!(
        kinds,
        ["record_created", "record_updated", "record_deleted"]
    );
    assert_eq!(log.iter().map(|l| l.seq).collect::<Vec<_>>(), [0, 1, 2]);
    assert_eq!(s.verify(), Ok(3));
    assert!(!s.log_text().contains("never"));
    // Entries name the record by alias only, never by id.
    let alias = log[0].record_alias.clone().unwrap();
    assert!(!s.log_text().contains(&e.id.to_string()));
    assert_eq!(log[2].record_alias.as_deref(), Some(alias.as_str()));
}

#[test]
fn a_delete_entry_carries_only_the_date() {
    let (s, _) = store();
    let e = save(&s, b"x");
    s.delete(e.id, vec![]).unwrap();
    let last = s.read_log(0..100).pop().unwrap();
    assert_eq!(last.ts, datetime!(2026-11-22 00:00:00 UTC));
    assert!(last.details.is_empty());
}

#[test]
fn feature_events_land_with_the_record_change_in_the_given_order() {
    let (s, _) = store();
    s.register_kind(kind("name_added"), &[DetailKey::ChangedParts])
        .unwrap();
    s.register_kind(kind("name_noted"), &[]).unwrap();
    s.save(
        ADDR,
        b"x".to_vec(),
        Provenance::Entered,
        vec![
            event(
                "name_added",
                &[DetailValue::ChangedParts(vec![(PartRole::Family, 2)])],
            ),
            event("name_noted", &[]),
        ],
    )
    .unwrap();
    let kinds: Vec<String> = s
        .read_log(0..100)
        .iter()
        .map(|l| l.kind.as_str().to_string())
        .collect();
    assert_eq!(kinds, ["record_created", "name_added", "name_noted"]);
}

#[test]
fn an_invalid_event_fails_the_whole_call_and_changes_nothing() {
    let (s, _) = store();
    s.register_kind(kind("name_added"), &[DetailKey::ChangedParts])
        .unwrap();
    let before = s.log_text();
    // Unregistered kind.
    assert_eq!(
        s.save(ADDR, vec![], Provenance::Entered, vec![event("nope", &[])]),
        Err(Error::UnknownKind)
    );
    // Detail key not registered for the kind.
    assert_eq!(
        s.save(
            ADDR,
            vec![],
            Provenance::Entered,
            vec![event(
                "name_added",
                &[DetailValue::AsOf(datetime!(2026-11-22 08:00:00 UTC))]
            )]
        ),
        Err(Error::DetailRefused {
            key: DetailKey::AsOf
        })
    );
    // Free text, a name or a key cannot be built as a detail at all: details are typed values
    // (detail_key.rs holds the shape tests and the surname canary), so nothing else to refuse.
    assert!(s.list(None).is_empty());
    assert_eq!(s.log_text(), before);
}

#[test]
fn registering_a_kind_twice_fails() {
    let (s, _) = store();
    s.register_kind(kind("name_added"), &[]).unwrap();
    assert_eq!(
        s.register_kind(kind("name_added"), &[]),
        Err(Error::KindExists)
    );
    assert_eq!(
        s.register_kind(kind("record_created"), &[]),
        Err(Error::KindExists)
    );
}

#[test]
fn foundation_append_refuses_unknown_kinds_and_bad_details() {
    let (s, _) = store();
    assert_eq!(
        s.append(kind("nope"), None, vec![]),
        Err(Error::UnknownKind)
    );
    assert_eq!(
        s.append(
            kind("record_created"),
            None,
            vec![DetailValue::AsOf(datetime!(2026-11-22 08:00:00 UTC))]
        ),
        Err(Error::DetailRefused {
            key: DetailKey::AsOf
        })
    );
}

#[test]
fn a_read_only_store_refuses_every_write() {
    let (s, _) = store();
    let e = save(&s, b"x");
    s.set_read_only(true);
    assert_eq!(
        s.save(ADDR, vec![], Provenance::Entered, vec![]),
        Err(Error::StoreReadOnly)
    );
    assert_eq!(
        s.update(e.id, e.updated_at, vec![], vec![]),
        Err(Error::StoreReadOnly)
    );
    assert_eq!(s.delete(e.id, vec![]), Err(Error::StoreReadOnly));
    assert_eq!(
        s.append(kind("record_created"), None, vec![]),
        Err(Error::StoreReadOnly)
    );
    assert_eq!(s.apply(vec![]).unwrap_err().error, Error::StoreReadOnly);
    assert_eq!(s.read(e.id).unwrap().content, b"x");
}

fn batch_save(content: &[u8], events: Vec<Event>) -> Change {
    Change::Save {
        address: ADDR.to_string(),
        content: content.to_vec(),
        provenance: Provenance::Entered,
        events,
    }
}

#[test]
fn a_batch_lands_together_and_keeps_the_log_order() {
    let (s, _) = store();
    s.register_kind(kind("name_added"), &[]).unwrap();
    let old = save(&s, b"old-legal");
    let before = s.read_log(0..100).len() as u64;
    let done = s
        .apply(vec![
            Change::Update {
                id: old.id,
                expected_updated_at: old.updated_at,
                content: b"retired".to_vec(),
                events: vec![event("name_added", &[])],
            },
            batch_save(b"new-legal", vec![]),
        ])
        .unwrap();
    assert_eq!(done.len(), 2);
    let kinds: Vec<String> = s
        .read_log(before..100)
        .iter()
        .map(|l| l.kind.as_str().to_string())
        .collect();
    assert_eq!(kinds, ["record_updated", "name_added", "record_created"]);
    assert_eq!(s.read(old.id).unwrap().content, b"retired");
    assert_eq!(s.list(None).len(), 2);
}

#[test]
fn one_failing_change_leaves_every_record_and_the_log_unchanged() {
    let (s, _) = store();
    let a = save(&s, b"a");
    let log_before = s.log_text();
    let result = s.apply(vec![
        batch_save(b"new", vec![]),
        Change::Update {
            id: a.id,
            expected_updated_at: a.updated_at,
            content: b"a2".to_vec(),
            events: vec![],
        },
        Change::Update {
            id: a.id,
            expected_updated_at: datetime!(2000-01-01 00:00:00 UTC),
            content: b"stale".to_vec(),
            events: vec![],
        },
    ]);
    let err = result.unwrap_err();
    assert_eq!(err.index, Some(2));
    assert_eq!(err.error, Error::Stale);
    assert_eq!(s.list(None).len(), 1);
    assert_eq!(s.read(a.id).unwrap().content, b"a");
    assert_eq!(s.log_text(), log_before);
}

#[test]
fn a_bad_event_in_a_batch_names_its_position_and_changes_nothing() {
    let (s, _) = store();
    let before = s.log_text();
    let err = s
        .apply(vec![
            batch_save(b"a", vec![]),
            batch_save(b"b", vec![event("nope", &[])]),
        ])
        .unwrap_err();
    assert_eq!((err.index, err.error), (Some(1), Error::UnknownKind));
    assert!(s.list(None).is_empty());
    assert_eq!(s.log_text(), before);
}

#[test]
fn a_batch_of_one_behaves_like_the_single_call() {
    let (s, _) = store();
    let done = s.apply(vec![batch_save(b"x", vec![])]).unwrap();
    assert_eq!(done.len(), 1);
    assert_eq!(s.read(done[0].id).unwrap().content, b"x");
}

/// Codex P2: a value that cannot survive its own stored round trip is refused at the port.
#[test]
fn changed_parts_outside_one_to_twenty_pairs_are_refused() {
    let (s, _) = store();
    s.register_kind(kind("name_added"), &[DetailKey::ChangedParts])
        .unwrap();
    for parts in [vec![], vec![(PartRole::Family, 1); 21]] {
        assert_eq!(
            s.append(
                kind("name_added"),
                None,
                vec![DetailValue::ChangedParts(parts)]
            ),
            Err(Error::DetailRefused {
                key: DetailKey::ChangedParts
            })
        );
    }
    assert!(
        s.append(
            kind("name_added"),
            None,
            vec![DetailValue::ChangedParts(vec![(PartRole::Family, 1); 20])]
        )
        .is_ok()
    );
}
