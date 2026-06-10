// =============================================================================
// Challenge 07: Numbers and Conversions — Byte Rate Calculator
// (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

use std::num::TryFromIntError;

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

// -----------------------------------------------------------------------------
// Task 1: bytes_to_kb
// -----------------------------------------------------------------------------
fn bytes_to_kb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0
}

// -----------------------------------------------------------------------------
// Task 2: count_to_u32
// -----------------------------------------------------------------------------
fn count_to_u32(items: &[u8]) -> Result<u32, TryFromIntError> {
    u32::try_from(items.len())
}

// -----------------------------------------------------------------------------
// Task 3: increment_counter
// -----------------------------------------------------------------------------
fn increment_counter(current: u32, delta: u32) -> u32 {
    current.saturating_add(delta)
}

// -----------------------------------------------------------------------------
// Task 4: percent
// -----------------------------------------------------------------------------
fn percent(numerator: u64, denominator: u64) -> Option<f64> {
    if denominator == 0 {
        None
    } else {
        Some(numerator as f64 / denominator as f64 * 100.0)
    }
}

// -----------------------------------------------------------------------------
// Task 5: rates_close
// -----------------------------------------------------------------------------
fn rates_close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

// -----------------------------------------------------------------------------
// Task 6: shrink_to_u8
// -----------------------------------------------------------------------------
fn shrink_to_u8(value: u32) -> Result<u8, u32> {
    u8::try_from(value).map_err(|_| value)
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Task 1 -----

    #[test]
    fn bytes_to_kb_exact() {
        assert!((bytes_to_kb(2048) - 2.0).abs() < 1e-9);
        assert!((bytes_to_kb(1024) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn bytes_to_kb_zero() {
        assert_eq!(bytes_to_kb(0), 0.0);
    }

    #[test]
    fn bytes_to_kb_fractional() {
        assert!((bytes_to_kb(512) - 0.5).abs() < 1e-9);
    }

    // ----- Task 2 -----

    #[test]
    fn count_to_u32_small_succeeds() {
        let items = vec![0_u8; 100];
        assert_eq!(count_to_u32(&items).unwrap(), 100);
    }

    #[test]
    fn count_to_u32_empty() {
        let items: Vec<u8> = vec![];
        assert_eq!(count_to_u32(&items).unwrap(), 0);
    }

    // ----- Task 3 -----

    #[test]
    fn increment_counter_normal() {
        assert_eq!(increment_counter(100, 50), 150);
        assert_eq!(increment_counter(0, 1), 1);
    }

    #[test]
    fn increment_counter_saturates() {
        // Without saturating, u32::MAX + 100 would overflow.
        assert_eq!(increment_counter(u32::MAX, 100), u32::MAX);
        assert_eq!(increment_counter(u32::MAX - 5, 10), u32::MAX);
    }

    // ----- Task 4 -----

    #[test]
    fn percent_normal() {
        assert!((percent(50, 200).unwrap() - 25.0).abs() < 1e-9);
        assert!((percent(1, 4).unwrap() - 25.0).abs() < 1e-9);
        assert!((percent(0, 100).unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn percent_zero_denominator() {
        assert_eq!(percent(50, 0), None);
        assert_eq!(percent(0, 0), None);
    }

    // ----- Task 5 -----

    #[test]
    fn rates_close_exact() {
        assert!(rates_close(1.5, 1.5));
        assert!(rates_close(0.0, 0.0));
    }

    #[test]
    fn rates_close_within_epsilon() {
        // 0.1 + 0.2 != 0.3 exactly, but should be "close"
        assert!(rates_close(0.1 + 0.2, 0.3));
    }

    #[test]
    fn rates_close_far_apart() {
        assert!(!rates_close(1.0, 1.5));
        assert!(!rates_close(0.0, 0.001));
    }

    // ----- Task 6 -----

    #[test]
    fn shrink_to_u8_fits() {
        assert_eq!(shrink_to_u8(0), Ok(0));
        assert_eq!(shrink_to_u8(200), Ok(200));
        assert_eq!(shrink_to_u8(255), Ok(255));
    }

    #[test]
    fn shrink_to_u8_overflows() {
        assert_eq!(shrink_to_u8(256), Err(256));
        assert_eq!(shrink_to_u8(1_000_000), Err(1_000_000));
    }
}
