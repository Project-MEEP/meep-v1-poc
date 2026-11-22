//! Typed failures of the ports (contracts/ports.md, FR-025). Each one maps to a
//! Fluent message ID (PLAN-26); the programmer errors share one generic message.

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
    /// The owner-facing message for this failure. Programmer errors (`KindExists`, `UnknownKind`,
    /// `DetailRefused`) share the generic `internal-error` message and carry a diagnostic code.
    /// Always `Some` today; the `Option` leaves room for a failure that is never shown.
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
            Error::Stale => (MessageId::RecordChanged, Vec::new()),
            Error::NotFound => (MessageId::RecordNotFound, Vec::new()),
            Error::Unreadable { .. } => (MessageId::RecordUnreadable, Vec::new()),
            Error::RecordKeyDestroyed => (MessageId::RecordDeleted, Vec::new()),
            Error::KindExists | Error::UnknownKind | Error::DetailRefused { .. } => (
                MessageId::InternalError,
                vec![("code", self.diagnostic_code().to_string())],
            ),
        };
        Some(OwnerMessage { id, args })
    }

    /// A short, stable code for developers and support. It never holds record content or a key.
    pub fn diagnostic_code(&self) -> &'static str {
        match self {
            Error::AddressInvalid { .. } => "address-invalid",
            Error::StoreReadOnly => "store-read-only",
            Error::StoreFailed => "store-failed",
            Error::Stale => "stale",
            Error::NotFound => "not-found",
            Error::Unreadable { .. } => "unreadable",
            Error::KindExists => "kind-exists",
            Error::UnknownKind => "unknown-kind",
            Error::DetailRefused { .. } => "detail-refused",
            Error::ChainBroken { .. } => "chain-broken",
            Error::RecordKeyDestroyed => "record-key-destroyed",
        }
    }
}
