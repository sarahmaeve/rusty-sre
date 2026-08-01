//! Cargo describes packages, resolves dependency graphs, and invokes `rustc` for
//! each crate target. Read the root manifest before interpreting conditional code.
//!
//! A workspace coordinates packages. A package owns one manifest and can expose
//! library, binary, example, test, benchmark, and build-script crate targets.
//! Features should be additive. Profiles can change compilation and `cfg`-visible
//! behavior, so profile settings are correctness inputs.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/cargo/reference/manifest.html>
//! - <https://doc.rust-lang.org/cargo/reference/workspaces.html>
//! - <https://doc.rust-lang.org/cargo/reference/features.html>
//! - <https://doc.rust-lang.org/cargo/reference/profiles.html>
//! - <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
//! - Source study: <https://github.com/astral-sh/uv/blob/main/Cargo.toml>
//! - Source study: <https://github.com/rust-lang/rust/blob/master/Cargo.toml>

const WORKSPACE_MANIFEST: &str = r#"
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.97.1"

[workspace.dependencies]
tracing = "0.1"

[profile.release]
lto = "thin"
panic = "abort"
"#;

const MEMBER_MANIFEST: &str = r#"
[package]
name = "collector"
edition.workspace = true
rust-version.workspace = true

[features]
default = []
telemetry = ["dep:tracing"]

[dependencies]
tracing = { workspace = true, optional = true }
"#;

const BUILD_SCRIPT: &str = r#"
fn main() {
    println!("cargo::rerun-if-changed=schemas/metrics.txt");
    println!("cargo::rustc-check-cfg=cfg(generated_schema)");
    println!("cargo::rustc-cfg=generated_schema");
    let output = std::env::var_os("OUT_DIR").unwrap();
}
"#;

#[cfg(feature = "telemetry")]
fn compiled_mode() -> &'static str {
    "telemetry"
}

#[cfg(not(feature = "telemetry"))]
fn compiled_mode() -> &'static str {
    "minimal"
}

fn main() {
    assert_eq!(env!("CARGO_PKG_NAME"), "rusty-sre-guides");

    // `workspace = true` inherits a value; it does not merge arbitrary tables.
    assert!(WORKSPACE_MANIFEST.contains("[workspace.dependencies]"));
    assert!(MEMBER_MANIFEST.contains("edition.workspace = true"));
    assert!(MEMBER_MANIFEST.contains("tracing = { workspace = true"));

    // Features select `cfg(feature = "...")` items and optional dependencies.
    // They are resolved for the dependency graph, so disabling a feature should
    // not be required to make another feature correct.
    #[cfg(feature = "telemetry")]
    assert_eq!(compiled_mode(), "telemetry");
    #[cfg(not(feature = "telemetry"))]
    assert_eq!(compiled_mode(), "minimal");

    // `cfg!` yields a boolean, while `#[cfg]` removes an item before type checking.
    assert_eq!(cfg!(feature = "telemetry"), compiled_mode() == "telemetry");

    // Only the workspace root's profile tables apply. Review release settings for
    // overflow checks, panic behavior, debug information, LTO, and codegen units.
    assert!(WORKSPACE_MANIFEST.contains("[profile.release]"));
    assert!(WORKSPACE_MANIFEST.contains("panic = \"abort\""));
    assert_eq!(cfg!(debug_assertions), !cfg!(not(debug_assertions)));

    // `build.rs` is compiled for and runs on the host. Generated files belong in
    // `OUT_DIR`; target facts arrive through `CARGO_CFG_*` environment variables.
    // Audit generated code, native compilation, rerun conditions, and link flags.
    assert!(BUILD_SCRIPT.contains("cargo::rerun-if-changed"));
    assert!(BUILD_SCRIPT.contains("cargo::rustc-check-cfg"));
    assert!(BUILD_SCRIPT.contains("OUT_DIR"));
}
