//! The closed set of event detail keys (contracts/change-log-entry.md, ERR-01 item 4). A key is
//! a `Copy` value, never a string, so a refused key can be named in the diagnostic log without
//! free text. A variant is added when an event kind that uses it is registered.

use std::fmt;

macro_rules! detail_keys {
    ($($variant:ident => $code:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
