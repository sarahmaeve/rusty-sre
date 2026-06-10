// =============================================================================
// Challenge 07: Numbers and Conversions — Debug the Capacity Planner
// =============================================================================
//
// This program plans memory-pool capacity for a service: it projects
// utilization, counts chunks, sums byte deltas, and reports when usage
// matches a target. It contains FOUR numeric bugs — some stop it compiling,
// some misbehave at runtime. Find and fix all four so every test passes.
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

#[derive(Debug, PartialEq)]
pub struct Report {
    pub total_used: u32,
    pub chunk_count: u32,
}

// Project the pool utilization if `used` grows by the configured `max`.
fn project_utilization(used: u32, max: u64) -> u64 {
    used + max
}

// Number of chunks in the pool, as a u32 for the report payload.
fn count_chunks(chunks: &[Vec<u8>]) -> u32 {
    chunks.len()
}

// Sum the byte counters. SRE counters must never panic or silently wrap —
// if the true total exceeds u32::MAX, the report should pin at u32::MAX and
// say "lots" loudly.
fn total_bytes(counters: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for &c in counters {
        total = total + c;
    }
    total
}

// True when two throughput computations agree ("close enough" counts).
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

    #[test]
    fn total_bytes_normal() {
        let counters = vec![100, 200, 300];
        assert_eq!(total_bytes(&counters), 600);
    }

    #[test]
    fn total_bytes_saturates_on_overflow() {
        // Summing past u32::MAX must pin at u32::MAX — no panic, no wrap.
        let counters = vec![u32::MAX, 100];
        assert_eq!(total_bytes(&counters), u32::MAX);
    }

    #[test]
    fn throughputs_agree_exact() {
        assert!(throughputs_agree(0.5, 0.5));
        assert!(throughputs_agree(0.0, 0.0));
    }

    #[test]
    fn throughputs_agree_with_arith() {
        // Two arithmetically equivalent expressions, slightly different in
        // float math — these must count as agreeing.
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

    // Integration test
    #[test]
    fn build_report_basic() {
        let chunks = vec![vec![0u8; 16], vec![0u8; 32]];
        let counters = vec![100, 200];
        let report = build_report(&chunks, &counters);
        assert_eq!(report, Report { total_used: 300, chunk_count: 2 });
    }
}
