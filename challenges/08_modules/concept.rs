// =============================================================================
// Challenge 08: Modules and Visibility — Concept Explainer
// =============================================================================
//
// Up to this point every challenge has been a single file. Real Rust
// projects split code across modules (and usually files). This challenge
// teaches the module system without leaving a single file:
//
//   - mod foo { ... }            inline module declaration
//   - mod foo;                   "look in foo.rs (or foo/mod.rs) for the body"
//   - pub                        export an item to outside the module
//   - pub(crate) / pub(super)    more limited visibility
//   - use path::to::Item;        bring an item into scope
//   - super::, self::, crate::   relative path prefixes
//
// In a real project you'd split each module into its own file. Here we
// keep everything in concept.rs using inline `mod foo { ... }` blocks. The
// rules are identical — the only thing that changes in a multi-file layout
// is that `mod foo;` (with a semicolon) tells the compiler to look at
// `foo.rs` instead of expecting a body inline.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   foo.py + bar.py                mod foo; mod bar;
//   __init__.py                    mod.rs (or named-after-the-dir)
//   from foo import bar            use foo::bar;
//   from . import foo              use self::foo;       (or crate::foo;)
//   from .. import foo             use super::foo;
//   class _Private:                fn private_fn() ...  (no `pub`)

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. AN INLINE MODULE
// =============================================================================
// Items inside `mod foo { ... }` are private by default. To use them from
// outside, prefix with `foo::`. The item must be `pub` for that to work.

mod metrics {
    pub fn record_request() -> u32 {
        42
    }

    // Private — only callable inside `mod metrics { ... }`.
    fn internal_helper() -> u32 {
        1
    }

    pub fn public_using_private() -> u32 {
        // From INSIDE the module, you can call private items freely.
        internal_helper() + record_request()
    }
}

#[test]
fn calling_into_a_module() {
    // Public items: accessible.
    assert_eq!(metrics::record_request(), 42);
    assert_eq!(metrics::public_using_private(), 43);

    // Private items: not accessible from out here. Uncommenting the next
    // line would fail to compile with "function `internal_helper` is private".
    //   metrics::internal_helper();
}

// =============================================================================
// 2. STRUCTS AND ENUMS HAVE FIELD-LEVEL PRIVACY
// =============================================================================
// Marking the type `pub` exposes the type's NAME. Each FIELD also needs
// its own `pub` to be readable from outside. (Enum variants are different —
// they inherit the enum's visibility.)

mod alerts {
    pub struct Alert {
        pub service: String,
        pub severity: u8,
        // Private field — outsiders can't read or set it directly.
        deduped_count: u32,
    }

    impl Alert {
        pub fn new(service: &str, severity: u8) -> Self {
            Self {
                service: service.to_string(),
                severity,
                deduped_count: 0,
            }
        }

        pub fn dedupe_count(&self) -> u32 {
            self.deduped_count
        }

        pub fn note_duplicate(&mut self) {
            self.deduped_count += 1;
        }
    }

    // Enum variants follow the enum's visibility — once Severity is pub,
    // all its variants are pub.
    pub enum Severity {
        Info,
        Warn,
        Error,
    }
}

#[test]
fn field_privacy() {
    use alerts::Alert;

    let mut a = Alert::new("auth", 4);

    // Public fields: accessible.
    assert_eq!(a.service, "auth");
    assert_eq!(a.severity, 4);

    // Private field: not directly accessible. Uncommenting fails to compile.
    //   a.deduped_count = 5;

    // Use the public methods instead.
    a.note_duplicate();
    a.note_duplicate();
    assert_eq!(a.dedupe_count(), 2);
}

// =============================================================================
// 3. NESTED MODULES — super:: AND crate::
// =============================================================================
// Modules can nest. From a child, `super::` reaches one level up; `crate::`
// jumps to the root of the current crate.

