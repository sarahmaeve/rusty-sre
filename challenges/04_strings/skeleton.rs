// =============================================================================
// Challenge 04: Strings — Hostname Normalizer
// =============================================================================
//
// You are building helpers that normalize and validate hostnames coming
// from a heterogeneous collection of services. The pipeline:
//
//     "  Auth.Example.Com:8443. " → "auth.example.com"
//                                    (trim, strip port, lowercase, trim
//                                     trailing dots, validate non-empty)
//
// Complete each function by replacing the TODO comments with working code.
// Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
}

// -----------------------------------------------------------------------------
// Task 1: trim_whitespace
// -----------------------------------------------------------------------------
// Return the input with leading and trailing whitespace removed, as an
// owned String. (We return String so later steps can keep mutating without
// worrying about lifetimes.)
//
// HINT: &str::trim() returns a &str. Use .to_string() or .to_owned() to
// get a String.
fn trim_whitespace(_s: &str) -> String {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: strip_port
// -----------------------------------------------------------------------------
// If the hostname contains a colon, return everything before the first colon.
// Otherwise return the input unchanged.
//
// Examples:
//   "auth.example.com:8443" → "auth.example.com"
//   "auth.example.com"      → "auth.example.com"
//
// HINT: str::split_once(':') returns Option<(&str, &str)>. unwrap_or works
// well to fall back to the original input.
fn strip_port(_s: &str) -> String {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 3: to_lower
// -----------------------------------------------------------------------------
// Return a lowercased copy of the input. ASCII-only is fine for hostnames.
//
// HINT: str::to_lowercase().
fn to_lower(_s: &str) -> String {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: trim_trailing_dots
// -----------------------------------------------------------------------------
// Strip any number of trailing '.' characters from the end. Don't touch
// dots elsewhere in the string.
//
// Examples:
//   "auth.example.com..." → "auth.example.com"
//   "auth.example.com"    → "auth.example.com"
//   "...."                → ""
//
// HINT: str::trim_end_matches('.') is exactly this.
fn trim_trailing_dots(_s: &str) -> String {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: normalize_hostname
// -----------------------------------------------------------------------------
// Compose the four helpers above and return a Result.
//
//   - Apply trim → strip_port → to_lower → trim_trailing_dots
//   - If the final result is empty, return Err("empty hostname".to_string())
//   - Otherwise return Ok(result)
fn normalize_hostname(_s: &str) -> Result<String, String> {
    // TODO
    todo!()
}

// =============================================================================
// TESTS — Do not modify below this line
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Task 1 -----

    #[test]
    fn trim_whitespace_removes_both_sides() {
        assert_eq!(trim_whitespace("  hello  "), "hello");
        assert_eq!(trim_whitespace("\t\nfoo\n"), "foo");
    }

    #[test]
    fn trim_whitespace_no_change() {
        assert_eq!(trim_whitespace("clean"), "clean");
    }

    #[test]
    fn trim_whitespace_all_whitespace() {
        assert_eq!(trim_whitespace("   "), "");
    }

    // ----- Task 2 -----

    #[test]
    fn strip_port_with_port() {
        assert_eq!(strip_port("auth.example.com:8443"), "auth.example.com");
        assert_eq!(strip_port("localhost:80"), "localhost");
    }

    #[test]
    fn strip_port_without_port() {
        assert_eq!(strip_port("auth.example.com"), "auth.example.com");
    }

    #[test]
    fn strip_port_only_colon() {
        assert_eq!(strip_port(":"), "");
    }

    // ----- Task 3 -----

    #[test]
    fn to_lower_basic() {
        assert_eq!(to_lower("ServiceA"), "servicea");
        assert_eq!(to_lower("ALL CAPS"), "all caps");
    }

    #[test]
    fn to_lower_idempotent() {
        assert_eq!(to_lower("already-lower"), "already-lower");
    }

    // ----- Task 4 -----

    #[test]
    fn trim_trailing_dots_one() {
        assert_eq!(trim_trailing_dots("host.example.com."), "host.example.com");
    }

    #[test]
    fn trim_trailing_dots_many() {
        assert_eq!(trim_trailing_dots("host.example.com..."), "host.example.com");
    }

    #[test]
    fn trim_trailing_dots_internal_unchanged() {
        assert_eq!(trim_trailing_dots("a.b.c"), "a.b.c");
    }

    #[test]
    fn trim_trailing_dots_only_dots() {
        assert_eq!(trim_trailing_dots("...."), "");
    }

    // ----- Task 5 -----

    #[test]
    fn normalize_full_pipeline() {
        let result = normalize_hostname("  Auth.Example.Com:8443. ").unwrap();
        assert_eq!(result, "auth.example.com");
    }

    #[test]
    fn normalize_already_clean() {
        let result = normalize_hostname("auth.example.com").unwrap();
        assert_eq!(result, "auth.example.com");
    }

    #[test]
    fn normalize_just_port() {
        // Trim, strip port (everything before colon), lowercase, trim dots
        let result = normalize_hostname("HOST:9000").unwrap();
        assert_eq!(result, "host");
    }

    #[test]
    fn normalize_empty_input_errors() {
        let result = normalize_hostname("");
        assert!(result.is_err());
    }

    #[test]
    fn normalize_whitespace_only_errors() {
        let result = normalize_hostname("   \t  ");
        assert!(result.is_err());
    }

    #[test]
    fn normalize_dots_only_errors() {
        // After trimming trailing dots, "..." becomes "" → error.
        let result = normalize_hostname("....");
        assert!(result.is_err());
    }
}
