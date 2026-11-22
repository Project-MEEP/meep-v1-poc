//! Message catalog tests (R10, FR-035, FR-042, PLAN-26).

// ERR-01: helpers outside #[test] functions may unwrap and expect in test code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use meep_core::messages::MessageId;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const CATALOG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../meep-web/locales/en/foundation.ftl"
);

/// Parse `id = text` lines (every foundation message is a single line).
fn catalog() -> BTreeMap<String, String> {
    fs::read_to_string(CATALOG)
        .expect("catalog file")
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (id, text) = l.split_once(" = ").expect("`id = text` line");
            (id.to_string(), text.to_string())
        })
        .collect()
}

#[test]
fn the_catalog_has_exactly_the_ids_in_the_code() {
    let in_catalog: BTreeSet<String> = catalog().into_keys().collect();
    let in_code: BTreeSet<String> = MessageId::ALL
        .iter()
        .map(|m| m.as_str().to_string())
        .collect();
    assert_eq!(in_catalog, in_code);
}

#[test]
fn message_ids_are_lowercase_hyphenated_and_unique() {
    let mut seen = BTreeSet::new();
    for m in MessageId::ALL {
        let id = m.as_str();
        assert!(
            id.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
            "{id}"
        );
        assert!(seen.insert(id), "duplicate {id}");
    }
    assert_eq!(MessageId::ALL.len(), 30);
}

#[test]
fn the_two_owner_specified_texts_are_exact() {
    let c = catalog();
    assert_eq!(
        c["store-already-open"],
        "Meep may already be open. If it isn't, restart your computer and try again."
    );
    assert_eq!(
        c["backup-changes-after"],
        "Changes you made after { $time } aren't in this backup."
    );
}

#[test]
fn history_and_damage_are_different_messages() {
    let c = catalog();
    assert_ne!(c["history-does-not-verify"], c["store-damaged-readonly"]);
}

#[test]
fn no_message_names_an_intruder() {
    for (id, text) in catalog() {
        let lower = text.to_lowercase();
        for word in ["intruder", "hacker", "attacker"] {
            assert!(!lower.contains(word), "{id} mentions {word}");
        }
    }
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("source dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// R10: owner-visible wording lives only in the catalog. A first, partial guard: no foundation
/// source file may repeat a catalog sentence. The full check arrives with the web layer.
#[test]
fn foundation_code_does_not_repeat_catalog_wording() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    for krate in ["meep-core", "meep-storage", "meep-backup"] {
        rust_sources(&root.join(krate).join("src"), &mut files);
    }
    assert!(!files.is_empty());
    for (id, text) in catalog() {
        let fragment = text.split("{ $").next().unwrap_or("").trim();
        if fragment.len() < 12 {
            continue;
        }
        for file in &files {
            let source = fs::read_to_string(file).expect("source");
            assert!(
                !source.contains(fragment),
                "{id} wording is repeated in {}",
                file.display()
            );
        }
    }
}

/// Owner-approved wording (2026-11-22). Exact; do not reword.
#[test]
fn the_owner_approved_record_texts_are_exact() {
    let c = catalog();
    assert_eq!(
        c["record-changed"],
        "This item was changed in another window after you opened it, so your change wasn't saved. Copy anything you typed that you want to keep, then reload to see the latest version and try again."
    );
    assert_eq!(
        c["record-not-found"],
        "That item no longer exists. It may have been deleted."
    );
    assert_eq!(
        c["record-deleted"],
        "This item was deleted, and Meep can no longer open it. If you made a backup before it was deleted, it is still in that backup."
    );
    assert_eq!(
        c["internal-error"],
        "Something went wrong inside Meep. If this keeps happening, report this code: { $code }"
    );
}
