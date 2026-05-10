// =============================================================================
// Challenge 07: Numbers and Conversions — Debug Solution
// =============================================================================
//
// Bugs fixed:
//
//   1. project_utilization: widen `used: u32` to u64 with `u64::from(used)`
//      before adding to `max: u64`. (Could also use `used as u64`, but
//      From/Into is the lossless-by-construction style.)
//   2. count_chunks: `chunks.len()` returns usize. Convert with try_into
//      and saturate to u32::MAX on overflow — for a chunk count this is
//      defensible. (Alternative: propagate the error as Result<u32, _>.)
//   3. total_bytes: replace `total + c` with `total.saturating_add(c)` so
//      the result clamps at u32::MAX instead of panicking or wrapping.
//   4. throughputs_agree: replace `==` with an absolute-epsilon compare.
// =============================================================================

#[derive(Debug, PartialEq)]
pub struct Report {
    pub total_used: u32,
    pub chunk_count: u32,
}

// FIX 1: widen the u32 to u64 before the add.
fn project_utilization(used: u32, max: u64) -> u64 {
    u64::from(used) + max
}

// FIX 2: usize → u32 with saturation. (Alternative: return Result.)
fn count_chunks(chunks: &[Vec<u8>]) -> u32 {
    u32::try_from(chunks.len()).unwrap_or(u32::MAX)
}

// FIX 3: saturating_add never panics, never silently wraps.
fn total_bytes(counters: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for &c in counters {
        total = total.saturating_add(c);
    }
    total
}

// FIX 4: compare with an absolute epsilon. For a real metric you'd
// parameterize this — here, 1e-9 catches the 0.1+0.2 class of jitter.
fn throughputs_agree(combined: f64, summed_halves: f64) -> bool {
    (combined - summed_halves).abs() < 1e-9
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_utilization_basic() {
        assert_eq!(project_utilization(100, 1_000_000), 1_000_100);
    }

    #[test]
    fn project_utilization_handles_large_max() {
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
        let a = 0.1_f64;
        let b = 0.2_f64;
        let combined = (a + b) / 2.0;
        let summed_halves = a / 2.0 + b / 2.0;
        assert!(throughputs_agree(combined, summed_halves));
    }

    #[test]
    fn throughputs_agree_clearly_different() {
        assert!(!throughputs_agree(0.1, 0.9));
    }

    #[test]
    fn build_report_basic() {
        let chunks = vec![vec![0u8; 16], vec![0u8; 32]];
        let counters = vec![100, 200];
        let report = build_report(&chunks, &counters);
        assert_eq!(report, Report { total_used: 300, chunk_count: 2 });
    }
}
