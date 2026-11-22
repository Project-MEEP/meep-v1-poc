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
fn record_failures_map_to_their_own_messages() {
    assert_eq!(id_of(&Error::Stale), Some(MessageId::RecordChanged));
    assert_eq!(id_of(&Error::NotFound), Some(MessageId::RecordNotFound));
    assert_eq!(
        id_of(&Error::Unreadable {
            id: RecordId::new()
        }),
        Some(MessageId::RecordUnreadable)
    );
    assert_eq!(
        id_of(&Error::RecordKeyDestroyed),
        Some(MessageId::RecordDeleted)
    );
}

#[test]
fn programmer_errors_share_one_generic_message_with_a_diagnostic_code() {
    let cases = [
        (Error::KindExists, "kind-exists"),
        (Error::UnknownKind, "unknown-kind"),
        (
            Error::DetailRefused {
                key: "x".to_string(),
            },
            "detail-refused",
        ),
    ];
    for (e, code) in cases {
        let m = e.message().expect("generic message");
        assert_eq!(m.id, MessageId::InternalError, "{e}");
        assert_eq!(m.args, vec![("code", code.to_string())], "{e}");
    }
}

#[test]
fn developer_error_text_names_the_record_but_never_a_key() {
    let id = RecordId::new();
    let text = Error::Unreadable { id }.to_string();
    assert!(text.contains(&id.to_string()));
    assert!(!text.to_lowercase().contains("key"));
}
