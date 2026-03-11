// ============================================================================
// Challenge 03a: Derive Ecosystem — Debug Solution: "SRE Log Exporter"
// ============================================================================
//
// This is the fixed version of debug.rs with all 4 bugs resolved.
//
// Run with:
//     rustc debug_solution.rs --edition 2021 --test && ./debug_solution
//
// BUG FIXES:
//   #1: Added From<ParseIntError> for ExportError so ? works on u64 parse
//   #2: Changed EventBatch's FromStr::Err from String to ExportError
//   #3: Fixed source() to return Some(err) for the Io variant
//   #4: Fixed Display for DeployEvent — was writing self.service instead of self.version

use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

// ── DeployStatus enum (unchanged) ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeployStatus {
    Pending,
    InProgress,
    Succeeded,
    Failed,
    RolledBack,
}

impl fmt::Display for DeployStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeployStatus::Pending => write!(f, "pending"),
            DeployStatus::InProgress => write!(f, "in_progress"),
            DeployStatus::Succeeded => write!(f, "succeeded"),
            DeployStatus::Failed => write!(f, "failed"),
            DeployStatus::RolledBack => write!(f, "rolled_back"),
        }
    }
}

impl FromStr for DeployStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(DeployStatus::Pending),
            "in_progress" => Ok(DeployStatus::InProgress),
            "succeeded" => Ok(DeployStatus::Succeeded),
            "failed" => Ok(DeployStatus::Failed),
            "rolled_back" => Ok(DeployStatus::RolledBack),
            other => Err(format!("unknown deploy status: '{}'", other)),
        }
    }
}

// ── DeployEvent struct ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeployEvent {
    deploy_id: String,
    service: String,
    version: String,
    status: DeployStatus,
    timestamp: u64,
}

// FIX #4: Changed self.service to self.version for the version field
impl fmt::Display for DeployEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "deploy_id={} service={} version={} status={} timestamp={}",
            self.deploy_id, self.service, self.version, self.status, self.timestamp
        )
    }
}

impl FromStr for DeployEvent {
    type Err = ExportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for token in s.split_whitespace() {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| ExportError::Parse(format!("invalid token: '{}'", token)))?;
            fields.insert(key, value);
        }

        let deploy_id = fields
            .get("deploy_id")
            .ok_or_else(|| ExportError::Parse("missing field: deploy_id".into()))?
            .to_string();

        let service = fields
            .get("service")
            .ok_or_else(|| ExportError::Parse("missing field: service".into()))?
            .to_string();

        let version = fields
            .get("version")
            .ok_or_else(|| ExportError::Parse("missing field: version".into()))?
            .to_string();

        let status: DeployStatus = fields
            .get("status")
            .ok_or_else(|| ExportError::Parse("missing field: status".into()))?
            .parse()
            .map_err(|e: String| ExportError::Parse(e))?;

        let timestamp: u64 = fields
            .get("timestamp")
            .ok_or_else(|| ExportError::Parse("missing field: timestamp".into()))?
            .parse()?;

        Ok(DeployEvent {
            deploy_id,
            service,
            version,
            status,
            timestamp,
        })
    }
}

// ── Error types ─────────────────────────────────────────────────────────────

#[derive(Debug)]
enum ExportError {
    Parse(String),
    Io(std::io::Error),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::Parse(msg) => write!(f, "parse error: {}", msg),
            ExportError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

// FIX #3: source() returns Some(err) for Io variant
impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExportError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::Io(err)
    }
}

// FIX #1: Added From<ParseIntError> so ? works on u64 parse
impl From<ParseIntError> for ExportError {
    fn from(err: ParseIntError) -> Self {
        ExportError::Parse(format!("invalid integer: {}", err))
    }
}

// FIX #2: Changed Err type from String to ExportError
struct EventBatch {
    events: Vec<DeployEvent>,
}

