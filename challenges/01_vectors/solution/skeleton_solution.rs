// =============================================================================
// Challenge 01: Vectors — SRE Metrics Collector (SKELETON SOLUTION)
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
// Task 1: Initialize a vector of response times
// -----------------------------------------------------------------------------
fn collect_response_times() -> Vec<f64> {
    vec![120.5, 200.3, 95.0, 310.7, 150.2, 89.4, 420.1, 175.6]
}

// -----------------------------------------------------------------------------
// Task 2: Add new entries and remove outliers
// -----------------------------------------------------------------------------
fn add_and_clean(mut times: Vec<f64>) -> Vec<f64> {
    times.extend([88.0, 500.5, 132.0]);
    times.retain(|&t| t <= 400.0);
    times
}

// -----------------------------------------------------------------------------
// Task 3: Compute min, max, and average
// -----------------------------------------------------------------------------
// f64 doesn't implement Ord, so we fold with f64::min / f64::max instead of
// using Iterator::min / Iterator::max.
fn compute_stats(times: &[f64]) -> (f64, f64, f64) {
    if times.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let min = times.iter().copied().fold(f64::INFINITY, f64::min);
    let max = times.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let avg = times.iter().sum::<f64>() / times.len() as f64;
    (min, max, avg)
}

// -----------------------------------------------------------------------------
// Task 4: Filter entries above a threshold
// -----------------------------------------------------------------------------
fn filter_above_threshold(times: &[f64], threshold: f64) -> Vec<f64> {
    times.iter().copied().filter(|&t| t > threshold).collect()
}

// -----------------------------------------------------------------------------
// Task 5: Deduplicate and sort
// -----------------------------------------------------------------------------
// dedup() only removes consecutive duplicates, so sort first.
fn deduplicate_sorted(mut names: Vec<String>) -> Vec<String> {
    names.sort();
    names.dedup();
    names
}

// =============================================================================
// TESTS — identical to skeleton.rs
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
