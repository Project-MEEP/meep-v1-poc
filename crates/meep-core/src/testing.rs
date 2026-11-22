//! In-memory fakes of the three ports (T021, BUILD-01, FOUND-01), behind the `testing` feature so
//! specs 001 and 010 can be built and tested before the real store lands. They follow the same
//! rules as the real store: an envelope on every record, addresses checked, stale updates
//! refused, delete destroys the key, an append-only log with registered kinds and no content,
//! read-only mode, and all-or-nothing writes and batches. They are test aids only: nothing is
//! encrypted, the log has no hash chain (that is the real store's, `change_log.rs`), and the
//! alias source is not the curated one of FR-048. Never in a release build (FOUND-07).

use crate::address::Address;
use crate::clock::Clock;
use crate::detail::{DetailKey, DetailValue};
use crate::errors::{AddressReason, Error};
use crate::fragment::Snapshot;
use crate::ids::{AccountId, RecordId};
use crate::ports::{
    Actor, ApplyError, Change, ChangeLog, Envelope, Event, EventKind, FoundationLog, KeyCustody,
    LogEntryView, Outcome, Provenance, Record, RecordStore,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use time::{Duration, OffsetDateTime, Time, UtcOffset};
use uuid::Uuid;

/// The foundation's own kinds and the detail keys each may carry (contracts/change-log-entry.md).
const FOUNDATION_KINDS: &[(&str, &[DetailKey])] = &[
    ("store_created", &[DetailKey::FormatVersion]),
    ("record_created", &[DetailKey::Address]),
    (
        "record_updated",
        &[DetailKey::Address, DetailKey::ChangedParts],
    ),
    ("record_deleted", &[]),
    ("backup_made", &[DetailKey::AsOf, DetailKey::RecordCount]),
    ("restored_from_rescue_backup", &[DetailKey::Date]),
    ("restored_from_backup", &[DetailKey::Date]),
    ("notice_dismissed", &[DetailKey::NoticeId]),
];

/// Letters and digits that cannot spell words or look like names (a stand-in for PLAN-28).
const ALIAS_ALPHABET: &[u8] = b"BCDFGHJKLMNPQRSTVWXZ23456789";

#[derive(Clone)]
struct Stored {
    envelope: Envelope,
    content: Vec<u8>,
    alias: String,
}

#[derive(Clone)]
struct State {
    account_id: AccountId,
    records: BTreeMap<RecordId, Stored>,
    keys: BTreeMap<RecordId, [u8; 32]>,
    destroyed: BTreeSet<RecordId>,
    kinds: BTreeMap<String, BTreeSet<DetailKey>>,
    /// Registered addresses, lowercased for the ASCII case-insensitive comparison (R16).
    addresses: BTreeSet<String>,
    log: Vec<LogEntryView>,
    read_only: bool,
}

/// One in-memory store implementing `RecordStore`, `ChangeLog`, `FoundationLog` and
/// `KeyCustody` over shared state.
pub struct FakeStore {
    clock: Arc<dyn Clock>,
    state: Mutex<State>,
}

impl FakeStore {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        let kinds = FOUNDATION_KINDS
            .iter()
            .map(|(k, d)| ((*k).to_string(), d.iter().copied().collect()))
            .collect();
        Self {
            clock,
            state: Mutex::new(State {
                account_id: AccountId::new(),
                records: BTreeMap::new(),
                keys: BTreeMap::new(),
                destroyed: BTreeSet::new(),
                kinds,
                addresses: BTreeSet::new(),
                log: Vec::new(),
                read_only: false,
            }),
        }
    }

    // A poisoned lock only means another test thread panicked; the fake holds plain data.
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The account ID every record carries (R9, FR-007).
    pub fn account_id(&self) -> AccountId {
        self.lock().account_id
    }

    /// Put the store in or out of read-only mode (FR-028).
    pub fn set_read_only(&self, read_only: bool) {
        self.lock().read_only = read_only;
    }

    /// Whether any live record still holds `needle` (a stand-in for the raw-byte scan).
    pub fn holds_bytes(&self, needle: &[u8]) -> bool {
        !needle.is_empty()
            && self
                .lock()
                .records
                .values()
                .any(|r| r.content.windows(needle.len()).any(|w| w == needle))
    }

    /// The whole log as text, to compare before and after and to search for content.
    pub fn log_text(&self) -> String {
        format!("{:?}", self.lock().log)
    }

    /// A point-in-time view for backup fragments (FR-041).
    pub fn snapshot(&self) -> FakeSnapshot {
        FakeSnapshot {
            as_of: self.clock.now(),
            state: self.lock().clone(),
        }
    }

    fn run(&self, changes: Vec<Change>) -> Result<Vec<Envelope>, ApplyError> {
        let mut guard = self.lock();
        if guard.read_only {
            return Err(ApplyError {
                index: None,
                error: Error::StoreReadOnly,
            });
        }
        let now = self.clock.now().to_offset(UtcOffset::UTC);
        let mut staged = guard.clone();
        let mut done = Vec::with_capacity(changes.len());
        for (index, change) in changes.into_iter().enumerate() {
            match staged.apply_change(change, now) {
                Ok(envelope) => done.push(envelope),
                Err(error) => {
                    return Err(ApplyError {
                        index: Some(index),
                        error,
                    });
                }
            }
        }
        *guard = staged;
        Ok(done)
    }

    fn run_one(&self, change: Change) -> Result<Envelope, Error> {
        let mut done = self.run(vec![change]).map_err(|e| e.error)?;
        done.pop().ok_or(Error::StoreFailed {
            cause: crate::errors::Cause::None,
        })
    }
}

