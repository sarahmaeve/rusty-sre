//! Tests are executable contracts. Unit tests can inspect private details;
//! integration and documentation tests exercise the public API as callers do.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch11-00-testing.html>
//! - <https://doc.rust-lang.org/cargo/guide/tests.html>
//! - <https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html>
//! - <https://doc.rust-lang.org/reference/conditional-compilation.html>
//! - Source study: <https://github.com/BurntSushi/ripgrep/tree/master/tests>

use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum BackoffError {
    AttemptTooLarge,
}

/// Returns an exponential delay.
///
/// Public library functions should include fenced examples in their API docs.
/// Rustdoc compiles those examples as external callers, catching stale usage.
pub fn retry_delay(base_ms: u64, attempt: u32) -> Result<Duration, BackoffError> {
    2_u64
        .checked_pow(attempt)
        .and_then(|multiplier| base_ms.checked_mul(multiplier))
        .map(Duration::from_millis)
        .ok_or(BackoffError::AttemptTooLarge)
}

fn normalize_label(label: &str) -> String {
    label.trim().to_ascii_lowercase()
}

fn backoff_properties(base_ms: u64, attempts: std::ops::RangeInclusive<u32>) -> bool {
    let mut previous = Duration::ZERO;
    for attempt in attempts {
        let Ok(current) = retry_delay(base_ms, attempt) else {
            return false;
        };
        if current < previous || current.as_millis() % u128::from(base_ms) != 0 {
            return false;
        }
        previous = current;
    }
    true
}

#[cfg(test)]
fn fixture_label() -> &'static str {
    " API "
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_can_check_a_private_helper() {
        assert_eq!(normalize_label(fixture_label()), "api");
    }

    #[test]
    fn public_contract_reports_overflow() {
        assert_eq!(retry_delay(u64::MAX, 1), Err(BackoffError::AttemptTooLarge));
    }

    #[test]
    fn deterministic_inputs_check_general_properties() {
        for base in [1, 10, 250, 1_000] {
            assert!(backoff_properties(base, 0..=12));
        }
    }
}

fn main() {
    assert_eq!(retry_delay(25, 0).unwrap(), Duration::from_millis(25));
    assert_eq!(retry_delay(25, 3).unwrap(), Duration::from_millis(200));
    assert_eq!(normalize_label(" Queue "), "queue");

    // This property-style loop checks an invariant over many cases without a
    // property-testing dependency. Random generators and shrinking extend the
    // same idea; failures should always print or retain their reproducing input.
    for base in 1..=32 {
        assert!(backoff_properties(base, 0..=10));
    }
}
