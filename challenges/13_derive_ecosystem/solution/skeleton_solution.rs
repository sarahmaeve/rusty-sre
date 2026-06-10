// ============================================================================
// Challenge 13: Derive Ecosystem — Skeleton Solution: "Incident Report
// Pipeline" (all 6 tasks done)
// ============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// ── Provided types ───────────────────────────────────────────────────────────

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

// ── TODO 1: Display and FromStr for Severity ─────────────────────────────────

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Severity::Low),
            "medium" => Ok(Severity::Medium),
            "high" => Ok(Severity::High),
            "critical" => Ok(Severity::Critical),
            other => Err(format!("unknown severity: '{other}'")),
        }
    }
}

// ── TODO 2: Display for IncidentReport (serialization) ───────────────────────
// Spaces in the summary become underscores for logfmt safety.

impl fmt::Display for IncidentReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id={} service={} severity={} summary={} timestamp={}",
            self.id,
            self.service,
            self.severity,
            self.summary.replace(' ', "_"),
            self.timestamp
        )
    }
}

// ── TODO 3: FromStr for IncidentReport (deserialization) ─────────────────────

impl FromStr for IncidentReport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for token in s.split_whitespace() {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| format!("invalid token: '{token}'"))?;
            fields.insert(key, value);
        }

        let get = |key: &str| -> Result<&str, String> {
            fields
                .get(key)
                .copied()
                .ok_or_else(|| format!("missing field: {key}"))
        };

        let id = get("id")?.to_string();
        let service = get("service")?.to_string();
        let severity: Severity = get("severity")?.parse()?;
        let summary = get("summary")?.replace('_', " ");
        let timestamp: u64 = get("timestamp")?
            .parse()
            .map_err(|_| "invalid timestamp".to_string())?;

        Ok(IncidentReport { id, service, severity, summary, timestamp })
    }
}

// ── TODO 4: ReportError with Display and Error ───────────────────────────────

#[derive(Debug)]
enum ReportError {
    ParseError(String),
    MissingField(String),
    InvalidArg { flag: String, reason: String },
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportError::ParseError(message) => write!(f, "parse error: {message}"),
            ReportError::MissingField(field) => {
                write!(f, "missing required argument: '{field}'")
            }
            ReportError::InvalidArg { flag, reason } => {
                write!(f, "invalid value for '{flag}': {reason}")
            }
        }
    }
}

impl std::error::Error for ReportError {}

// ── TODO 5: From<String> for ReportError ─────────────────────────────────────

impl From<String> for ReportError {
    fn from(msg: String) -> Self {
        ReportError::ParseError(msg)
    }
}

// ── TODO 6: parse_report_args ────────────────────────────────────────────────

fn parse_report_args(args: &[String]) -> Result<IncidentReport, ReportError> {
    let mut values: HashMap<&str, &str> = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        let flag = args[i].as_str();
        let value = args
            .get(i + 1)
            .ok_or_else(|| ReportError::MissingField(flag.to_string()))?;
        values.insert(flag, value.as_str());
        i += 2;
    }

    let get = |flag: &str| -> Result<&str, ReportError> {
        values
            .get(flag)
            .copied()
            .ok_or_else(|| ReportError::MissingField(flag.to_string()))
    };

    let severity: Severity = get("--severity")?
        .parse()
        .map_err(|reason| ReportError::InvalidArg {
            flag: "--severity".to_string(),
            reason,
        })?;
    let timestamp: u64 = get("--timestamp")?
        .parse()
        .map_err(|_| ReportError::InvalidArg {
            flag: "--timestamp".to_string(),
            reason: "not a number".to_string(),
        })?;

    Ok(IncidentReport {
        id: get("--id")?.to_string(),
        service: get("--service")?.to_string(),
        severity,
        summary: get("--summary")?.to_string(),
        timestamp,
    })
}

// ============================================================================
// Tests — identical to skeleton.rs
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
    println!("Reference solution — run with --test to execute the tests.");
}
