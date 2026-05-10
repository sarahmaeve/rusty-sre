// =============================================================================
// Challenge 06: Control Flow as Expressions — Debug Solution
// =============================================================================
//
// Bugs fixed:
//
//   1. classify_response: middle branch returns "success" (matching the
//      other arms' &str type) instead of the integer 200.
//   2. priority: the "low" arm returns 4 (u8) instead of "low" (&str).
//   3. is_success: range pattern is now `200..=299` (inclusive end) so that
//      299 counts as a success.
//   4. count_failures: replaces the discarded `count + 1` expression with
//      the side-effecting `count += 1` statement.
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

// FIX 1: middle branch now returns "success", not 200.
fn classify_response(code: u16) -> &'static str {
    if code < 200 {
        "informational"
    } else if code < 400 {
        "success"
    } else {
        "error"
    }
}

// FIX 2: "low" arm returns 4 (u8).
fn priority(category: &str) -> u8 {
    match category {
        "critical" => 1,
        "high" => 2,
        "medium" => 3,
        "low" => 4,
        _ => 5,
    }
}

// FIX 3: inclusive range so 299 counts.
fn is_success(code: u16) -> bool {
    matches!(code, 200..=299)
}

// FIX 4: count += 1 mutates the counter; the bare `count + 1` was discarded.
fn count_failures(codes: &[u16]) -> u32 {
    let mut count: u32 = 0;
    for &code in codes {
        if classify_response(code) == "error" {
            count += 1;
        }
    }
    count
}

fn notable_codes(codes: &[u16]) -> Vec<u16> {
    codes
        .iter()
        .copied()
        .filter(|&code| matches!(code, 408 | 429 | 500..=599))
        .collect()
}

fn analyze(codes: &[u16]) -> Report {
    let successes = codes.iter().filter(|&&c| is_success(c)).count() as u32;
    Report {
        successes,
        failures: count_failures(codes),
        notable: notable_codes(codes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_response_basic() {
        assert_eq!(classify_response(150), "informational");
        assert_eq!(classify_response(200), "success");
        assert_eq!(classify_response(301), "success");
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

    #[test]
    fn analyze_report() {
        let codes = vec![200, 201, 299, 301, 404, 500, 503, 200];
        let report = analyze(&codes);
        assert_eq!(report.successes, 4);
        assert_eq!(report.failures, 3);
        assert_eq!(report.notable, vec![500, 503]);
    }
}
