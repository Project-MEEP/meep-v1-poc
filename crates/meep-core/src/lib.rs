//! Domain types and ports for the foundation (spec 002). No I/O.
#![forbid(unsafe_code)]

pub mod address;
pub mod clock;
pub mod detail;
pub mod errors;
pub mod failpoint;
pub mod fragment;
pub mod ids;
pub mod messages;
pub mod panic_hook;
pub mod ports;
#[cfg(feature = "testing")]
pub mod testing;
