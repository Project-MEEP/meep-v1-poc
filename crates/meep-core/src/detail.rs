//! The closed set of event detail keys (contracts/change-log-entry.md, ERR-01 item 4). A key is
//! a `Copy` value, never a string, so a refused key can be named in the diagnostic log without
//! free text. A variant is added when an event kind that uses it is registered.

use std::fmt;

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

/// The longest detail value the change log accepts (FR-014, R8).
pub const MAX_DETAIL_LEN: usize = 64;

/// Whether `value` may be stored as a log detail. A detail is a short token (an enumeration, a
/// number, a date, an address, a position such as `family[2]`), never content or key material:
/// no spaces, at most 64 characters, and nothing that looks like a key (32 or more characters
/// drawn only from the hexadecimal and base64 alphabets).
pub fn value_is_allowed(value: &str) -> bool {
    let token = value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ".:_-+[],/=".contains(c));
    let key_like = value.len() >= 32
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "+/=_-".contains(c));
    value.len() <= MAX_DETAIL_LEN && token && !key_like
}