impl State {
    fn check_registered(&self, address: &Address) -> Result<(), Error> {
        if self
            .addresses
            .contains(&address.as_str().to_ascii_lowercase())
        {
            Ok(())
        } else {
            Err(Error::AddressInvalid {
                reason: AddressReason::Unregistered,
            })
        }
    }

    fn validate_events(&self, events: &[Event]) -> Result<(), Error> {
        for event in events {
            self.validate_details(&event.kind, &event.details)?;
        }
        Ok(())
    }

    fn validate_details(&self, kind: &EventKind, details: &[DetailValue]) -> Result<(), Error> {
        let allowed = self.kinds.get(kind.as_str()).ok_or(Error::UnknownKind)?;
        for value in details {
            if let DetailValue::Address(address) = value {
                self.check_registered(address)?;
            }
            let key = value.key();
            if !allowed.contains(&key) {
                return Err(Error::DetailRefused { key });
            }
        }
        Ok(())
    }

    fn push(
        &mut self,
        ts: OffsetDateTime,
        alias: Option<&str>,
        kind: &str,
        details: Vec<DetailValue>,
    ) {
        let seq = self.log.len() as u64;
        self.log.push(LogEntryView {
            seq,
            ts,
            record_alias: alias.map(str::to_string),
            actor: Actor::Owner,
            outcome: Outcome::Ok,
            kind: EventKind::new(kind),
            details,
        });
    }

    fn push_events(&mut self, now: OffsetDateTime, alias: &str, events: Vec<Event>) {
        for event in events {
            self.push(now, Some(alias), event.kind.as_str(), event.details);
        }
    }

