// =============================================================================
// Challenge 07: Numbers and Conversions — Debug the Capacity Planner
// =============================================================================
//
// This program plans memory-pool capacity for a service: it projects
// utilization, counts chunks, sums byte deltas, and reports when usage
// matches a target. It contains FOUR numeric bugs:
//
//   1. project_utilization mixes u32 and u64 in arithmetic (compile error)
//   2. count_chunks returns u32 from a usize value (compile error)
//   3. total_bytes uses plain `+` and panics on integer overflow (runtime)
//   4. throughputs_agree uses == on f64 derived from arithmetic (runtime)
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

#[derive(Debug, PartialEq)]
pub struct Report {
    pub total_used: u32,
    pub chunk_count: u32,
}

// -----------------------------------------------------------------------------
// BUG 1: u32 + u64 — Rust does no implicit widening
// -----------------------------------------------------------------------------
// The two operands have different types, so `+` doesn't apply. Convert one
// side; widening u32 → u64 is lossless, so `u64::from(used) + max` works.
fn project_utilization(used: u32, max: u64) -> u64 {
    used + max
}

// -----------------------------------------------------------------------------
// BUG 2: returns u32 from a usize
// -----------------------------------------------------------------------------
// Vec::len() returns usize, not u32. The two are different types and won't
// silently convert. Use try_into() and propagate or saturate the overflow.
fn count_chunks(chunks: &[Vec<u8>]) -> u32 {
    chunks.len()
}

// -----------------------------------------------------------------------------
// BUG 3: integer overflow panics on plain `+`
// -----------------------------------------------------------------------------
// Summing a slice that totals more than u32::MAX panics in debug builds and
// wraps to a smaller-than-expected value in release builds. Both are wrong
// for an SRE counter — saturate at u32::MAX so the report says "lots" loudly.
fn total_bytes(counters: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for &c in counters {
        total = total + c;
    }
    total
}

// -----------------------------------------------------------------------------
// BUG 4: f64 == on derived value
// -----------------------------------------------------------------------------
// Two arithmetically-equivalent expressions can produce slightly different
// f64 results. `combined == summed_halves` rejects values that are actually
// "close enough." Compare with an absolute epsilon instead.
fn throughputs_agree(combined: f64, summed_halves: f64) -> bool {
    combined == summed_halves
}

// Build a small report. Depends on count_chunks and total_bytes being right.
fn build_report(chunks: &[Vec<u8>], counters: &[u32]) -> Report {
    Report {
        total_used: total_bytes(counters),
        chunk_count: count_chunks(chunks),
    }
}

fn main() {
    let chunks = vec![vec![0u8; 64], vec![0u8; 128]];
    let counters = vec![100, 200, 300];
    let report = build_report(&chunks, &counters);
    println!("{report:#?}");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // BUG 1: project_utilization must compile (u64 sum of u32 used + u64 max)
    #[test]
    fn project_utilization_basic() {
        assert_eq!(project_utilization(100, 1_000_000), 1_000_100);
    }

    #[test]
    fn project_utilization_handles_large_max() {
        // 5 GB-ish — u32 used fits, u64 max is much bigger.
        let max: u64 = 5 * 1024 * 1024 * 1024;
        assert_eq!(project_utilization(2048, max), max + 2048);
    }

    // BUG 2: count_chunks must compile (usize → u32 conversion)
    #[test]
    fn count_chunks_basic() {
        let chunks = vec![vec![0u8; 16], vec![0u8; 32], vec![0u8; 8]];
        assert_eq!(count_chunks(&chunks), 3);
    }

    #[test]
    fn count_chunks_empty() {
        let chunks: Vec<Vec<u8>> = vec![];
        assert_eq!(count_chunks(&chunks), 0);
    }

    // BUG 3: total_bytes must saturate, not panic, on overflow
    #[test]
    fn total_bytes_normal() {
        let counters = vec![100, 200, 300];
        assert_eq!(total_bytes(&counters), 600);
    }

    #[test]
    fn total_bytes_saturates_on_overflow() {
        // u32::MAX + 100 would overflow. After the fix this returns u32::MAX,
        // not a panic and not a wrap to 99.
        let counters = vec![u32::MAX, 100];
        assert_eq!(total_bytes(&counters), u32::MAX);
    }

    // BUG 4: throughputs_agree must accept arithmetically-equal values
    #[test]
    fn throughputs_agree_exact() {
        assert!(throughputs_agree(0.5, 0.5));
        assert!(throughputs_agree(0.0, 0.0));
    }

    #[test]
    fn throughputs_agree_with_arith() {
        // Two arithmetically equivalent expressions, slightly different in
        // float math. After the fix, throughputs_agree treats them as equal.
        let a = 0.1_f64;
        let b = 0.2_f64;
        let combined = (a + b) / 2.0;
        let summed_halves = a / 2.0 + b / 2.0;
        assert!(throughputs_agree(combined, summed_halves));
    }

    #[test]
    fn throughputs_agree_clearly_different() {
        // Far apart values should still be rejected.
        assert!(!throughputs_agree(0.1, 0.9));
    }

    // Integration — fix bugs 2 and 3 together to make this pass
    #[test]
    fn build_report_basic() {
        let chunks = vec![vec![0u8; 16], vec![0u8; 32]];
        let counters = vec![100, 200];
        let report = build_report(&chunks, &counters);
        assert_eq!(report, Report { total_used: 300, chunk_count: 2 });
    }
}
