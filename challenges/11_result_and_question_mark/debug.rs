// =============================================================================
// Challenge 11: Result and `?` — Debug the Alert Ingestion Pipeline
// =============================================================================
//
// This program reads alert records from text and aggregates them by service.
// It contains FOUR bugs related to Result and the `?` operator:
//
//   1. parse_alert returns the wrong type (compile error)
//   2. Missing `From<ParseIntError> for IngestError` (compile error)
//   3. parse_all calls .unwrap() instead of propagating (runtime panic)
//   4. process silently swallows errors with .ok() (runtime: hidden failure)
//
// Bugs 1 and 2 are entangled — the file won't compile until you fix both.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    let raw = sample_alerts();
    match process(&raw) {
        Ok(report) => println!("{report:#?}"),
        Err(e) => eprintln!("error: {e}"),
    }
}

// -----------------------------------------------------------------------------
// Domain types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    service: String,
    severity: u8, // 1..=5
    count: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct ServiceSummary {
    total_alerts: u32,
    max_severity: u8,
}

// -----------------------------------------------------------------------------
// Error type
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum IngestError {
    BadLine(String),
    BadSeverity(u8),
    Parse(ParseIntError),
}

impl fmt::Display for IngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngestError::BadLine(line) => write!(f, "malformed line: {line}"),
            IngestError::BadSeverity(s) => write!(f, "severity out of range: {s}"),
            IngestError::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}

impl Error for IngestError {}

// BUG 2: NO `impl From<ParseIntError> for IngestError`. parse_alert uses `?`
// on a u8/u32 parse, so this won't compile until you add the conversion.

// -----------------------------------------------------------------------------
// BUG 1: parse_alert returns the wrong type
// -----------------------------------------------------------------------------
// This function should return Result<Alert, IngestError>. The signature below
// returns Alert directly, which won't compile because we use `?` and Err(...)
// inside.
//
// Lines look like "auth 3 17" — service severity count, whitespace separated.

fn parse_alert(line: &str) -> Alert {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(IngestError::BadLine(line.to_string()));
    }

    let service = parts[0].to_string();
    let severity: u8 = parts[1].parse()?; // needs From<ParseIntError> (BUG 2)
    let count: u32 = parts[2].parse()?; // same

    if !(1..=5).contains(&severity) {
        return Err(IngestError::BadSeverity(severity));
    }

    Ok(Alert { service, severity, count })
}

// -----------------------------------------------------------------------------
// BUG 3: parse_all calls .unwrap() and panics on bad input
// -----------------------------------------------------------------------------
// parse_all should propagate the first error rather than panic. Replace
// `.unwrap()` with proper propagation (`?`, or collect into Result<Vec<_>, _>).

fn parse_all(lines: &[&str]) -> Vec<Alert> {
    lines.iter().map(|line| parse_alert(line).unwrap()).collect()
}

// -----------------------------------------------------------------------------
// Aggregate alerts per service. (No bug here — but it depends on parse_all
// being correct.)
// -----------------------------------------------------------------------------

fn summarize(alerts: &[Alert]) -> HashMap<String, ServiceSummary> {
    let mut out: HashMap<String, ServiceSummary> = HashMap::new();
    for a in alerts {
        let entry = out.entry(a.service.clone()).or_default();
        entry.total_alerts += a.count;
        if a.severity > entry.max_severity {
            entry.max_severity = a.severity;
        }
    }
    out
}

// -----------------------------------------------------------------------------
// BUG 4: process silently swallows errors via .ok()
// -----------------------------------------------------------------------------
// Callers rely on process() to return Err on bad input. But .ok() turns Err
// into None, and the unwrap_or_default makes the function look like it
// succeeded with empty data — masking real ingest failures.

fn process(lines: &[&str]) -> Result<HashMap<String, ServiceSummary>, IngestError> {
    let alerts: Vec<Alert> = parse_all(lines).ok().unwrap_or_default();
    Ok(summarize(&alerts))
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn sample_alerts() -> Vec<&'static str> {
    vec![
        "auth 3 17",
        "auth 5 4",
        "payments 2 9",
        "search 4 22",
        "search 3 6",
    ]
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn good_lines() -> Vec<&'static str> {
        vec!["auth 3 17", "payments 2 9", "search 4 22"]
    }

    // BUG 1 + 2: parse_alert must return Result<_, IngestError> with From<ParseIntError>
    #[test]
    fn parse_alert_valid() {
        let a = parse_alert("auth 3 17").unwrap();
        assert_eq!(a, Alert { service: "auth".to_string(), severity: 3, count: 17 });
    }

    #[test]
    fn parse_alert_bad_format() {
        let r = parse_alert("auth 3");
        assert!(matches!(r, Err(IngestError::BadLine(_))));
    }

    #[test]
    fn parse_alert_bad_severity_value() {
        let r = parse_alert("auth 9 17"); // out of 1..=5
        assert!(matches!(r, Err(IngestError::BadSeverity(9))));
    }

    #[test]
    fn parse_alert_bad_number() {
        let r = parse_alert("auth abc 17");
        assert!(matches!(r, Err(IngestError::Parse(_))));
    }

    // BUG 3: parse_all must propagate errors instead of panicking
    #[test]
    fn parse_all_happy_path() {
        let alerts = parse_all(&good_lines()).unwrap();
        assert_eq!(alerts.len(), 3);
    }

    #[test]
    fn parse_all_propagates_error() {
        let lines = vec!["auth 3 17", "payments oops 4", "search 4 22"];
        let r = parse_all(&lines);
        assert!(matches!(r, Err(IngestError::Parse(_))));
    }

    // BUG 4: process must surface errors instead of swallowing them
    #[test]
    fn process_returns_err_on_bad_input() {
        let lines = vec!["auth 3 17", "broken-line", "search 4 22"];
        let r = process(&lines);
        assert!(r.is_err(), "process should return Err on malformed input");
    }

    #[test]
    fn process_aggregates_when_clean() {
        let lines = vec!["auth 3 5", "auth 5 2", "payments 2 9"];
        let summary = process(&lines).unwrap();
        let auth = summary.get("auth").unwrap();
        assert_eq!(auth.total_alerts, 7);
        assert_eq!(auth.max_severity, 5);
    }
}
