// =============================================================================
// Challenge 04: Strings — Hostname Normalizer (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

// -----------------------------------------------------------------------------
// Task 1: trim_whitespace
// -----------------------------------------------------------------------------
fn trim_whitespace(s: &str) -> String {
    s.trim().to_string()
}

// -----------------------------------------------------------------------------
// Task 2: strip_port
// -----------------------------------------------------------------------------
fn strip_port(s: &str) -> String {
    s.split_once(':').map(|(host, _)| host).unwrap_or(s).to_string()
}

// -----------------------------------------------------------------------------
// Task 3: to_lower
// -----------------------------------------------------------------------------
fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

// -----------------------------------------------------------------------------
// Task 4: trim_trailing_dots
// -----------------------------------------------------------------------------
fn trim_trailing_dots(s: &str) -> String {
    s.trim_end_matches('.').to_string()
}

// -----------------------------------------------------------------------------
// Task 5: normalize_hostname
// -----------------------------------------------------------------------------
fn normalize_hostname(s: &str) -> Result<String, String> {
    let result = trim_trailing_dots(&to_lower(&strip_port(&trim_whitespace(s))));
    if result.is_empty() {
        Err("empty hostname".to_string())
    } else {
        Ok(result)
    }
}

// =============================================================================
// TESTS — identical to skeleton.rs
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
