//! Event details (contracts/change-log-entry.md, LOG-01, ERR-01 item 4).
//!
//! **Rule:** every detail value is a number, a time or date, or a member of a closed registered
//! set. There are no open vocabularies, so free text, a name and a key cannot be represented as
//! a detail at all. A detail is built from a typed [`DetailValue`]; a [`DetailKey`] names which
//! kind of value it is. A new variant is added when an event kind that needs it is registered.
//!
//! Text only appears on the way in from outside: [`DetailValue::from_stored`] (and its boolean
//! form [`DetailKey::accepts`]) is the defensive check run when a stored log entry is read back
//! or a backup is imported. It accepts only canonical text of the key's own type.

use crate::address::Address;
use crate::clock::{parse_rfc3339, to_rfc3339};
use std::fmt;
use time::{Date, Month, OffsetDateTime};

/// A closed set with stable codes: the variants, `ALL`, `as_str`, `from_code` and `Display`.
macro_rules! closed_set {
    ($(#[$meta:meta])* $name:ident [$sep:literal] { $($variant:ident => $code:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            /// Every member, in a fixed order.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The stable code stored in the log.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $code),+
                }
            }

            /// The member with this exact code. A code that fits the grammar but names no member is
            /// `UnknownCode` (newer data); anything else is `Malformed`.
            pub fn from_code(code: &str) -> Result<Self, StoredValueError> {
                match code {
                    $($code => Ok($name::$variant),)+
                    _ if code_grammar(code, $sep) => Err(StoredValueError::UnknownCode),
                    _ => Err(StoredValueError::Malformed),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

/// Why stored text was not accepted as a detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredValueError {
    /// Text in the code grammar that names no known member: written by a newer Meep, not damage.
    UnknownCode,
    /// Anything else.
    Malformed,
}

/// `^[a-z][a-z0-9<sep>]*$`: the shape of every stable code.
fn code_grammar(text: &str, sep: char) -> bool {
    let mut chars = text.chars();
    matches!(chars.next(), Some('a'..='z'))
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == sep)
}

closed_set! {
    /// Which kind of value a detail carries. Features add a variant with the kind they register.
    DetailKey ['_'] {
        FormatVersion => "format_version",
        Address => "address",
        ChangedParts => "changed_parts",
        AsOf => "as_of",
        RecordCount => "record_count",
        Date => "date",
        NoticeId => "notice_id",
    }
}

closed_set! {
    /// What a name part is (spec 001 data-model.md part `role`, ID-04); exactly these six.
    PartRole ['_'] {
        Given => "given",
        GivenAdditional => "given_additional",
        Family => "family",
        Particle => "particle",
        Prefix => "prefix",
        Suffix => "suffix",
    }
}

closed_set! {
    /// A notification-tray notice, one member per notice kind, its code the notice's Fluent
    /// message ID. The foundation lists its own; a feature adds the notices it raises.
    NoticeId ['-'] {
        BackupMade => "backup-made",
        BackupFailed => "backup-failed",
        SaveFailed => "save-failed",
    }
}

/// One detail value, typed so that only a number, a time or date, or a member of a closed set
/// can be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetailValue {
    FormatVersion(u32),
    /// A syntactically valid address. Whether it must also belong to a registered address space
    /// is the owner's open question; see the pull request.
    Address(Address),
    /// Which parts of a record changed: the part's role and its position (the part's `seq`).
    ChangedParts(Vec<(PartRole, u16)>),
    AsOf(OffsetDateTime),
    RecordCount(u32),
    Date(Date),
    NoticeId(NoticeId),
}

/// The most role and position pairs one `ChangedParts` value may carry.
const MAX_CHANGED_PARTS: usize = 20;

impl DetailValue {
    /// The key this value belongs to.
    pub fn key(&self) -> DetailKey {
        match self {
            DetailValue::FormatVersion(_) => DetailKey::FormatVersion,
            DetailValue::Address(_) => DetailKey::Address,
            DetailValue::ChangedParts(_) => DetailKey::ChangedParts,
            DetailValue::AsOf(_) => DetailKey::AsOf,
            DetailValue::RecordCount(_) => DetailKey::RecordCount,
            DetailValue::Date(_) => DetailKey::Date,
            DetailValue::NoticeId(_) => DetailKey::NoticeId,
        }
    }

