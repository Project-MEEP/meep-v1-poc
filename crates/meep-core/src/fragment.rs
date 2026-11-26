//! What each feature contributes to the owner backup (contracts/ports.md, FR-036, FR-037,
//! FR-041, FOUND-05). The foundation collects fragments; the features supply their own data,
//! schema and part of the read-me.

use crate::errors::Error;
use crate::ids::RecordId;
use crate::ports::{Envelope, Record, RecordStore};
use serde_json::Value;
use time::OffsetDateTime;

/// One consistent, read-only view of the store taken at a single moment (FR-041).
pub trait Snapshot {
    /// The moment the snapshot shows; changes after it are not in the backup (FR-042).
    fn as_of(&self) -> OffsetDateTime;

    fn list(&self, address_prefix: Option<&str>) -> Vec<Envelope>;

    fn read(&self, id: RecordId) -> Result<Record, Error>;
}

/// A problem found in imported data. The restore refusal lists every problem (FR-017). A problem
/// names its rule and where it was found, never the data itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// The stable rule ID (for example `NAMEVAL-03`), which the feature's catalog words.
    pub rule_id: String,
    /// Where it was found, as a field reference path (for example `names[2].family`).
    pub location: String,
}

/// A feature's part of the owner backup.
pub trait BackupFragment {
    /// Stable id, for example `names`.
    fn fragment_id(&self) -> &str;

    /// The feature's data, read from one snapshot.
    fn export(&self, snapshot: &dyn Snapshot) -> Result<Value, Error>;

    /// The machine-readable schema of that data (FR-036).
    fn schema(&self) -> Value;

    /// The feature's part of READ-ME-FIRST.txt, including how its fields map to standard
    /// formats where relevant (FR-037).
    fn readme_section(&self) -> String;

    /// Check imported data against the feature's own rules and return every problem found.
    fn validate(&self, data: &Value) -> Vec<Problem>;

    /// Write validated data through the record store during a restore.
    fn apply(&self, data: &Value, store: &dyn RecordStore) -> Result<(), Error>;
}
