// =============================================================================
// Challenge 01: Vectors — SRE Metrics Collector
// =============================================================================
//
// You are building a simple metrics collector for an SRE dashboard. Response
// times (in milliseconds) come in from various services and need to be stored,
// cleaned, analyzed, and deduplicated.
//
// Complete each function by replacing the TODO comments with working code.
// Run the tests to verify your solutions:
//     rustc skeleton.rs --edition 2021 --test && ./skeleton
// =============================================================================

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
}

// -----------------------------------------------------------------------------
// Task 1: Initialize a vector of response times
// -----------------------------------------------------------------------------
// Return a Vec<f64> containing these response times in order:
//     120.5, 200.3, 95.0, 310.7, 150.2, 89.4, 420.1, 175.6
fn collect_response_times() -> Vec<f64> {
    // TODO: Create and return a vector with the values above
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: Add new entries and remove outliers
// -----------------------------------------------------------------------------
// Given a mutable vector of response times:
//   1. Push these new values onto the end: 88.0, 500.5, 132.0
//   2. Remove any entries greater than 400.0 (these are outliers)
// Return the modified vector.
fn add_and_clean(mut times: Vec<f64>) -> Vec<f64> {
    // TODO: Push the three new values

    // TODO: Remove entries greater than 400.0 (hint: use retain())

    times
}

// -----------------------------------------------------------------------------
// Task 3: Compute min, max, and average
// -----------------------------------------------------------------------------
// Given a slice of response times, return (min, max, average) as a tuple.
// If the slice is empty, return (0.0, 0.0, 0.0).
//
// Hint: f64 doesn't implement Ord, so you can't use .min() directly on an
// iterator of f64. Use .fold() or iterate manually. Alternatively, use
// f64::min() and f64::max() which compare two floats.
fn compute_stats(times: &[f64]) -> (f64, f64, f64) {
    // TODO: Handle the empty case

    // TODO: Calculate min, max, and average

    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: Filter entries above a threshold
// -----------------------------------------------------------------------------
// Return a new Vec<f64> containing only the response times that are strictly
// greater than `threshold`.
fn filter_above_threshold(times: &[f64], threshold: f64) -> Vec<f64> {
    // TODO: Use iterator methods to filter and collect into a new vector
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: Deduplicate and sort
// -----------------------------------------------------------------------------
// Given a vector of service names (strings) that may contain duplicates,
// return a new Vec<String> that is sorted alphabetically with no duplicates.
//
// Hint: There are multiple approaches — you could use a HashSet, or sort + dedup.
fn deduplicate_sorted(names: Vec<String>) -> Vec<String> {
    // TODO: Remove duplicates and sort alphabetically
    todo!()
}

// =============================================================================
// TESTS — Do not modify below this line
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_response_times() {
        let times = collect_response_times();
        assert_eq!(times.len(), 8);
        assert_eq!(times[0], 120.5);
        assert_eq!(times[7], 175.6);
        assert!((times.iter().sum::<f64>() - 1561.8).abs() < 0.01);
    }

    #[test]
    fn test_add_and_clean() {
        let initial = vec![100.0, 200.0, 450.0, 300.0];
        let result = add_and_clean(initial);

        // 450.0 and 500.5 should be removed (both > 400.0)
        assert!(!result.contains(&450.0));
        assert!(!result.contains(&500.5));

        // These should all remain
        assert!(result.contains(&100.0));
        assert!(result.contains(&200.0));
        assert!(result.contains(&300.0));
        assert!(result.contains(&88.0));
        assert!(result.contains(&132.0));
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_compute_stats_normal() {
        let times = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        let (min, max, avg) = compute_stats(&times);
        assert_eq!(min, 100.0);
        assert_eq!(max, 500.0);
        assert!((avg - 300.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_stats_single() {
        let times = vec![42.0];
        let (min, max, avg) = compute_stats(&times);
        assert_eq!(min, 42.0);
        assert_eq!(max, 42.0);
        assert_eq!(avg, 42.0);
    }

    #[test]
    fn test_compute_stats_empty() {
        let times: Vec<f64> = vec![];
        let (min, max, avg) = compute_stats(&times);
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.0);
        assert_eq!(avg, 0.0);
    }

    #[test]
    fn test_filter_above_threshold() {
        let times = vec![50.0, 150.0, 250.0, 350.0, 450.0];
        let result = filter_above_threshold(&times, 200.0);
        assert_eq!(result, vec![250.0, 350.0, 450.0]);
    }

    #[test]
    fn test_filter_above_threshold_none_match() {
        let times = vec![10.0, 20.0, 30.0];
        let result = filter_above_threshold(&times, 100.0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_deduplicate_sorted() {
        let names = vec![
            "payments".to_string(),
            "auth".to_string(),
            "payments".to_string(),
            "gateway".to_string(),
            "auth".to_string(),
            "logging".to_string(),
            "gateway".to_string(),
        ];
        let result = deduplicate_sorted(names);
        assert_eq!(
            result,
            vec![
                "auth".to_string(),
                "gateway".to_string(),
                "logging".to_string(),
                "payments".to_string(),
            ]
        );
    }

    #[test]
    fn test_deduplicate_sorted_no_dupes() {
        let names = vec!["z".to_string(), "a".to_string(), "m".to_string()];
        let result = deduplicate_sorted(names);
        assert_eq!(
            result,
            vec!["a".to_string(), "m".to_string(), "z".to_string()]
        );
    }
}