    /// The canonical stored text: decimal numbers, `family[2],given[1]`, a UTC RFC 3339 time
    /// ending in `Z`, `YYYY-MM-DD`, the address as written, a notice's code.
    pub fn to_stored(&self) -> String {
        match self {
            DetailValue::FormatVersion(n) | DetailValue::RecordCount(n) => n.to_string(),
            DetailValue::Address(a) => a.as_str().to_string(),
            DetailValue::ChangedParts(parts) => parts
                .iter()
                .map(|(role, position)| format!("{}[{position}]", role.as_str()))
                .collect::<Vec<_>>()
                .join(","),
            // Formatting fails only outside the supported years; that stores empty text, which
            // `from_stored` then refuses.
            DetailValue::AsOf(t) => to_rfc3339(*t).unwrap_or_default(),
            DetailValue::Date(d) => {
                format!("{:04}-{:02}-{:02}", d.year(), u8::from(d.month()), d.day())
            }
            DetailValue::NoticeId(n) => n.as_str().to_string(),
        }
    }

    /// Read stored text back as a typed value, or say why not: `UnknownCode` for a well-formed
    /// code this version does not know (newer data), `Malformed` for anything else. Only the
    /// canonical text of a value of this key's type is accepted.
    pub fn from_stored(key: DetailKey, text: &str) -> Result<DetailValue, StoredValueError> {
        let malformed = StoredValueError::Malformed;
        let value = match key {
            DetailKey::FormatVersion => {
                DetailValue::FormatVersion(text.parse().map_err(|_| malformed)?)
            }
            DetailKey::RecordCount => {
                DetailValue::RecordCount(text.parse().map_err(|_| malformed)?)
            }
            DetailKey::Address => {
                DetailValue::Address(Address::parse(text).map_err(|_| malformed)?)
            }
            DetailKey::ChangedParts => {
                DetailValue::ChangedParts(parse_parts(text).ok_or(malformed)?)
            }
            DetailKey::AsOf => DetailValue::AsOf(parse_rfc3339(text).map_err(|_| malformed)?),
            DetailKey::Date => DetailValue::Date(parse_date(text).ok_or(malformed)?),
            DetailKey::NoticeId => DetailValue::NoticeId(NoticeId::from_code(text)?),
        };
        // Only the canonical spelling is accepted: no leading zeros, no offset, no stray text.
        if value.to_stored() == text {
            Ok(value)
        } else {
            Err(malformed)
        }
    }
}

impl DetailKey {
    /// The defensive check for stored entries and imported backups: is `text` exactly the
    /// canonical text of a value of this key's type? Typed construction never needs it.
    pub fn accepts(self, text: &str) -> bool {
        DetailValue::from_stored(self, text).is_ok()
    }
}

fn parse_parts(text: &str) -> Option<Vec<(PartRole, u16)>> {
    let pairs: Vec<&str> = text.split(',').collect();
    if pairs.is_empty() || pairs.len() > MAX_CHANGED_PARTS {
        return None;
    }
    pairs.into_iter().map(parse_part).collect()
}

fn parse_part(pair: &str) -> Option<(PartRole, u16)> {
    let (role, rest) = pair.split_once('[')?;
    let position = rest.strip_suffix(']')?;
    Some((PartRole::from_code(role).ok()?, position.parse().ok()?))
}

/// `YYYY-MM-DD`, a real calendar date.
fn parse_date(text: &str) -> Option<Date> {
    let mut parts = text.split('-');
    let (year, month, day) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() || year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return None;
    }
    let month = Month::try_from(month.parse::<u8>().ok()?).ok()?;
    Date::from_calendar_date(year.parse().ok()?, month, day.parse().ok()?).ok()
}
