// =============================================================================
// Challenge 06: Control Flow as Expressions — HTTP Status Classifier
// (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

#[derive(Debug, PartialEq)]
enum Category {
    Informational, // 1xx
    Success,       // 2xx
    Redirect,      // 3xx
    ClientError,   // 4xx
    ServerError,   // 5xx
    Unknown,       // anything else
}

// -----------------------------------------------------------------------------
// Task 1: classify
// -----------------------------------------------------------------------------
fn classify(code: u16) -> Category {
    match code {
        100..=199 => Category::Informational,
        200..=299 => Category::Success,
        300..=399 => Category::Redirect,
        400..=499 => Category::ClientError,
        500..=599 => Category::ServerError,
        _ => Category::Unknown,
    }
}

// -----------------------------------------------------------------------------
// Task 2: is_retriable
// -----------------------------------------------------------------------------
fn is_retriable(code: u16) -> bool {
    matches!(code, 408 | 429 | 500..=599)
}

// -----------------------------------------------------------------------------
// Task 3: short_label
// -----------------------------------------------------------------------------
fn short_label(code: u16) -> &'static str {
    match classify(code) {
        Category::Informational => "INF",
        Category::Success => "OK_",
        Category::Redirect => "RDR",
        Category::ClientError => "C--",
        Category::ServerError => "S--",
        Category::Unknown => "???",
    }
}

// -----------------------------------------------------------------------------
// Task 4: parse_status
// -----------------------------------------------------------------------------
fn parse_status(s: &str) -> Option<u16> {
    let Ok(code) = s.parse::<u16>() else {
        return None;
    };
    if (100..=599).contains(&code) {
        Some(code)
    } else {
        None
    }
}

// -----------------------------------------------------------------------------
// Task 5: first_error_code
// -----------------------------------------------------------------------------
fn first_error_code(codes: &[u16]) -> Option<u16> {
    codes.iter().copied().find(|&code| is_retriable(code))
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Task 1 -----

    #[test]
    fn classify_buckets() {
        assert_eq!(classify(100), Category::Informational);
        assert_eq!(classify(199), Category::Informational);
        assert_eq!(classify(200), Category::Success);
        assert_eq!(classify(204), Category::Success);
        assert_eq!(classify(301), Category::Redirect);
        assert_eq!(classify(404), Category::ClientError);
        assert_eq!(classify(503), Category::ServerError);
    }

    #[test]
    fn classify_unknown() {
        assert_eq!(classify(0), Category::Unknown);
        assert_eq!(classify(99), Category::Unknown);
        assert_eq!(classify(600), Category::Unknown);
        assert_eq!(classify(999), Category::Unknown);
    }

    // ----- Task 2 -----

    #[test]
    fn retriable_codes() {
        assert!(is_retriable(408));
        assert!(is_retriable(429));
        assert!(is_retriable(500));
        assert!(is_retriable(502));
        assert!(is_retriable(599));
    }

    #[test]
    fn non_retriable_codes() {
        assert!(!is_retriable(200));
        assert!(!is_retriable(301));
        assert!(!is_retriable(404)); // not in the retry list
        assert!(!is_retriable(401));
    }

    // ----- Task 3 -----

    #[test]
    fn short_label_basic() {
        assert_eq!(short_label(100), "INF");
        assert_eq!(short_label(200), "OK_");
        assert_eq!(short_label(301), "RDR");
        assert_eq!(short_label(404), "C--");
        assert_eq!(short_label(503), "S--");
        assert_eq!(short_label(99), "???");
    }

    // ----- Task 4 -----

    #[test]
    fn parse_status_valid() {
        assert_eq!(parse_status("200"), Some(200));
        assert_eq!(parse_status("404"), Some(404));
        assert_eq!(parse_status("100"), Some(100));
        assert_eq!(parse_status("599"), Some(599));
    }

    #[test]
    fn parse_status_garbage() {
        assert_eq!(parse_status("nope"), None);
        assert_eq!(parse_status(""), None);
    }

    #[test]
    fn parse_status_out_of_range() {
        assert_eq!(parse_status("99"), None);
        assert_eq!(parse_status("600"), None);
        assert_eq!(parse_status("0"), None);
    }

    // ----- Task 5 -----

    #[test]
    fn first_error_code_finds_first_retriable() {
        let codes = vec![200, 200, 408, 500, 200];
        assert_eq!(first_error_code(&codes), Some(408));
    }

    #[test]
    fn first_error_code_no_match() {
        let codes = vec![200, 200, 301, 404]; // 404 is not retriable
        assert_eq!(first_error_code(&codes), None);
    }

    #[test]
    fn first_error_code_empty() {
        let codes: Vec<u16> = vec![];
        assert_eq!(first_error_code(&codes), None);
    }
}
