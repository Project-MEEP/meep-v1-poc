//! The shared address checker (R16, FR-008): exactly five dot-separated segments, each starting
//! with an ASCII letter and holding only ASCII letters, digits and hyphens. The written case is
//! kept for display; comparison ignores case.

use crate::errors::{AddressReason, Error};
use std::hash::{Hash, Hasher};

const SEGMENTS: usize = 5;

/// A validated address such as `Alpha.Meep.Personal.Identity.Names`.
#[derive(Debug, Clone)]
pub struct Address(String);

impl Address {
    /// Check `text` and keep it as written.
    pub fn parse(text: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = text.split('.').collect();
        if parts.len() != SEGMENTS {
            return Err(refused(AddressReason::WrongSegmentCount));
        }
        if !parts.iter().all(|p| valid_segment(p)) {
            return Err(refused(AddressReason::BadSegment));
        }
        Ok(Self(text.to_string()))
    }

    /// The address as written.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this address starts with `prefix`, compared segment by segment, ignoring case.
    /// The prefix may have one to five segments.
    pub fn has_prefix(&self, prefix: &str) -> bool {
        let mut mine = self.0.split('.');
        prefix
            .split('.')
            .all(|p| mine.next().is_some_and(|m| m.eq_ignore_ascii_case(p)))
    }
}

fn refused(reason: AddressReason) -> Error {
    Error::AddressInvalid { reason }
}

fn valid_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-')
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for Address {}

impl Hash for Address {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.0.bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
