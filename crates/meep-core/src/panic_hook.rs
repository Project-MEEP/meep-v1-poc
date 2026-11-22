//! The panic hook (ERR-01 item 1). A panic writes one fixed code and the source location, and
//! nothing from the panic itself: the payload and message may hold owner content, so they are
//! never read. The sink is where the line goes (the boot log, LOG-01 item 3, once it exists).

use std::panic;

/// The fixed code every panic line starts with.
pub const PANIC_CODE: &str = "E-PANIC";

/// Install the hook. `sink` receives `"E-PANIC <crate path>:<line>"`.
pub fn install(sink: fn(&str)) {
    panic::set_hook(Box::new(move |info| {
        let line = match info.location() {
            Some(loc) => format!("{PANIC_CODE} {}:{}", trim_path(loc.file()), loc.line()),
            None => PANIC_CODE.to_string(),
        };
        sink(&line);
    }));
}

/// Keep only the in-repository part of a source path, so no machine path reaches a log.
fn trim_path(file: &str) -> &str {
    let file = file.trim_start_matches(['\\', '/']);
    for marker in ["crates\\", "crates/", "tests\\", "tests/"] {
        if let Some(i) = file.rfind(marker) {
            return &file[i..];
        }
    }
    file.rsplit(['\\', '/']).next().unwrap_or(file)
}
