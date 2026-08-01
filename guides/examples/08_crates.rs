//! A crate is Rust's compilation unit; a package is Cargo's manifest plus one or
//! more crate targets. Modules form the crate's namespace and privacy boundary.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
//! - <https://doc.rust-lang.org/cargo/reference/manifest.html>
//! - <https://doc.rust-lang.org/cargo/reference/features.html>
//! - <https://doc.rust-lang.org/cargo/reference/workspaces.html>
//! - Source study: <https://github.com/rust-lang/cargo>

mod health {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Report {
        // Visible to this crate, but not to downstream crates.
        pub(crate) checks: Vec<Check>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct Check {
        name: String,
        healthy: bool,
    }

    impl Report {
        pub fn all_healthy(&self) -> bool {
            self.checks.iter().all(|check| check.healthy)
        }
    }

    pub fn run() -> Report {
        Report {
            checks: vec![
                Check {
                    name: "database".to_owned(),
                    healthy: true,
                },
                Check {
                    name: "queue".to_owned(),
                    healthy: true,
                },
            ],
        }
    }

    // Child modules can access private items in ancestor modules.
    pub mod summary {
        use super::Report;

        pub fn names(report: &Report) -> Vec<&str> {
            report
                .checks
                .iter()
                .map(|check| check.name.as_str())
                .collect()
        }
    }
}

#[cfg(feature = "telemetry")]
fn mode() -> &'static str {
    "telemetry-enabled"
}

#[cfg(not(feature = "telemetry"))]
fn mode() -> &'static str {
    "minimal"
}

fn main() {
    let report = health::run();
    assert!(report.all_healthy());
    assert_eq!(health::summary::names(&report), ["database", "queue"]);

    #[cfg(feature = "telemetry")]
    assert_eq!(mode(), "telemetry-enabled");
    #[cfg(not(feature = "telemetry"))]
    assert_eq!(mode(), "minimal");
}