    fn apply_change(&mut self, change: Change, now: OffsetDateTime) -> Result<Envelope, Error> {
        match change {
            Change::Save {
                address,
                content,
                provenance,
                events,
            } => {
                let address = Address::parse(&address)?;
                self.check_registered(&address)?;
                self.validate_events(&events)?;
                let id = RecordId::new();
                let alias = new_alias();
                let envelope = Envelope {
                    id,
                    account_id: self.account_id,
                    address,
                    provenance,
                    created_at: now,
                    updated_at: now,
                };
                self.keys.insert(id, new_key());
                self.push(
                    now,
                    Some(&alias),
                    "record_created",
                    vec![DetailValue::Address(envelope.address.clone())],
                );
                self.push_events(now, &alias, events);
                self.records.insert(
                    id,
                    Stored {
                        envelope: envelope.clone(),
                        content,
                        alias,
                    },
                );
                Ok(envelope)
            }
            Change::Update {
                id,
                expected_updated_at,
                content,
                events,
            } => {
                let stored = self.records.get(&id).ok_or(Error::NotFound)?;
                if stored.envelope.updated_at != expected_updated_at {
                    return Err(Error::Stale);
                }
                self.validate_events(&events)?;
                let alias = stored.alias.clone();
                // Strictly later than before, so two updates in one instant still conflict.
                let updated_at = now.max(stored.envelope.updated_at + Duration::nanoseconds(1));
                self.push(now, Some(&alias), "record_updated", Vec::new());
                self.push_events(now, &alias, events);
                let stored = self.records.get_mut(&id).ok_or(Error::NotFound)?;
                stored.content = content;
                stored.envelope.updated_at = updated_at;
                Ok(stored.envelope.clone())
            }
            Change::Delete { id, events } => {
                let stored = self.records.get(&id).ok_or(Error::NotFound)?;
                self.validate_events(&events)?;
                let alias = stored.alias.clone();
                let last = stored.envelope.clone();
                // Only the date is kept for a deletion (R8, FR-048).
                let date_only = now.replace_time(Time::MIDNIGHT);
                self.push(date_only, Some(&alias), "record_deleted", Vec::new());
                self.push_events(now, &alias, events);
                self.records.remove(&id);
                self.keys.remove(&id);
                self.destroyed.insert(id);
                Ok(last)
            }
        }
    }
}

fn new_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    key[..16].copy_from_slice(Uuid::new_v4().as_bytes());
    key[16..].copy_from_slice(Uuid::new_v4().as_bytes());
    key
}

fn new_alias() -> String {
    Uuid::new_v4()
        .as_bytes()
        .iter()
        .take(8)
        .map(|b| char::from(ALIAS_ALPHABET[usize::from(*b) % ALIAS_ALPHABET.len()]))
        .collect()
}

impl RecordStore for FakeStore {
    fn save(
        &self,
        address: &str,
        content: Vec<u8>,
        provenance: Provenance,
        events: Vec<Event>,
    ) -> Result<Envelope, Error> {
        self.run_one(Change::Save {
            address: address.to_string(),
            content,
            provenance,
            events,
        })
    }

    fn update(
        &self,
        id: RecordId,
        expected_updated_at: OffsetDateTime,
        content: Vec<u8>,
        events: Vec<Event>,
    ) -> Result<Envelope, Error> {
        self.run_one(Change::Update {
            id,
            expected_updated_at,
            content,
            events,
        })
    }

    fn read(&self, id: RecordId) -> Result<Record, Error> {
        let state = self.lock();
        let stored = state.records.get(&id).ok_or(Error::NotFound)?;
        if !state.keys.contains_key(&id) {
            return Err(Error::Unreadable { id });
        }
        Ok(Record {
            envelope: stored.envelope.clone(),
            content: stored.content.clone(),
        })
    }

    fn list(&self, address_prefix: Option<&str>) -> Vec<Envelope> {
        self.lock()
            .records
            .values()
            .filter(|r| address_prefix.is_none_or(|p| r.envelope.address.has_prefix(p)))
            .map(|r| r.envelope.clone())
            .collect()
    }

    fn delete(&self, id: RecordId, events: Vec<Event>) -> Result<(), Error> {
        self.run_one(Change::Delete { id, events }).map(|_| ())
    }

    fn apply(&self, changes: Vec<Change>) -> Result<Vec<Envelope>, ApplyError> {
        self.run(changes)
    }
}

