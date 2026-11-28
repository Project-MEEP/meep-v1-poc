//! Error to message mapping and the stable diagnostic code (FR-025, PLAN-26, ERR-01).

// ERR-01: helpers outside #[test] functions may unwrap and expect in test code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::detail::DetailKey;
use meep_core::errors::{AddressReason, Cause, Error};
use meep_core::ids::RecordId;
use meep_core::messages::MessageId;
use std::collections::BTreeSet;
use std::io::ErrorKind;

fn id_of(e: &Error) -> Option<MessageId> {
    e.message().map(|m| m.id)
}

// 1315#S9.2: these matches have no wildcard arm on purpose. Adding a variant to `Error` or
// `AddressReason` fails to compile here until it is numbered and given its code, and a test fails
// until `every_error()` has a sample of it. Variant counts are read from `src/errors.rs`.

fn ordinal(e: &Error) -> usize {
    match e {
        Error::AddressInvalid { .. } => 0,
        Error::StoreReadOnly => 1,
        Error::StoreFailed { .. } => 2,
        Error::Stale => 3,
        Error::NotFound => 4,
        Error::Unreadable { .. } => 5,
        Error::KindExists => 6,
        Error::UnknownKind => 7,
        Error::DetailRefused { .. } => 8,
        Error::ChainBroken { .. } => 9,
        Error::RecordKeyDestroyed => 10,
        Error::StoreKeyRejected => 11,
        Error::StoreMissing => 12,
        Error::StoreExists => 13,
        Error::StoreOpenFailed { .. } => 14,
    }
}

fn reason_ordinal(r: AddressReason) -> usize {
    match r {
        AddressReason::WrongSegmentCount => 0,
        AddressReason::BadSegment => 1,
        AddressReason::Unregistered => 2,
        AddressReason::AlreadyRegistered => 3,
    }
}

/// The number of variants `src/errors.rs` declares for `header` (an enum), counted from the
/// source, so no count is kept by hand.
fn declared(header: &str) -> usize {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/errors.rs"))
        .expect("errors.rs");
    let body = src.split(header).nth(1).expect("enum header");
    let variant =
        |l: &&str| l.starts_with("    ") && l[4..].starts_with(|c: char| c.is_ascii_uppercase());
    body.lines()
        .skip(1)
        .take_while(|l| l.trim_end() != "}")
        .filter(variant)
        .count()
}

fn expected_code(e: &Error) -> &'static str {
    match e {
        Error::AddressInvalid { reason } => match reason {
            AddressReason::WrongSegmentCount => "E-ADDRESS-SEGMENT-COUNT",
            AddressReason::BadSegment => "E-ADDRESS-SEGMENT-CHARS",
            AddressReason::Unregistered => "E-ADDRESS-UNREGISTERED",
            AddressReason::AlreadyRegistered => "E-ADDRESS-ALREADY-REGISTERED",
        },
        Error::StoreReadOnly => "E-STORE-READ-ONLY",
        Error::StoreFailed { .. } => "E-STORE-WRITE-FAILED",
        Error::Stale => "E-RECORD-STALE",
        Error::NotFound => "E-RECORD-NOT-FOUND",
        Error::Unreadable { .. } => "E-RECORD-UNREADABLE",
        Error::KindExists => "E-KIND-EXISTS",
        Error::UnknownKind => "E-KIND-UNKNOWN",
        Error::DetailRefused { .. } => "E-DETAIL-REFUSED",
        Error::ChainBroken { .. } => "E-HISTORY-UNVERIFIED",
        Error::RecordKeyDestroyed => "E-RECORD-DELETED",
        Error::StoreKeyRejected => "E-STORE-KEY-REJECTED",
        Error::StoreMissing => "E-STORE-MISSING",
        Error::StoreExists => "E-STORE-EXISTS",
        Error::StoreOpenFailed { .. } => "E-STORE-OPEN-FAILED",
    }
}

fn every_error() -> Vec<Error> {
    vec![
        Error::AddressInvalid {
            reason: AddressReason::WrongSegmentCount,
        },
        Error::AddressInvalid {
            reason: AddressReason::BadSegment,
        },
        Error::AddressInvalid {
            reason: AddressReason::Unregistered,
        },
        Error::AddressInvalid {
            reason: AddressReason::AlreadyRegistered,
        },
        Error::StoreReadOnly,
        Error::StoreFailed { cause: Cause::None },
        Error::Stale,
        Error::NotFound,
        Error::Unreadable {
            id: RecordId::new(),
        },
        Error::KindExists,
        Error::UnknownKind,
        Error::DetailRefused {
            key: DetailKey::Address,
        },
        Error::ChainBroken { first_seq: 17 },
        Error::RecordKeyDestroyed,
        Error::StoreKeyRejected,
        Error::StoreMissing,
        Error::StoreExists,
        Error::StoreOpenFailed { cause: Cause::None },
    ]
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
    assert_eq!(
        id_of(&Error::StoreFailed {
            cause: Cause::Sqlite(10)
        }),
        Some(MessageId::SaveFailed)
    );
}

