//! The closed set of event detail keys (contracts/change-log-entry.md, ERR-01 item 4). A key is
//! a `Copy` value, never a string, so a refused key can be named in the diagnostic log without
//! free text. A variant is added when an event kind that uses it is registered.

use crate::address::Address;
use crate::clock::parse_rfc3339;
use std::fmt;
use time::{Date, Month};

macro_rules! detail_keys {
    ($($variant:ident => $code:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum DetailKey {
            $($variant),+
        }

        impl DetailKey {
            /// Every key, in the order of the change-log contract.
            pub const ALL: &'static [DetailKey] = &[$(DetailKey::$variant),+];

            /// The stable code stored in the log.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(DetailKey::$variant => $code),+
                }
            }
        }
    };
}

detail_keys! {
    FormatVersion => "format_version",
    Address => "address",
    ChangedParts => "changed_parts",
    AsOf => "as_of",
    RecordCount => "record_count",
    Date => "date",
    NoticeId => "notice_id",
}

impl fmt::Display for DetailKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl DetailKey {
    /// Whether `value` has the shape this key's value must have (FR-014, R8). Each key carries
    /// one kind of value: a number, a date, an address, a list of `role[position]` pairs, an
    /// identifier. A name, a key or free text fits none of them, so it cannot pass as a detail
    /// merely by being short or having no spaces. A new key comes with its own shape.
    pub fn accepts(self, value: &str) -> bool {
        match self {
            DetailKey::FormatVersion => is_number(value, 6),
            DetailKey::RecordCount => is_number(value, 9),
            DetailKey::Address => value.len() <= 200 && Address::parse(value).is_ok(),
            DetailKey::ChangedParts => is_changed_parts(value),
            DetailKey::AsOf => value.len() <= 40 && parse_rfc3339(value).is_ok(),
            DetailKey::Date => is_date(value),
            DetailKey::NoticeId => is_identifier(value, 40),
        }
    }
}

fn is_number(value: &str, max_len: usize) -> bool {
    !value.is_empty() && value.len() <= max_len && value.bytes().all(|b| b.is_ascii_digit())
}

/// `YYYY-MM-DD`, a real calendar date.
fn is_date(value: &str) -> bool {
    let parts: Vec<&str> = value.split('-').collect();
    let [year, month, day] = parts.as_slice() else {
        return false;
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return false;
    }
    let numbers = (year.parse::<i32>(), month.parse::<u8>(), day.parse::<u8>());
    let (Ok(y), Ok(m), Ok(d)) = numbers else {
        return false;
    };
    Month::try_from(m)
        .ok()
        .is_some_and(|m| Date::from_calendar_date(y, m, d).is_ok())
}

/// One to twenty `role[position]` pairs separated by commas: a lowercase role of up to sixteen
/// letters and hyphens, and a position of one to three digits (`family[2],given[1]`).
fn is_changed_parts(value: &str) -> bool {
    let pairs: Vec<&str> = value.split(',').collect();
    !pairs.is_empty() && pairs.len() <= 20 && pairs.iter().all(|p| is_part(p))
}

fn is_part(pair: &str) -> bool {
    let Some((role, rest)) = pair.split_once('[') else {
        return false;
    };
    let Some(position) = rest.strip_suffix(']') else {
        return false;
    };
    let role_ok = role.len() <= 16
        && role.starts_with(|c: char| c.is_ascii_lowercase())
        && role.chars().all(|c| c.is_ascii_lowercase() || c == '-');
    role_ok && is_number(position, 3)
}

/// A lowercase kebab identifier that starts with a letter (`backup-failed`).
fn is_identifier(value: &str, max_len: usize) -> bool {
    value.len() <= max_len
        && value.starts_with(|c: char| c.is_ascii_lowercase())
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
