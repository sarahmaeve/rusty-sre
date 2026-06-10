// =============================================================================
// Challenge 06: Control Flow as Expressions — Debug the Response Analyzer
// =============================================================================
//
// This program analyzes batches of HTTP responses for a load test report.
// It contains FOUR control-flow bugs — some stop it compiling, some
// misbehave at runtime. Find and fix all four so every test passes.
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

fn main() {
    let codes = vec![200, 201, 299, 301, 404, 500, 503, 200];
    let report = analyze(&codes);
    println!("{report:#?}");
}

#[derive(Debug, PartialEq)]
struct Report {
    successes: u32,
    failures: u32,
    notable: Vec<u16>,
}

// Returns a short text label for an HTTP status code.
fn classify_response(code: u16) -> &'static str {
    if code < 200 {
        "informational"
    } else if code < 400 {
        200
    } else {
        "error"
    }
}

// Map a category name to a priority number.
fn priority(category: &str) -> u8 {
    match category {
        "critical" => 1,
        "high" => 2,
        "medium" => 3,
        "low" => "low",
        _ => 5,
    }
}

// True if the code is an HTTP success status. Real HTTP success codes run
// from 200 through 299, inclusive.
fn is_success(code: u16) -> bool {
    matches!(code, 200..299)
}

// Count the responses that classify as errors.
fn count_failures(codes: &[u16]) -> u32 {
    let mut count: u32 = 0;
    for &code in codes {
        if classify_response(code) == "error" {
            count + 1;
        }
    }
    count
}

// Pull out "notable" status codes — anything in the retry-eligible set.
fn notable_codes(codes: &[u16]) -> Vec<u16> {
    codes
        .iter()
        .copied()
        .filter(|&code| matches!(code, 408 | 429 | 500..=599))
        .collect()
}

// Build the full report from the helpers above.
fn analyze(codes: &[u16]) -> Report {
    let successes = codes.iter().filter(|&&c| is_success(c)).count() as u32;
    Report {
        successes,
        failures: count_failures(codes),
        notable: notable_codes(codes),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_response_basic() {
        assert_eq!(classify_response(150), "informational");
        assert_eq!(classify_response(200), "success");
        assert_eq!(classify_response(301), "success"); // anything 200..400
        assert_eq!(classify_response(404), "error");
        assert_eq!(classify_response(503), "error");
    }

    #[test]
    fn priority_table() {
        assert_eq!(priority("critical"), 1);
        assert_eq!(priority("high"), 2);
        assert_eq!(priority("medium"), 3);
        assert_eq!(priority("low"), 4);
        assert_eq!(priority("unknown"), 5);
    }

    #[test]
    fn is_success_inclusive() {
        assert!(is_success(200));
        assert!(is_success(204));
        assert!(is_success(299));
    }

    #[test]
    fn is_success_excludes_others() {
        assert!(!is_success(199));
        assert!(!is_success(300));
        assert!(!is_success(404));
    }

    #[test]
    fn count_failures_counts() {
        let codes = vec![200, 404, 503, 200, 500];
        assert_eq!(count_failures(&codes), 3);
    }

    #[test]
    fn count_failures_empty() {
        let codes: Vec<u16> = vec![];
        assert_eq!(count_failures(&codes), 0);
    }

    // Integration test
    #[test]
    fn analyze_report() {
        let codes = vec![200, 201, 299, 301, 404, 500, 503, 200];
        let report = analyze(&codes);
        // 200, 201, 299, 200 — four successes
        assert_eq!(report.successes, 4);
        // 404, 500, 503 — three failures
        assert_eq!(report.failures, 3);
        // 500 and 503 are in the notable retry-eligible set
        assert_eq!(report.notable, vec![500, 503]);
    }
}
