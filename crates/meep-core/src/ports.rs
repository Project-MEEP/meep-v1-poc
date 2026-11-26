//! The three ports features may call (contracts/ports.md, FOUND-01, FR-025): `RecordStore`,
//! `ChangeLog` and `KeyCustody`. Features never touch encryption, storage or the log directly,
//! and no operation here returns key material (principle 15). The foundation-internal append
//! is a separate trait, `FoundationLog`, that features do not use (FOUND-08, FR-014).

use crate::address::Address;
use crate::detail::{DetailKey, DetailValue};
use crate::errors::Error;
use crate::ids::{AccountId, RecordId};
use std::ops::Range;
use time::OffsetDateTime;

/// Where a record came from (data-model.md records.provenance, PLAN-24).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    Entered,
    Imported,
    Restored,
}

/// The standard outer wrapper of every record (FR-006). Meep claims no authority over the
/// content, so `authority` is always `none`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub id: RecordId,
    pub account_id: AccountId,
    pub address: Address,
    pub provenance: Provenance,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Envelope {
    /// The statement that Meep claims no authority over the record (FR-006).
    pub fn authority(&self) -> &'static str {
        "none"
    }
}

/// A record as read back: its envelope and its decrypted content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub envelope: Envelope,
    pub content: Vec<u8>,
}

/// The name of a kind of event, such as `record_created` or a feature's own (`name_added`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventKind(String);

impl EventKind {
    pub fn new(kind: impl Into<String>) -> Self {
        Self(kind.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One feature event to log with a record change (FOUND-08): a registered kind and details whose
/// keys are registered for that kind. Details never hold content or key material (FR-014, R8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub kind: EventKind,
    pub details: Vec<DetailValue>,
}

/// Who or what caused an entry (data-model.md change_log.actor).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Owner,
    System,
}

/// How an entry ended (data-model.md change_log.outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    Refused,
    Failed,
}

/// A change-log entry as features and the owner's History view see it. The chain hashes belong
/// to the stored entry (`change_log.rs`); `ChangeLog::verify` checks them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntryView {
    pub seq: u64,
    pub ts: OffsetDateTime,
    /// The record concerned, by alias; `None` for store-level events (FR-048).
    pub record_alias: Option<String>,
    pub actor: Actor,
    pub outcome: Outcome,
    pub kind: EventKind,
    pub details: Vec<DetailValue>,
}

/// One change inside an atomic batch (FOUND-09, FR-058).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    Save {
        address: String,
        content: Vec<u8>,
        provenance: Provenance,
        events: Vec<Event>,
    },
    Update {
        id: RecordId,
        expected_updated_at: OffsetDateTime,
        content: Vec<u8>,
        events: Vec<Event>,
    },
    Delete {
        id: RecordId,
        events: Vec<Event>,
    },
}

/// Why a batch was refused: the first failing change's error and its position, or `None` when
/// the whole batch failed (a read-only store, a failed write).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyError {
    pub index: Option<usize>,
    pub error: Error,
}

/// Saving, reading and deleting records, each in one transaction (FR-006, FR-010, FR-015).
pub trait RecordStore {
    /// Create a record with a full envelope, a fresh record key, a `record_created` entry and
    /// the calling feature's `events`, all or nothing. The address goes through the shared
    /// checker (FR-008).
    fn save(
        &self,
        address: &str,
        content: Vec<u8>,
        provenance: Provenance,
        events: Vec<Event>,
    ) -> Result<Envelope, Error>;

    /// Replace content and append `record_updated` plus the feature's `events`. Refused with
    /// `Stale` when the record changed since `expected_updated_at` (NAMES-05).
    fn update(
        &self,
        id: RecordId,
        expected_updated_at: OffsetDateTime,
        content: Vec<u8>,
        events: Vec<Event>,
    ) -> Result<Envelope, Error>;

    /// The envelope and decrypted content; `Unreadable{id}` when it cannot be decrypted.
    fn read(&self, id: RecordId) -> Result<Record, Error>;

    /// Envelopes, optionally for one feature's address prefix.
    fn list(&self, address_prefix: Option<&str>) -> Vec<Envelope>;

    /// Destroy the record key, delete the record and append `record_deleted` with only the date
    /// and the feature's `events` (FR-010, FR-011).
    fn delete(&self, id: RecordId, events: Vec<Event>) -> Result<(), Error>;

    /// Apply an ordered list of changes in one transaction: all are validated first, then all
    /// land or none do. The log gets, per change in the order given, the foundation's entry and
    /// then that change's events. A deleted record is reported by its last envelope.
    fn apply(&self, changes: Vec<Change>) -> Result<Vec<Envelope>, ApplyError>;
}

/// The tamper-evident history (FR-013, FR-014). There is no operation to change or remove an
/// entry. Features add entries only by passing `events` to the record operations.
pub trait ChangeLog {
    /// Declare an address a feature owns. A record can be saved, and an address detail logged,
    /// only at a registered address (FOUND-13). Comparison ignores ASCII case; registering the
    /// same address twice is refused.
    fn register_address(&self, address: Address) -> Result<(), Error>;

    /// Declare an event kind and the only detail keys it may carry.
    fn register_kind(&self, kind: EventKind, allowed_details: &[DetailKey]) -> Result<(), Error>;

    /// Recompute the chain from the genesis entry: the number of entries checked, or
    /// `ChainBroken{first_seq}`.
    fn verify(&self) -> Result<u64, Error>;

    /// Entries whose `seq` lies in `range`.
    fn read_log(&self, range: Range<u64>) -> Vec<LogEntryView>;
}

/// Foundation-internal appends for store-level kinds (`store_created`, `backup_made`,
/// `restored_from_rescue_backup`). Features do not call this (FOUND-08).
pub trait FoundationLog {
    fn append(
        &self,
        kind: EventKind,
        record: Option<RecordId>,
        details: Vec<DetailValue>,
    ) -> Result<(), Error>;
}

/// Keyed values tied to a record's own key. There is no operation that returns key material
/// (FR-025, principle 15).
pub trait KeyCustody {
    /// A keyed, non-reversible value for `purpose` and `input`, tied to the record's own key
    /// and meaningless once that key is destroyed (FR-012).
    fn purpose_value(
        &self,
        record: RecordId,
        purpose: &str,
        input: &[u8],
    ) -> Result<Vec<u8>, Error>;

    /// Destroy the record's key. Only `delete` and restore call it (FR-010).
    fn destroy_record_key(&self, record: RecordId) -> Result<(), Error>;
}