/// D10, D12-A (002:T023): the store open and create failures and the owner message each shows.
#[test]
fn store_open_failures_map_to_their_messages() {
    assert_eq!(
        id_of(&Error::StoreKeyRejected),
        Some(MessageId::StoreWrongKey)
    );
    assert_eq!(
        id_of(&Error::StoreMissing),
        Some(MessageId::StoreUnreadable)
    );
    let exists = Error::StoreExists.message().unwrap();
    assert_eq!(exists.id, MessageId::InternalError);
    assert_eq!(exists.args, vec![("code", "E-STORE-EXISTS".to_string())]);
    let failed = Error::StoreOpenFailed {
        cause: Cause::Sqlite(14),
    };
    assert_eq!(id_of(&failed), Some(MessageId::StoreUnreadable));
    assert_eq!(failed.cause(), Cause::Sqlite(14));
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
        (Error::KindExists, "E-KIND-EXISTS"),
        (Error::UnknownKind, "E-KIND-UNKNOWN"),
        (
            Error::DetailRefused {
                key: DetailKey::NoticeId,
            },
            "E-DETAIL-REFUSED",
        ),
    ];
    for (e, code) in cases {
        let m = e.message().expect("generic message");
        assert_eq!(m.id, MessageId::InternalError, "{e}");
        assert_eq!(m.args, vec![("code", code.to_string())], "{e}");
    }
}

/// ERR-01 item 2: Display is the stable diagnostic code, never prose.
#[test]
fn display_is_the_diagnostic_code_for_every_variant() {
    for e in every_error() {
        assert_eq!(e.to_string(), e.diagnostic_code());
    }
}

#[test]
fn diagnostic_codes_are_uppercase_area_condition_and_unique() {
    let mut seen = BTreeSet::new();
    for e in every_error() {
        let code = e.diagnostic_code();
        assert!(code.starts_with("E-"), "{code}");
        assert!(
            code.chars()
                .all(|c| c.is_ascii_uppercase() || c == '-' || c.is_ascii_digit()),
            "{code}"
        );
        assert!(code.split('-').count() >= 3, "{code}");
        // Each variant has its own code, and the four address reasons each have a separate one.
        seen.insert(code);
    }
    assert_eq!(seen.len(), every_error().len());
}

#[test]
fn display_never_carries_a_record_id_or_a_key_name() {
    let id = RecordId::new();
    let text = Error::Unreadable { id }.to_string();
    assert!(!text.contains(&id.to_string()));
    let text = Error::DetailRefused {
        key: DetailKey::Address,
    }
    .to_string();
    assert!(!text.contains("address"));
}

/// ERR-01 item 3: the cause is content-free, Copy, and never shown in Display.
#[test]
fn a_storage_failure_carries_a_content_free_cause() {
    let cause = Cause::Io(ErrorKind::PermissionDenied);
    let copy = cause;
    let e = Error::StoreFailed { cause };
    assert_eq!(e.cause(), copy);
    assert_eq!(
        e.to_string(),
        Error::StoreFailed { cause: Cause::None }.to_string()
    );
    assert_eq!(Error::Stale.cause(), Cause::None);
    assert_eq!(
        Error::StoreFailed {
            cause: Cause::Sqlite(10)
        }
        .cause(),
        Cause::Sqlite(10)
    );
}

/// ERR-01 / NAMING.md section 2: the exact registered codes, one per sample, including every
/// address reason. `expected_code` matches without a wildcard, so a new variant or reason fails
/// to compile until its code is written down here.
#[test]
fn the_registered_codes_are_exactly_these() {
    for e in every_error() {
        assert_eq!(e.diagnostic_code(), expected_code(&e));
    }
}

/// The registry in contracts/diagnostic-codes.md lists every code exactly once.
#[test]
fn the_code_registry_lists_every_code() {
    let registry = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specs/002-foundation/contracts/diagnostic-codes.md"
    ))
    .expect("registry file");
    let mut codes = BTreeSet::new();
    for e in every_error() {
        codes.insert(e.diagnostic_code());
    }
    for code in codes {
        assert_eq!(
            registry.matches(&format!("`{code}`")).count(),
            1,
            "{code} must appear once in the registry"
        );
    }
}

/// 1315#S9.2: every `Error` variant and every address reason has a sample in `every_error()`.
/// `ordinal` and `reason_ordinal` match without a wildcard, so a new variant fails to compile
/// until it is numbered there, and fails here until it is counted and given a sample.
#[test]
fn every_error_has_a_sample_of_every_variant_and_address_reason() {
    let samples = every_error();
    let variants: BTreeSet<usize> = samples.iter().map(ordinal).collect();
    assert_eq!(
        variants,
        (0..declared("pub enum Error {")).collect::<BTreeSet<_>>()
    );
    let reasons: BTreeSet<usize> = samples
        .iter()
        .filter_map(|e| match e {
            Error::AddressInvalid { reason } => Some(reason_ordinal(*reason)),
            _ => None,
        })
        .collect();
    assert_eq!(
        reasons,
        (0..declared("pub enum AddressReason {")).collect::<BTreeSet<_>>()
    );
}
