//! Typed failures of the ports (contracts/ports.md, FR-025, ERR-01). Each one maps to a Fluent
//! message ID (PLAN-26) and has a stable diagnostic code. `Display` is that code, never prose, so
//! no developer text can reach the owner (R10). There are no `From` impls: a library error is
//! never converted with `?`; the storage layer maps it to a `Cause` (ERR-01 item 3).

use crate::ids::RecordId;
use crate::messages::{MessageId, OwnerMessage};
use std::fmt;

/// Why an address was refused (FR-008, R16).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressReason {
    /// Not exactly five dot-separated parts.
    WrongSegmentCount,
    /// A part does not start with a letter, or holds a character other than letters, digits or hyphens.
    BadSegment,
}

/// What went wrong underneath a storage failure, without any message text (ERR-01 item 3).
/// It goes to the diagnostic log only, never to the owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    None,
    /// The kind of an operating-system I/O failure.
    Io(std::io::ErrorKind),
    /// The extended result code of a SQLite failure.
    Sqlite(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AddressInvalid {
        reason: AddressReason,
    },
    StoreReadOnly,
    /// The store operation failed and nothing was changed.
    StoreFailed {
        cause: Cause,
    },
    /// The record changed since it was read.
    Stale,
    NotFound,
    /// The record cannot be decrypted or validated.
    Unreadable {
        id: RecordId,
    },
    KindExists,
    UnknownKind,
    /// The event detail key is not allowed for this kind.
    DetailRefused {
        key: String,
    },
    /// The history chain breaks at entry `first_seq`.
    ChainBroken {
        first_seq: u64,
    },
    /// The record key was destroyed.
    RecordKeyDestroyed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.diagnostic_code())
    }
}

impl std::error::Error for Error {}

impl Error {
    /// The owner-facing message for this failure. Programmer errors (`KindExists`, `UnknownKind`,
    /// `DetailRefused`) share the generic `internal-error` message and carry the diagnostic code.
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
            Error::StoreFailed { .. } => (MessageId::SaveFailed, Vec::new()),
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

    /// A short, stable code for developers and support, `E-<AREA>-<CONDITION>` (NAMING.md §2).
    /// Codes are permanent once shipped. It never holds record content or a key.
    pub fn diagnostic_code(&self) -> &'static str {
        match self {
            Error::AddressInvalid { .. } => "E-ADDRESS-INVALID",
            Error::StoreReadOnly => "E-STORE-READONLY",
            Error::StoreFailed { .. } => "E-STORE-FAILED",
            Error::Stale => "E-RECORD-STALE",
            Error::NotFound => "E-RECORD-NOTFOUND",
            Error::Unreadable { .. } => "E-RECORD-UNREADABLE",
            Error::KindExists => "E-KIND-EXISTS",
            Error::UnknownKind => "E-KIND-UNKNOWN",
            Error::DetailRefused { .. } => "E-DETAIL-REFUSED",
            Error::ChainBroken { .. } => "E-HISTORY-BROKEN",
            Error::RecordKeyDestroyed => "E-RECORD-DESTROYED",
        }
    }

    /// The content-free cause for the diagnostic log (ERR-01 item 3); `Cause::None` when the
    /// variant wraps no underlying failure.
    pub fn cause(&self) -> Cause {
        match self {
            Error::StoreFailed { cause } => *cause,
            _ => Cause::None,
        }
    }
}
