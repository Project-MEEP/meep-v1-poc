//! ERR-01 item 1: the panic hook writes a fixed code and a source location, never the payload.

// ERR-01: helpers outside #[test] functions may unwrap and expect in test code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::panic_hook::{PANIC_CODE, install};
use std::sync::Mutex;

static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn sink(line: &str) {
    SEEN.lock().unwrap().push(line.to_string());
}

#[test]
fn the_hook_writes_a_fixed_code_and_never_the_panic_message() {
    install(sink);
    let _ = std::panic::catch_unwind(|| {
        panic!("owner-content-canary-4711");
    });
    let seen = SEEN.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert!(seen[0].starts_with(PANIC_CODE), "{}", seen[0]);
    assert!(seen[0].contains("panic_hook.rs"), "{}", seen[0]);
    assert!(!seen[0].contains("canary"), "{}", seen[0]);
}
