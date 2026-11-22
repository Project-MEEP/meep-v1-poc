//! Injectable clock (PLAN-24): UTC time, RFC 3339 text, and a fixed clock for tests.
//! Used for store_header.created_at, records.created_at, records.updated_at and change_log.ts.

use std::sync::{Mutex, PoisonError};
use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

/// A source of the current time. Production code takes a `&dyn Clock` so tests can control it.
pub trait Clock: Send + Sync {
    fn now(&self) -> OffsetDateTime;
}

/// RFC 3339 text in UTC with a `Z` suffix, whatever offset the value carries.
pub fn to_rfc3339(t: OffsetDateTime) -> Result<String, time::error::Format> {
    t.to_offset(UtcOffset::UTC).format(&Rfc3339)
}

pub fn parse_rfc3339(s: &str) -> Result<OffsetDateTime, time::error::Parse> {
    OffsetDateTime::parse(s, &Rfc3339)
}

/// The real clock: the operating system's time, in UTC.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

/// A clock that holds one time until told otherwise, including backwards (quickstart 28).
pub struct FixedClock(Mutex<OffsetDateTime>);

impl FixedClock {
    pub fn new(t: OffsetDateTime) -> Self {
        Self(Mutex::new(t))
    }

    pub fn set(&self, t: OffsetDateTime) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = t;
    }
}

impl Clock for FixedClock {
    fn now(&self) -> OffsetDateTime {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;

    fn at(s: &str) -> OffsetDateTime {
        OffsetDateTime::parse(s, &Rfc3339).expect("fixture time")
    }

    #[test]
    fn system_clock_is_utc_and_close_to_real_time() {
        let now = SystemClock.now();
        assert_eq!(now.offset(), UtcOffset::UTC);
        let drift = (now - OffsetDateTime::now_utc()).abs();
        assert!(drift < Duration::seconds(5));
    }

    #[test]
    fn rfc3339_text_is_utc_with_a_z_suffix_and_round_trips() {
        let t = at("2026-11-21T04:07:03Z");
        let text = to_rfc3339(t).unwrap();
        assert_eq!(text, "2026-11-21T04:07:03Z");
        assert_eq!(parse_rfc3339(&text).unwrap(), t);
    }

    #[test]
    fn rfc3339_text_normalizes_other_offsets_to_utc() {
        let t = at("2026-11-21T09:07:03+05:00");
        assert_eq!(to_rfc3339(t).unwrap(), "2026-11-21T04:07:03Z");
    }

    #[test]
    fn rfc3339_keeps_sub_second_precision() {
        let t = at("2026-11-21T04:07:03.250Z");
        let text = to_rfc3339(t).unwrap();
        assert_eq!(parse_rfc3339(&text).unwrap(), t);
    }

    #[test]
    fn parse_refuses_text_that_is_not_rfc3339() {
        assert!(parse_rfc3339("21/11/2026 04:07").is_err());
        assert!(parse_rfc3339("").is_err());
    }

    #[test]
    fn fixed_clock_holds_its_time_and_can_move_backwards() {
        let clock = FixedClock::new(at("2026-11-21T04:07:03Z"));
        assert_eq!(clock.now(), at("2026-11-21T04:07:03Z"));
        assert_eq!(clock.now(), clock.now());
        clock.set(at("2026-11-20T00:00:00Z"));
        assert_eq!(clock.now(), at("2026-11-20T00:00:00Z"));
    }

    #[test]
    fn a_clock_can_be_used_through_a_trait_object() {
        let fixed = FixedClock::new(at("2026-11-21T04:07:03Z"));
        let dynamic: &dyn Clock = &fixed;
        assert_eq!(dynamic.now(), at("2026-11-21T04:07:03Z"));
    }
}
