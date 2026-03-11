// ============================================================================
// Challenge 03a: Derive Ecosystem — Skeleton: "Incident Report Pipeline"
// ============================================================================
//
// SRE Scenario: Build an incident report system that can serialize reports
// to a text format, deserialize them back, parse CLI arguments, and handle
// errors cleanly — all patterns that serde, thiserror, and clap automate.
//
// Complete the 6 TODO tasks below. Each builds on the previous.
//
// Run tests with:
//     rustc skeleton.rs --edition 2021 --test && ./skeleton

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// ── Provided types (do NOT modify) ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IncidentReport {
    id: String,
    service: String,
    severity: Severity,
    summary: String,
    timestamp: u64,
}

// ── TODO 1: Implement Display and FromStr for Severity ──────────────────────
//
// Display: format as lowercase string:
//   Low → "low", Medium → "medium", High → "high", Critical → "critical"
//
// FromStr: parse a lowercase string back into a Severity.
//   "low" → Ok(Low), "medium" → Ok(Medium), etc.
//   Anything else → Err with a descriptive message.
//
// These two must be inverses: severity.to_string().parse() == Ok(severity)
//
// Python equivalent:
//   Display  = __str__
//   FromStr  = a classmethod like Severity.from_string("low")

// TODO: Implement fmt::Display for Severity
// impl fmt::Display for Severity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// TODO: Implement FromStr for Severity
// impl FromStr for Severity {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }

// ── TODO 2: Implement Display for IncidentReport (serialization) ────────────
//
// Format as a logfmt-style string with ALL fields, space-separated:
//   "id=INC001 service=auth severity=critical summary=Login_failures timestamp=1700000000"
//
// Note: replace spaces in the summary with underscores for logfmt safety.
//
// This is what serde::Serialize does — converts a struct to a portable format.
//
// Hint: use self.summary.replace(' ', "_")

// TODO: Implement fmt::Display for IncidentReport
// impl fmt::Display for IncidentReport {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// ── TODO 3: Implement FromStr for IncidentReport (deserialization) ──────────
//
// Parse a logfmt string back into an IncidentReport.
//
// Steps:
//   1. Split input on whitespace to get "key=value" tokens
//   2. Split each token on '=' to get (key, value) pairs
//   3. Store pairs in a HashMap<&str, &str>
//   4. Extract each field, parsing numbers and Severity as needed
//   5. For summary: replace underscores back to spaces
//
// Return Err(String) with a descriptive message if:
//   - A token has no '=' separator
//   - A required field is missing
//   - A numeric field can't be parsed
//
// This is what serde::Deserialize does.
//
// Hint: use fields.get("severity").ok_or("missing field: severity")?

// TODO: Implement FromStr for IncidentReport
// impl FromStr for IncidentReport {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }

// ── TODO 4: Create a ReportError enum with Display and Error ────────────────
//
// Define an error enum for the report pipeline with THREE variants:
//
//   ParseError(String)     — failed to parse a report from text
//   MissingField(String)   — a required CLI argument was not provided
//   InvalidArg { flag: String, reason: String } — a CLI argument had a bad value
//
// Implement:
//   - fmt::Display — a human-readable message for each variant:
//       ParseError:   "parse error: {message}"
//       MissingField: "missing required argument: '{field}'"
//       InvalidArg:   "invalid value for '{flag}': {reason}"
//   - std::error::Error — default impl (no source override needed)
//   - #[derive(Debug)] on the enum
//
// This is what thiserror::Error generates from #[error("...")] attributes.

// TODO: Define and implement ReportError
// #[derive(Debug)]
// enum ReportError {
//     ...
// }
//
// impl fmt::Display for ReportError { ... }
// impl std::error::Error for ReportError {}

// ── TODO 5: Implement From conversions for ReportError ──────────────────────
//
// Implement From<String> for ReportError — converts a String error into
// ReportError::ParseError. This lets the ? operator automatically convert
// String errors (from our FromStr impls) into ReportErrors.
//
// This is what thiserror's #[from] attribute generates.
//
// Python equivalent: except ValueError as e: raise ParseError(str(e)) from e

// TODO: Implement From<String> for ReportError
// impl From<String> for ReportError {
//     fn from(msg: String) -> Self {
//         todo!()
//     }
// }

// ── TODO 6: Implement parse_report_args ─────────────────────────────────────
//
// Parse CLI-style arguments into an IncidentReport.
//
// Supported flags:
//   --id VALUE          (required)
//   --service VALUE     (required)
//   --severity VALUE    (required, must be valid Severity)
//   --summary VALUE     (required)
//   --timestamp VALUE   (required, must be a u64)
//
// Return ReportError::MissingField if a required flag is missing.
// Return ReportError::InvalidArg if a value can't be parsed.
//
// This is what clap::Parser::parse_from() does automatically.
//
// Hint: iterate args with a while loop and index, match on flag names,
//       advance index to read the value.

// TODO: Implement parse_report_args
// fn parse_report_args(args: &[String]) -> Result<IncidentReport, ReportError> {
//     todo!()
// }

