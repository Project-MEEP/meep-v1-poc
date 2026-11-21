//! Typed failures of the ports (contracts/ports.md, FR-025). Each one that has an owner-facing
//! message maps to its Fluent ID (PLAN-26); the rest are worded by the calling feature.

use crate::ids::RecordId;
use crate::messages::{MessageId, OwnerMessage};

/// Why an address was refused (FR-008, R16).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressReason {
    /// Not exactly five dot-separated parts.
    WrongSegmentCount,
    /// A part does not start with a letter, or holds a character other than letters, digits or hyphens.
    BadSegment,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("address refused: {reason:?}")]
    AddressInvalid { reason: AddressReason },
    #[error("the store is read-only")]
    StoreReadOnly,
    #[error("the store operation failed; nothing was changed")]
    StoreFailed,
    #[error("the record changed since it was read")]
    Stale,
    #[error("no such record")]
    NotFound,
    #[error("record {id} cannot be decrypted or validated")]
    Unreadable { id: RecordId },
    #[error("the event kind is already registered")]
    KindExists,
    #[error("the event kind is not registered")]
    UnknownKind,
    #[error("the event detail key {key:?} is not allowed for this kind")]
    DetailRefused { key: String },
    #[error("the history chain breaks at entry {first_seq}")]
    ChainBroken { first_seq: u64 },
    #[error("the record key was destroyed")]
    RecordKeyDestroyed,
}

impl Error {
    /// The owner-facing message for this failure, or `None` when the foundation has no message
    /// for it and the calling feature words it (for example the stale-save message).
    pub fn message(&self) -> Option<OwnerMessage> {
        let (id, args) = match self {
            Error::AddressInvalid {
                reason: AddressReason::WrongSegmentCount,
            } => (MessageId::AddressInvalidSegments, Vec::new()),
            Error::AddressInvalid {
                reason: AddressReason::BadSegment,
            } => (MessageId::AddressInvalidSegment, Vec::new()),
            Error::StoreReadOnly => (MessageId::StoreReadOnly, Vec::new()),
            Error::StoreFailed => (MessageId::SaveFailed, Vec::new()),
            Error::ChainBroken { first_seq } => (
                MessageId::HistoryDoesNotVerify,
                vec![("seq", first_seq.to_string())],
            ),
            Error::Stale
            | Error::NotFound
            | Error::Unreadable { .. }
            | Error::KindExists
            | Error::UnknownKind
            | Error::DetailRefused { .. }
            | Error::RecordKeyDestroyed => return None,
        };
        Some(OwnerMessage { id, args })
    }
}