impl FromStr for EventBatch {
    type Err = ExportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut events = Vec::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let event: DeployEvent = line.parse()?;
            events.push(event);
        }
        Ok(EventBatch { events })
    }
}

// ── Export functions (unchanged) ────────────────────────────────────────────

fn export_batch(events: &[DeployEvent]) -> String {
    events
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn import_batch_lenient(input: &str) -> (Vec<DeployEvent>, Vec<String>) {
    let mut events = Vec::new();
    let mut errors = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line.parse::<DeployEvent>() {
            Ok(event) => events.push(event),
            Err(e) => errors.push(format!("line {}: {}", i + 1, e)),
        }
    }
    (events, errors)
}

fn read_events_from_file(path: &str) -> Result<Vec<DeployEvent>, ExportError> {
    let content = std::fs::read_to_string(path)?;
    let batch: EventBatch = content.parse()?;
    Ok(batch.events)
}

// ============================================================================
// Tests (identical to debug.rs)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_event_roundtrip() {
        let original = DeployEvent {
            deploy_id: "dep-001".into(),
            service: "api".into(),
            version: "v2.3.1".into(),
            status: DeployStatus::Succeeded,
            timestamp: 1700000000,
        };
        let serialized = original.to_string();
        let deserialized: DeployEvent = serialized.parse().unwrap();
        assert_eq!(original, deserialized, "Round-trip failed: serialized as '{}'", serialized);
    }

    #[test]
    fn test_deploy_event_parse_valid() {
        let input = "deploy_id=dep-002 service=web version=v1.0.0 status=failed timestamp=1700000001";
        let event: DeployEvent = input.parse().unwrap();
        assert_eq!(event.deploy_id, "dep-002");
        assert_eq!(event.service, "web");
        assert_eq!(event.version, "v1.0.0");
        assert_eq!(event.status, DeployStatus::Failed);
    }

    #[test]
    fn test_deploy_event_parse_invalid_timestamp() {
        let input = "deploy_id=dep-003 service=db version=v1.0 status=pending timestamp=not_a_number";
        let result: Result<DeployEvent, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_export_import_batch() {
        let events = vec![
            DeployEvent {
                deploy_id: "dep-001".into(),
                service: "api".into(),
                version: "v2.3.1".into(),
                status: DeployStatus::Succeeded,
                timestamp: 1700000000,
            },
            DeployEvent {
                deploy_id: "dep-002".into(),
                service: "web".into(),
                version: "v1.0.0".into(),
                status: DeployStatus::Failed,
                timestamp: 1700000001,
            },
        ];
        let exported = export_batch(&events);
        let (imported, errors) = import_batch_lenient(&exported);
        assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
        assert_eq!(imported.len(), 2);
        assert_eq!(events, imported);
    }

    #[test]
    fn test_import_lenient_skips_bad_lines() {
        let input = "\
deploy_id=dep-001 service=api version=v1.0 status=succeeded timestamp=100
this is garbage
deploy_id=dep-002 service=db version=v2.0 status=failed timestamp=200
";
        let (events, errors) = import_batch_lenient(input);
        assert_eq!(events.len(), 2);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("line 2"));
    }

    #[test]
    fn test_error_chain_io() {
        use std::error::Error;
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
        let export_err = ExportError::Io(io_err);

        let source = export_err.source().expect("source() should return Some for Io variant");
        assert!(source.to_string().contains("no such file"));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_events_from_file("/nonexistent/deploy_events.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("I/O error"));
    }

    #[test]
    fn test_event_batch_fromstr() {
        let input = "\
deploy_id=dep-001 service=api version=v1.0 status=succeeded timestamp=100
deploy_id=dep-002 service=db version=v2.0 status=pending timestamp=200
";
        let batch: EventBatch = input.parse().unwrap();
        assert_eq!(batch.events.len(), 2);
        assert_eq!(batch.events[0].deploy_id, "dep-001");
    }
}

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc debug_solution.rs --edition 2021 --test && ./debug_solution");
}