// ============================================================================
// Tests — these will pass once all TODOs are complete
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ── TODO 1 tests: Display and FromStr for Severity ──────────────────

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Low.to_string(), "low");
        assert_eq!(Severity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_severity_fromstr() {
        assert_eq!("high".parse::<Severity>(), Ok(Severity::High));
        assert_eq!("medium".parse::<Severity>(), Ok(Severity::Medium));
        assert!("unknown".parse::<Severity>().is_err());
    }

    #[test]
    fn test_severity_roundtrip() {
        for sev in [Severity::Low, Severity::Medium, Severity::High, Severity::Critical] {
            let s = sev.to_string();
            let parsed: Severity = s.parse().unwrap();
            assert_eq!(sev, parsed);
        }
    }

    // ── TODO 2 tests: Display for IncidentReport ────────────────────────

    #[test]
    fn test_report_display() {
        let report = IncidentReport {
            id: "INC001".into(),
            service: "auth".into(),
            severity: Severity::Critical,
            summary: "Login failures".into(),
            timestamp: 1700000000,
        };
        let output = report.to_string();
        assert!(output.contains("id=INC001"));
        assert!(output.contains("service=auth"));
        assert!(output.contains("severity=critical"));
        assert!(output.contains("summary=Login_failures")); // spaces → underscores
        assert!(output.contains("timestamp=1700000000"));
    }

    #[test]
    fn test_report_display_no_spaces_in_summary() {
        let report = IncidentReport {
            id: "INC002".into(),
            service: "db".into(),
            severity: Severity::High,
            summary: "Replication lag too high".into(),
            timestamp: 1700000001,
        };
        let output = report.to_string();
        assert!(output.contains("summary=Replication_lag_too_high"));
        assert!(!output.contains("summary=Replication lag")); // no raw spaces in value
    }

    // ── TODO 3 tests: FromStr for IncidentReport ────────────────────────

    #[test]
    fn test_report_fromstr() {
        let input = "id=INC001 service=auth severity=critical summary=Login_failures timestamp=1700000000";
        let report: IncidentReport = input.parse().unwrap();
        assert_eq!(report.id, "INC001");
        assert_eq!(report.service, "auth");
        assert_eq!(report.severity, Severity::Critical);
        assert_eq!(report.summary, "Login failures"); // underscores → spaces
        assert_eq!(report.timestamp, 1700000000);
    }

    #[test]
    fn test_report_fromstr_missing_field() {
        let input = "id=INC001 service=auth";
        let result = input.parse::<IncidentReport>();
        assert!(result.is_err());
    }

    #[test]
    fn test_report_roundtrip() {
        let original = IncidentReport {
            id: "INC042".into(),
            service: "cache".into(),
            severity: Severity::Medium,
            summary: "Cache hit rate dropping".into(),
            timestamp: 1700099999,
        };
        let serialized = original.to_string();
        let deserialized: IncidentReport = serialized.parse().unwrap();
        assert_eq!(original, deserialized);
    }

    // ── TODO 4 tests: ReportError ───────────────────────────────────────

    #[test]
    fn test_error_display_parse() {
        let err = ReportError::ParseError("bad input".into());
        assert_eq!(err.to_string(), "parse error: bad input");
    }

    #[test]
    fn test_error_display_missing() {
        let err = ReportError::MissingField("--service".into());
        assert_eq!(err.to_string(), "missing required argument: '--service'");
    }

    #[test]
    fn test_error_display_invalid() {
        let err = ReportError::InvalidArg {
            flag: "--port".into(),
            reason: "not a number".into(),
        };
        assert_eq!(err.to_string(), "invalid value for '--port': not a number");
    }

    // ── TODO 5 tests: From<String> for ReportError ──────────────────────

    #[test]
    fn test_from_string_conversion() {
        let err: ReportError = ReportError::from("something failed".to_string());
        assert!(matches!(err, ReportError::ParseError(_)));
        assert!(err.to_string().contains("something failed"));
    }

    #[test]
    fn test_question_mark_with_from() {
        // Simulate ? converting String error to ReportError
        fn try_parse() -> Result<Severity, ReportError> {
            let sev: Severity = "invalid".parse()?; // String err → ReportError via From
            Ok(sev)
        }
        let result = try_parse();
        assert!(matches!(result, Err(ReportError::ParseError(_))));
    }

    // ── TODO 6 tests: parse_report_args ─────────────────────────────────

    #[test]
    fn test_parse_args_full() {
        let args: Vec<String> = vec![
            "--id", "INC001",
            "--service", "api",
            "--severity", "high",
            "--summary", "Request timeouts",
            "--timestamp", "1700000000",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let report = parse_report_args(&args).unwrap();
        assert_eq!(report.id, "INC001");
        assert_eq!(report.service, "api");
        assert_eq!(report.severity, Severity::High);
        assert_eq!(report.summary, "Request timeouts");
        assert_eq!(report.timestamp, 1700000000);
    }

    #[test]
    fn test_parse_args_missing_field() {
        let args: Vec<String> = vec!["--id", "INC001", "--service", "api"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = parse_report_args(&args);
        assert!(matches!(result, Err(ReportError::MissingField(_))));
    }

    #[test]
    fn test_parse_args_invalid_severity() {
        let args: Vec<String> = vec![
            "--id", "INC001",
            "--service", "api",
            "--severity", "mega",
            "--summary", "bad",
            "--timestamp", "100",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let result = parse_report_args(&args);
        assert!(matches!(result, Err(ReportError::InvalidArg { .. })));
    }
}

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc skeleton.rs --edition 2021 --test && ./skeleton");
}
