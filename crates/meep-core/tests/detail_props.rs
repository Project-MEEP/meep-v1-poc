//! Property tests over stored detail text (S6, LOG-01): whatever text arrives, `from_stored`
//! accepts only the canonical text of a value of the key's type.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::detail::{DetailKey, DetailValue, PartRole};
use proptest::prelude::*;

proptest! {
    /// Ok only if the value stores back as exactly the text that was read.
    #[test]
    fn prop_from_stored_round_trips(s in ".*", i in 0..DetailKey::ALL.len()) {
        let key = DetailKey::ALL[i];
        if let Ok(value) = DetailValue::from_stored(key, &s) {
            prop_assert_eq!(value.to_stored(), s);
        }
    }

    /// A lowercase word that is not a part role, shaped `word[n]`, is never changed parts.
    #[test]
    fn prop_non_role_word_never_changed_parts(word in "[a-z]{1,12}", n in 0u32..100_000) {
        prop_assume!(PartRole::from_code(&word).is_err());
        let text = format!("{word}[{n}]");
        prop_assert!(DetailValue::from_stored(DetailKey::ChangedParts, &text).is_err());
    }
}
