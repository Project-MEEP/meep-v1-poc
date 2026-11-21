//! Error to message mapping (FR-025, PLAN-26).

use meep_core::errors::{AddressReason, Error};
use meep_core::ids::RecordId;
use meep_core::messages::MessageId;

fn id_of(e: &Error) -> Option<MessageId> {
    e.message().map(|m| m.id)
}

#[test]
fn address_failures_map_to_their_own_messages() {
    let segments = Error::AddressInvalid {
        reason: AddressReason::WrongSegmentCount,
    };
    let segment = Error::AddressInvalid {
        reason: AddressReason::BadSegment,
    };
    assert_eq!(id_of(&segments), Some(MessageId::AddressInvalidSegments));
    assert_eq!(id_of(&segment), Some(MessageId::AddressInvalidSegment));
}

#[test]
fn write_failures_map_to_their_messages() {
    assert_eq!(id_of(&Error::StoreReadOnly), Some(MessageId::StoreReadOnly));
    assert_eq!(id_of(&Error::StoreFailed), Some(MessageId::SaveFailed));
}

#[test]
fn a_broken_chain_names_the_first_broken_entry() {
    let m = Error::ChainBroken { first_seq: 17 }.message().unwrap();
    assert_eq!(m.id, MessageId::HistoryDoesNotVerify);
    assert_eq!(m.args, vec![("seq", "17".to_string())]);
}

#[test]
fn failures_the_calling_feature_words_have_no_foundation_message() {
    for e in [
        Error::Stale,
        Error::NotFound,
        Error::Unreadable {
            id: RecordId::new(),
        },
        Error::KindExists,
        Error::UnknownKind,
        Error::DetailRefused {
            key: "x".to_string(),
        },
        Error::RecordKeyDestroyed,
    ] {
        assert_eq!(id_of(&e), None, "{e}");
    }
}

#[test]
fn developer_error_text_names_the_record_but_never_a_key() {
    let id = RecordId::new();
    let text = Error::Unreadable { id }.to_string();
    assert!(text.contains(&id.to_string()));
    assert!(!text.to_lowercase().contains("key"));
}
