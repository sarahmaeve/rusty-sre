// =============================================================================
// Challenge 06: Control Flow as Expressions — Debug the Response Analyzer
// =============================================================================
//
// This program analyzes batches of HTTP responses for a load test report.
// It contains FOUR control-flow bugs:
//
//   1. classify_response: if/else arms produce different types (compile)
//   2. priority: a match arm returns the wrong type (compile)
//   3. is_success: 200..299 (exclusive end) used where 200..=299 was intended,
//      so a 299 status is misclassified (runtime)
//   4. count_failures: `count + 1` is computed and thrown away instead of
//      `count += 1`, so the counter never increases (runtime)
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

// -----------------------------------------------------------------------------
// BUG 1: if/else arms produce different types
// -----------------------------------------------------------------------------
// Returns a short text label for an HTTP status code. The middle branch
// accidentally returns an integer instead of a &str, so the if/else doesn't
// type-check.
fn classify_response(code: u16) -> &'static str {
    if code < 200 {
        "informational"
    } else if code < 400 {
        200
    } else {
        "error"
    }
}

// -----------------------------------------------------------------------------
// BUG 2: match arm returns the wrong type
// -----------------------------------------------------------------------------
// Map a category name to a priority number. One arm returns a &str instead
// of u8.
fn priority(category: &str) -> u8 {
    match category {
        "critical" => 1,
        "high" => 2,
        "medium" => 3,
        "low" => "low",
        _ => 5,
    }
}

// -----------------------------------------------------------------------------
// BUG 3: range pattern uses exclusive end where inclusive was intended
// -----------------------------------------------------------------------------
// 200..299 matches 200..=298 — it does NOT include 299. Real HTTP success
// codes go through 299 (inclusive). The fix is `200..=299`.
fn is_success(code: u16) -> bool {
    matches!(code, 200..299)
}

// -----------------------------------------------------------------------------
// BUG 4: count + 1 is computed and discarded
// -----------------------------------------------------------------------------
// The body of the `if` is an expression that produces `count + 1`, but the
// result is never assigned anywhere. So count stays at 0 forever. The fix
// is `count += 1` — a statement that mutates count.
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

// Build the full report. Depends on is_success and count_failures being
// correct — neither bug shows up here directly, but the assertions in the
// integration test will fail until both are fixed.
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

    // BUG 1: classify_response must compile
    #[test]
    fn classify_response_basic() {
        assert_eq!(classify_response(150), "informational");
        assert_eq!(classify_response(200), "success");
        assert_eq!(classify_response(301), "success"); // anything 200..400
        assert_eq!(classify_response(404), "error");
        assert_eq!(classify_response(503), "error");
    }

    // BUG 2: priority must compile and return u8
    #[test]
    fn priority_table() {
        assert_eq!(priority("critical"), 1);
        assert_eq!(priority("high"), 2);
        assert_eq!(priority("medium"), 3);
        assert_eq!(priority("low"), 4);
        assert_eq!(priority("unknown"), 5);
    }

    // BUG 3: 299 must be a success
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

    // BUG 4: count_failures must actually count
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

    // Integration — exercises bugs 1, 3, and 4 together
    #[test]
    fn analyze_report() {
        let codes = vec![200, 201, 299, 301, 404, 500, 503, 200];
        let report = analyze(&codes);
        // 200, 201, 299, 200 — four successes (note: 299 only counts after BUG 3)
        assert_eq!(report.successes, 4);
        // 404, 500, 503 — three failures (counter only works after BUG 4)
        assert_eq!(report.failures, 3);
        // 500 and 503 are in the notable retry-eligible set
        assert_eq!(report.notable, vec![500, 503]);
    }
}

// Some classify_response branches use "success" as the label for both 2xx
// and 3xx. The compile error from BUG 1 returns 200 instead — fix it to
// return "success".
