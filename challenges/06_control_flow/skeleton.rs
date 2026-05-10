// =============================================================================
// Challenge 06: Control Flow as Expressions — HTTP Status Classifier
// =============================================================================
//
// You are building helpers that summarize HTTP status codes for a metrics
// pipeline. Each task uses Rust's expression-oriented control flow to keep
// the code short.
//
// Complete each TODO. Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
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
// Given an HTTP status code, return the matching Category.
//
// 100..=199 → Informational
// 200..=299 → Success
// 300..=399 → Redirect
// 400..=499 → ClientError
// 500..=599 → ServerError
// otherwise → Unknown
//
// HINT: use match with inclusive range patterns. Make the whole match an
// expression that's the function's return value.
fn classify(_code: u16) -> Category {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: is_retriable
// -----------------------------------------------------------------------------
// Return true if the status is one we should retry on. Retry policy:
//   - 408 (Request Timeout)
//   - 429 (Too Many Requests)
//   - 500..=599 (any server error)
//
// HINT: use match with alternation `408 | 429` and a range. The whole match
// is a bool expression.
fn is_retriable(_code: u16) -> bool {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 3: short_label
// -----------------------------------------------------------------------------
// Return a short, fixed-width 3-letter label for a status code. Use:
//   - "INF" for 1xx
//   - "OK_" for 2xx
//   - "RDR" for 3xx
//   - "C--" for 4xx (any 4xx)
//   - "S--" for 5xx (any 5xx)
//   - "???" for everything else
//
// HINT: build on classify() and use `match` on Category. Return &'static str.
fn short_label(_code: u16) -> &'static str {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: parse_status
// -----------------------------------------------------------------------------
// Parse a string like "200" into an Option<u16>. Reject:
//   - any string that doesn't parse as u16
//   - any value outside 100..=599
//
// HINT: use `let ... else` for the parse step, then a guarded match (or a
// simple if) for the range check. The whole function body should be one
// expression after the `let else`.
fn parse_status(_s: &str) -> Option<u16> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: first_error_code
// -----------------------------------------------------------------------------
// Given a slice of status codes, return the FIRST one that is_retriable as
// Some(code). If none qualify, return None.
//
// HINT: Iterator::find returns Option<&T>. .copied() turns Option<&u16> into
// Option<u16>.
fn first_error_code(_codes: &[u16]) -> Option<u16> {
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
