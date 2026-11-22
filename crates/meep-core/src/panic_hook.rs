//! The panic hook (ERR-01 item 1). A panic writes one fixed code and a source location, and
//! nothing from the panic itself: the payload and message may hold owner content, so they are
//! never read. The sink is where the line goes (the boot log, LOG-01 item 3, once it exists).
//!
//! Only a repository path (`crates/...` or `tests/...`) is ever shown. A panic inside a
//! dependency or the standard library has a path under the cargo registry or the toolchain,
//! which holds the operating-system user name, so it is reported as `<dep>` with no path or line.

use std::panic;

/// The fixed code every panic line starts with.
pub const PANIC_CODE: &str = "E-PANIC";

/// What a dependency panic is reported as.
const DEPENDENCY: &str = "<dep>";

/// Install the hook. `sink` receives the line built by [`describe`].
pub fn install(sink: fn(&str)) {
    panic::set_hook(Box::new(move |info| {
        let line = match info.location() {
            Some(loc) => describe(loc.file(), loc.line()),
            None => format!("{PANIC_CODE} {DEPENDENCY}"),
        };
        sink(&line);
    }));
}

/// The log line for a panic at `file:line`: the repository path with forward slashes, or
/// `<dep>` alone when the path has no repository prefix.
pub fn describe(file: &str, line: u32) -> String {
    let path = file.replace('\\', "/");
    match repository_part(&path) {
        Some(part) => format!("{PANIC_CODE} {part}:{line}"),
        None => format!("{PANIC_CODE} {DEPENDENCY}"),
    }
}

/// The part of `path` starting at a `crates/` or `tests/` directory, if it has one.
fn repository_part(path: &str) -> Option<&str> {
    ["crates/", "tests/"]
        .iter()
        .filter_map(|marker| {
            if path.starts_with(marker) {
                Some(0)
            } else {
                path.find(&format!("/{marker}")).map(|i| i + 1)
            }
        })
        .min()
        .map(|start| &path[start..])
}
