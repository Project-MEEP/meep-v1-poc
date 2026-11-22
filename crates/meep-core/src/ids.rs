//! Random identifiers (R9, FR-007, PLAN-24): UUID v4 from the operating system's secure random
//! source, as distinct types so a store id can never be passed where a record id is wanted.

use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    #[error("an identifier must be a lowercase or uppercase hyphenated random UUID (version 4)")]
    Invalid,
}

macro_rules! id_type {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.hyphenated())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl FromStr for $name {
            type Err = IdError;
            fn from_str(s: &str) -> Result<Self, IdError> {
                // Only the canonical hyphenated form; parse_str alone also takes braces and URNs.
                if s.len() != 36 {
                    return Err(IdError::Invalid);
                }
                let id = Uuid::parse_str(s).map_err(|_| IdError::Invalid)?;
                if id.get_version() != Some(uuid::Version::Random)
                    || id.get_variant() != uuid::Variant::RFC4122
                {
                    return Err(IdError::Invalid);
                }
                Ok(Self(id))
            }
        }
    };
}

id_type!(
    /// store_header.store_id
    StoreId
);
id_type!(
    /// store_header.account_id and the account id on every record
    AccountId
);
id_type!(
    /// records.id
    RecordId
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_ids_are_random_version_4_uuids() {
        let id = RecordId::new();
        assert_eq!(id.as_uuid().get_version_num(), 4);
        assert_eq!(id.as_uuid().get_variant(), uuid::Variant::RFC4122);
    }

    #[test]
    fn ten_thousand_new_ids_are_all_different() {
        let all: HashSet<RecordId> = (0..10_000).map(|_| RecordId::new()).collect();
        assert_eq!(all.len(), 10_000);
        assert_ne!(StoreId::new(), StoreId::new());
        assert_ne!(AccountId::new(), AccountId::new());
    }

    #[test]
    fn text_form_is_lowercase_hyphenated_36_characters() {
        let text = StoreId::new().to_string();
        assert_eq!(text.len(), 36);
        assert_eq!(text, text.to_lowercase());
        assert_eq!(text.matches('-').count(), 4);
    }

    #[test]
    fn text_form_round_trips_and_accepts_uppercase() {
        let id = AccountId::new();
        assert_eq!(id.to_string().parse::<AccountId>().unwrap(), id);
        assert_eq!(
            id.to_string().to_uppercase().parse::<AccountId>().unwrap(),
            id
        );
    }

    #[test]
    fn parsing_refuses_anything_that_is_not_a_hyphenated_v4_uuid() {
        for bad in [
            "",
            "not-an-id",
            "00000000-0000-0000-0000-000000000000",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            "0b5b3d6a8f4c4b5aa0d2c1e3f4a5b6c7",
            "{0b5b3d6a-8f4c-4b5a-a0d2-c1e3f4a5b6c7}",
            "urn:uuid:0b5b3d6a-8f4c-4b5a-a0d2-c1e3f4a5b6c7",
        ] {
            assert_eq!(bad.parse::<RecordId>(), Err(IdError::Invalid), "{bad}");
        }
    }

    #[test]
    fn debug_output_names_the_type() {
        let id = StoreId::new();
        assert!(format!("{id:?}").starts_with("StoreId("));
    }
}
