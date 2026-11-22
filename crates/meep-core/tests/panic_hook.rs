//! ERR-01 item 1: the panic hook writes a fixed code and a source location, never the payload,
//! and never a path outside the repository (a dependency's path holds the OS user name).

// ERR-01: helpers outside #[test] functions may unwrap and expect in test code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::panic_hook::{PANIC_CODE, describe, install};
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

#[test]
fn a_workspace_path_is_kept_with_either_slash() {
    assert_eq!(
        describe("crates/meep-core/src/clock.rs", 40),
        "E-PANIC crates/meep-core/src/clock.rs:40"
    );
    assert_eq!(
        describe(r"crates\meep-core\src\clock.rs", 40),
        "E-PANIC crates/meep-core/src/clock.rs:40"
    );
    assert_eq!(
        describe("tests/acceptance/src/main.rs", 3),
        "E-PANIC tests/acceptance/src/main.rs:3"
    );
}

#[test]
fn an_absolute_workspace_path_drops_everything_before_the_repository_part() {
    let line = describe(
        r"C:\Users\bob\Project-MEEP\crates\meep-core\src\clock.rs",
        9,
    );
    assert_eq!(line, "E-PANIC crates/meep-core/src/clock.rs:9");
    assert!(!line.contains("bob"));
}

#[test]
fn a_dependency_path_is_never_shown() {
    for file in [
        r"C:\Users\bob\.cargo\registry\src\index.crates.io-1234\serde-1.0.0\src\lib.rs",
        "/home/bob/.cargo/registry/src/index.crates.io-1234/serde-1.0.0/src/lib.rs",
        "/rustc/0123abcd/library/core/src/option.rs",
    ] {
        let line = describe(file, 77);
        assert_eq!(line, "E-PANIC <dep>", "{file}");
        assert!(!line.contains("bob") && !line.contains("77"), "{line}");
    }
}
