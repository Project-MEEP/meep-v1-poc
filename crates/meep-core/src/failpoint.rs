//! Feature-gated failpoints (FR-015, PLAN-18, PLAN-27): a named point in the code that a crash
//! test can turn into a process termination or an injected I/O error. Without the `failpoints`
//! feature the point compiles to nothing.

use std::io;

/// Exit code of a process ended by a `Terminate` failpoint, so a parent test can tell it apart.
pub const FAILPOINT_EXIT_CODE: i32 = 86;

/// Environment variable read by `configure_from_env`: `name=action;name=action`.
pub const FAILPOINTS_ENV: &str = "MEEP_FAILPOINTS";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// End the process at once, like a crash (no clean-up, no unwinding).
    Terminate,
    /// Return an I/O error of this kind (for example `StorageFull`, `PermissionDenied`).
    Error(io::ErrorKind),
}

/// Parse `name=action;name=action`; actions are `terminate`, `disk-full`, `permission-denied`.
pub fn parse_spec(spec: &str) -> Result<Vec<(String, Action)>, String> {
    if spec.is_empty() {
        return Ok(Vec::new());
    }
    spec.split(';')
        .map(|part| {
            let (name, action) = part
                .split_once('=')
                .ok_or_else(|| format!("failpoint entry {part:?} has no '='"))?;
            if name.is_empty() {
                return Err(format!("failpoint entry {part:?} has no name"));
            }
            let action = match action {
                "terminate" => Action::Terminate,
                "disk-full" => Action::Error(io::ErrorKind::StorageFull),
                "permission-denied" => Action::Error(io::ErrorKind::PermissionDenied),
                other => return Err(format!("unknown failpoint action {other:?}")),
            };
            Ok((name.to_string(), action))
        })
        .collect()
}

#[cfg(any(test, feature = "failpoints"))]
mod active {
    use super::{Action, FAILPOINT_EXIT_CODE, FAILPOINTS_ENV, parse_spec};
    use std::collections::HashMap;
    use std::io;
    use std::sync::{Mutex, OnceLock, PoisonError};

    fn registry() -> &'static Mutex<HashMap<String, Action>> {
        static REGISTRY: OnceLock<Mutex<HashMap<String, Action>>> = OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
    }

    pub fn set(name: &str, action: Action) {
        registry()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(name.to_string(), action);
    }

    pub fn remove(name: &str) {
        registry()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(name);
    }

    /// Arm the failpoints named in `MEEP_FAILPOINTS` (used by crash-test child processes).
    pub fn configure_from_env() -> Result<(), String> {
        if let Ok(spec) = std::env::var(FAILPOINTS_ENV) {
            for (name, action) in parse_spec(&spec)? {
                set(&name, action);
            }
        }
        Ok(())
    }

    pub fn hit(name: &str) -> io::Result<()> {
        let action = registry()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(name)
            .copied();
        match action {
            None => Ok(()),
            Some(Action::Error(kind)) => Err(io::Error::new(
                kind,
                format!("injected failure at failpoint {name}"),
            )),
            Some(Action::Terminate) => std::process::exit(FAILPOINT_EXIT_CODE),
        }
    }
}

#[cfg(any(test, feature = "failpoints"))]
pub use active::{configure_from_env, hit, remove, set};

/// Without the `failpoints` feature a failpoint does nothing and the call is optimized away.
#[cfg(not(any(test, feature = "failpoints")))]
#[inline(always)]
pub fn hit(_name: &str) -> io::Result<()> {
    Ok(())
}

/// Mark a failpoint: `failpoint!("save.after_key")?;`
#[macro_export]
macro_rules! failpoint {
    ($name:expr) => {
        $crate::failpoint::hit($name)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn an_unconfigured_failpoint_does_nothing() {
        assert!(hit("test.unconfigured").is_ok());
    }

    #[test]
    fn an_error_failpoint_returns_the_chosen_io_error_until_removed() {
        set("test.disk_full", Action::Error(io::ErrorKind::StorageFull));
        assert_eq!(
            hit("test.disk_full").unwrap_err().kind(),
            io::ErrorKind::StorageFull
        );
        set(
            "test.disk_full",
            Action::Error(io::ErrorKind::PermissionDenied),
        );
        assert_eq!(
            hit("test.disk_full").unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        remove("test.disk_full");
        assert!(hit("test.disk_full").is_ok());
    }

    #[test]
    fn a_failpoint_only_affects_its_own_name() {
        set("test.one", Action::Error(io::ErrorKind::StorageFull));
        assert!(hit("test.two").is_ok());
        remove("test.one");
    }

    #[test]
    fn the_macro_returns_the_injected_error_to_the_caller() {
        fn operation() -> io::Result<()> {
            failpoint!("test.macro")?;
            Ok(())
        }
        assert!(operation().is_ok());
        set("test.macro", Action::Error(io::ErrorKind::PermissionDenied));
        assert_eq!(
            operation().unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        remove("test.macro");
    }

    #[test]
    fn a_spec_string_lists_failpoints_and_actions() {
        let parsed = parse_spec("a.b=terminate;c.d=disk-full;e.f=permission-denied").unwrap();
        assert_eq!(
            parsed,
            vec![
                ("a.b".to_string(), Action::Terminate),
                ("c.d".to_string(), Action::Error(io::ErrorKind::StorageFull)),
                (
                    "e.f".to_string(),
                    Action::Error(io::ErrorKind::PermissionDenied)
                ),
            ]
        );
        assert_eq!(parse_spec("").unwrap(), vec![]);
    }

    #[test]
    fn a_malformed_spec_is_refused() {
        for bad in ["noequals", "=terminate", "a=explode", "a=terminate;;b"] {
            assert!(parse_spec(bad).is_err(), "{bad}");
        }
    }

    /// Helper run only as a child of `a_terminate_failpoint_ends_the_process_with_its_exit_code`.
    #[test]
    fn child_terminate_helper() {
        if std::env::var("MEEP_FAILPOINT_CHILD").is_err() {
            return;
        }
        set("child.point", Action::Terminate);
        let _ = hit("child.point");
        std::process::exit(1); // reached only if the failpoint did not terminate
    }

    #[test]
    fn a_terminate_failpoint_ends_the_process_with_its_exit_code() {
        let status = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "failpoint::tests::child_terminate_helper",
                "--nocapture",
            ])
            .env("MEEP_FAILPOINT_CHILD", "1")
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(FAILPOINT_EXIT_CODE));
    }
}