impl ChangeLog for FakeStore {
    fn register_address(&self, address: Address) -> Result<(), Error> {
        let mut state = self.lock();
        if !state
            .addresses
            .insert(address.as_str().to_ascii_lowercase())
        {
            return Err(Error::AddressInvalid {
                reason: AddressReason::AlreadyRegistered,
            });
        }
        Ok(())
    }

    fn register_kind(&self, kind: EventKind, allowed_details: &[DetailKey]) -> Result<(), Error> {
        let mut state = self.lock();
        if state.kinds.contains_key(kind.as_str()) {
            return Err(Error::KindExists);
        }
        state.kinds.insert(
            kind.as_str().to_string(),
            allowed_details.iter().copied().collect(),
        );
        Ok(())
    }

    fn verify(&self) -> Result<u64, Error> {
        let state = self.lock();
        for (i, entry) in state.log.iter().enumerate() {
            if entry.seq != i as u64 {
                return Err(Error::ChainBroken {
                    first_seq: i as u64,
                });
            }
        }
        Ok(state.log.len() as u64)
    }

    fn read_log(&self, range: Range<u64>) -> Vec<LogEntryView> {
        self.lock()
            .log
            .iter()
            .filter(|e| range.contains(&e.seq))
            .cloned()
            .collect()
    }
}

impl FoundationLog for FakeStore {
    fn append(
        &self,
        kind: EventKind,
        record: Option<RecordId>,
        details: Vec<DetailValue>,
    ) -> Result<(), Error> {
        let mut state = self.lock();
        if state.read_only {
            return Err(Error::StoreReadOnly);
        }
        state.validate_details(&kind, &details)?;
        let alias = record
            .and_then(|id| state.records.get(&id))
            .map(|r| r.alias.clone());
        let now = self.clock.now().to_offset(UtcOffset::UTC);
        state.push(now, alias.as_deref(), kind.as_str(), details);
        Ok(())
    }
}

impl KeyCustody for FakeStore {
    fn purpose_value(
        &self,
        record: RecordId,
        purpose: &str,
        input: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let state = self.lock();
        let Some(key) = state.keys.get(&record) else {
            return Err(if state.destroyed.contains(&record) {
                Error::RecordKeyDestroyed
            } else {
                Error::NotFound
            });
        };
        let mut hash = Sha256::new();
        hash.update(key);
        hash.update((purpose.len() as u64).to_be_bytes());
        hash.update(purpose.as_bytes());
        hash.update(input);
        Ok(hash.finalize().to_vec())
    }

    fn destroy_record_key(&self, record: RecordId) -> Result<(), Error> {
        let mut state = self.lock();
        if state.read_only {
            return Err(Error::StoreReadOnly);
        }
        if state.keys.remove(&record).is_some() {
            state.destroyed.insert(record);
            Ok(())
        } else if state.destroyed.contains(&record) {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }
}

/// A frozen copy of the fake store for `BackupFragment::export` (FR-041).
pub struct FakeSnapshot {
    as_of: OffsetDateTime,
    state: State,
}

impl Snapshot for FakeSnapshot {
    fn as_of(&self) -> OffsetDateTime {
        self.as_of
    }

    fn list(&self, address_prefix: Option<&str>) -> Vec<Envelope> {
        self.state
            .records
            .values()
            .filter(|r| address_prefix.is_none_or(|p| r.envelope.address.has_prefix(p)))
            .map(|r| r.envelope.clone())
            .collect()
    }

    fn read(&self, id: RecordId) -> Result<Record, Error> {
        let stored = self.state.records.get(&id).ok_or(Error::NotFound)?;
        if !self.state.keys.contains_key(&id) {
            return Err(Error::Unreadable { id });
        }
        Ok(Record {
            envelope: stored.envelope.clone(),
            content: stored.content.clone(),
        })
    }
}
