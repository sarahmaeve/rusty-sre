// =============================================================================
// Challenge 04: Result and `?` — Debug Solution
// =============================================================================
//
// Bugs fixed in this file:
//
//   1. parse_alert now returns Result<Alert, IngestError>
//   2. Added impl From<ParseIntError> for IngestError so `?` works
//   3. parse_all now returns Result<Vec<Alert>, IngestError> and uses
//      `collect::<Result<Vec<_>, _>>()` to short-circuit on the first error
//   4. process now propagates parse_all's error with `?` instead of `.ok()`
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

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    service: String,
    severity: u8,
    count: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct ServiceSummary {
    total_alerts: u32,
    max_severity: u8,
}

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

// FIX 2: From<ParseIntError> so `?` on numeric parses promotes into
// IngestError::Parse automatically.
impl From<ParseIntError> for IngestError {
    fn from(e: ParseIntError) -> Self {
        IngestError::Parse(e)
    }
}

// FIX 1: return type is now Result<Alert, IngestError>.
fn parse_alert(line: &str) -> Result<Alert, IngestError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(IngestError::BadLine(line.to_string()));
    }

    let service = parts[0].to_string();
    let severity: u8 = parts[1].parse()?;
    let count: u32 = parts[2].parse()?;

    if !(1..=5).contains(&severity) {
        return Err(IngestError::BadSeverity(severity));
    }

    Ok(Alert { service, severity, count })
}

// FIX 3: collect into Result<Vec<_>, _> short-circuits on the first error.
fn parse_all(lines: &[&str]) -> Result<Vec<Alert>, IngestError> {
    lines.iter().map(|line| parse_alert(line)).collect()
}

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

// FIX 4: propagate parse_all's error with `?` instead of swallowing with .ok().
fn process(lines: &[&str]) -> Result<HashMap<String, ServiceSummary>, IngestError> {
    let alerts = parse_all(lines)?;
    Ok(summarize(&alerts))
}

fn sample_alerts() -> Vec<&'static str> {
    vec![
        "auth 3 17",
        "auth 5 4",
        "payments 2 9",
        "search 4 22",
        "search 3 6",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_lines() -> Vec<&'static str> {
        vec!["auth 3 17", "payments 2 9", "search 4 22"]
    }

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
        let r = parse_alert("auth 9 17");
        assert!(matches!(r, Err(IngestError::BadSeverity(9))));
    }

    #[test]
    fn parse_alert_bad_number() {
        let r = parse_alert("auth abc 17");
        assert!(matches!(r, Err(IngestError::Parse(_))));
    }

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

    #[test]
    fn process_returns_err_on_bad_input() {
        let lines = vec!["auth 3 17", "broken-line", "search 4 22"];
        let r = process(&lines);
        assert!(r.is_err());
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
