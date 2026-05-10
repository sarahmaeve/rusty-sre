// =============================================================================
// Challenge 07: Numbers and Conversions — Byte Rate Calculator
// =============================================================================
//
// You are building helpers for a metrics dashboard that reports byte counts
// in human-readable units, computes throughput rates, and handles the tricky
// edge cases (overflow, division by zero, float comparison).
//
// Complete each TODO. Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

use std::num::TryFromIntError;

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
}

// -----------------------------------------------------------------------------
// Task 1: bytes_to_kb
// -----------------------------------------------------------------------------
// Convert a byte count (u64) to kilobytes (f64). 1 KB = 1024 bytes.
//
// HINT: cast u64 to f64 with `as`, then divide.
fn bytes_to_kb(_bytes: u64) -> f64 {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: count_to_u32
// -----------------------------------------------------------------------------
// Convert a slice's length (usize) into u32 for a metric payload. Return
// Err on overflow.
//
// HINT: u32::try_from(...).
fn count_to_u32(_items: &[u8]) -> Result<u32, TryFromIntError> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 3: increment_counter
// -----------------------------------------------------------------------------
// Increment `current` by `delta`, saturating at u32::MAX. Counters that
// silently wrap are an SRE nightmare — always saturate when in doubt.
//
// HINT: saturating_add.
fn increment_counter(_current: u32, _delta: u32) -> u32 {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: percent
// -----------------------------------------------------------------------------
// Compute (numerator / denominator) * 100.0 as f64. Return None if
// denominator is 0.
fn percent(_numerator: u64, _denominator: u64) -> Option<f64> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: rates_close
// -----------------------------------------------------------------------------
// Return true if two f64 rates are "close enough" — within 1e-6 of each
// other in absolute terms. (For real metrics work you'd parameterize the
// epsilon, but a fixed value is fine here.)
//
// HINT: (a - b).abs() < 1e-6.
fn rates_close(_a: f64, _b: f64) -> bool {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 6: shrink_to_u8
// -----------------------------------------------------------------------------
// Convert a u32 to a u8. If the value doesn't fit (> 255), return Err with
// the original value.
//
// HINT: u8::try_from(value).map_err(|_| original).
fn shrink_to_u8(_value: u32) -> Result<u8, u32> {
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