mod pipeline {
    pub fn name() -> &'static str {
        "pipeline"
    }

    pub mod sink {
        pub fn name() -> &'static str {
            "sink"
        }

        pub fn parent_name() -> &'static str {
            // sink is a child of pipeline — super:: walks up to pipeline.
            super::name()
        }

        pub fn root_constant() -> u32 {
            // crate:: walks up to the root of the current compilation unit.
            // In a binary this is the file with main(); in a library it's lib.rs.
            crate::ROOT_BUDGET
        }
    }
}

const ROOT_BUDGET: u32 = 1000;

#[test]
fn nested_modules() {
    assert_eq!(pipeline::name(), "pipeline");
    assert_eq!(pipeline::sink::name(), "sink");
    assert_eq!(pipeline::sink::parent_name(), "pipeline");
    assert_eq!(pipeline::sink::root_constant(), 1000);
}

// =============================================================================
// 4. `use` BRINGS NAMES INTO SCOPE
// =============================================================================
// Without `use`, every reference needs the full path. `use` creates a
// shorter alias for a path; it doesn't move the item.

mod parsers {
    pub fn parse_port(s: &str) -> Option<u16> {
        s.parse().ok()
    }

    pub fn parse_host(s: &str) -> &str {
        s.trim()
    }
}

#[test]
fn use_imports() {
    // Without use:
    let p = parsers::parse_port("8080");
    assert_eq!(p, Some(8080));

    // With use — bring the function into local scope.
    use parsers::{parse_host, parse_port};
    assert_eq!(parse_port("9090"), Some(9090));
    assert_eq!(parse_host(" example.com "), "example.com");

    // You can also alias on import:
    use parsers::parse_port as parse_port_value;
    assert_eq!(parse_port_value("80"), Some(80));
}

// =============================================================================
// 5. pub(crate) AND pub(super) — LIMITED PUBLIC VISIBILITY
// =============================================================================
// `pub` exposes an item to anyone who can name your module. Sometimes you
// want narrower visibility:
//
//   pub                — visible to anyone, including external crates that
//                        depend on yours.
//   pub(crate)         — visible anywhere in *this* crate, but not to
//                        external users. Great for "internal API" splits.
//   pub(super)         — visible to the parent module only.
//   pub(in path)       — visible to a named ancestor only.
//   (no modifier)      — private to this module.

mod telemetry {
    // Visible to any consumer of this crate.
    pub fn record() -> u32 {
        helpers::next_id() + 1
    }

    pub(crate) mod helpers {
        // Visible only inside this crate. Users of the crate can't call this.
        pub(crate) fn next_id() -> u32 {
            41
        }
    }
}

#[test]
fn limited_visibility() {
    assert_eq!(telemetry::record(), 42);
    // From inside the same crate, pub(crate) items are reachable:
    assert_eq!(telemetry::helpers::next_id(), 41);
}

// =============================================================================
// 6. THE TESTS MODULE PATTERN
// =============================================================================
// You've seen `#[cfg(test)] mod tests { ... }` in every challenge. That's
// just an inline module gated to test builds. Because it's a child of the
// module-under-test, it can see private items via `use super::*;`.

mod calculator {
    // Private function — visible only inside this module.
    fn double(n: i32) -> i32 {
        n * 2
    }

    pub fn quadruple(n: i32) -> i32 {
        double(double(n))
    }

    // The tests module sits inside `calculator`, so super::* covers all
    // its private items (including `double`).
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn private_double_works() {
            // We can call the PRIVATE function because we're inside the module.
            assert_eq!(double(5), 10);
        }

        #[test]
        fn public_quadruple_works() {
            assert_eq!(quadruple(3), 12);
        }
    }
}

// =============================================================================
// 7. WHERE THIS GOES IN A REAL PROJECT
// =============================================================================
// In a real Cargo project, you wouldn't write `mod foo { ... }` inline.
// You'd write `mod foo;` (with a semicolon) and put the body in `src/foo.rs`
// or `src/foo/mod.rs`. Everything else — pub, use, super::, the test
// module pattern — is identical.
//
// For our no-Cargo setup, inline modules teach the same rules without the
// filesystem dance.
