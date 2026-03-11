// ============================================================================
// Challenge 03: derive — Skeleton Challenge: "Alert Pipeline"
// ============================================================================
//
// SRE Scenario: Build an alert dedup and routing pipeline.
// Complete the 6 TODO tasks below. Each builds on the previous.
//
// Run tests with:
//     rustc skeleton.rs --edition 2021 --test && ./skeleton
//
// Tests will fail until you complete all TODOs.

use std::collections::HashSet;
use std::fmt;

// ── Provided types (do NOT modify) ──────────────────────────────────────────

/// Alert severity levels. Ordered by declaration: Info < Warning < Error < Critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// An alert from a monitored service.
#[derive(Debug, Clone)]
struct Alert {
    id: String,
    service: String,
    severity: Severity,
    message: String,
    timestamp: u64,
}

/// Configuration for the alert pipeline.
#[derive(Debug, Clone, PartialEq)]
struct AlertConfig {
    min_severity: Severity,
    dedup_window_secs: u64,
    max_alerts_per_service: usize,
    notify_slack: bool,
    notify_pagerduty: bool,
}

// ── TODO 1: Implement Display for Severity ──────────────────────────────────
//
// Format severity as an uppercase string:
//   Info → "INFO", Warning → "WARNING", Error → "ERROR", Critical → "CRITICAL"
//
// Python equivalent: __str__ method
// Hint: use `match self { ... }` with `write!(f, "...")`

// TODO: Implement fmt::Display for Severity
// impl fmt::Display for Severity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// ── TODO 2: Implement Display for Alert ─────────────────────────────────────
//
// Format as: "[SEVERITY] service: message"
// Example:   "[CRITICAL] auth: Connection pool exhausted"
//
// This uses Severity's Display from TODO 1.
// Hint: write!(f, "[{}] {}: {}", self.severity, ...)

// TODO: Implement fmt::Display for Alert
// impl fmt::Display for Alert {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// ── TODO 3: Implement PartialEq and Eq for Alert ───────────────────────────
//
// Custom equality: two alerts are equal if they have the same
// service, severity, AND message. Ignore id and timestamp.
//
// This is the foundation for dedup — alerts about the same problem
// from the same service should be considered duplicates.
//
// Also implement Eq (marker trait, empty body).
//
// Hint:
//   self.service == other.service && self.severity == ...

// TODO: Implement PartialEq for Alert
// impl PartialEq for Alert {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }
// impl Eq for Alert {}

// ── TODO 4: Implement Default for AlertConfig ───────────────────────────────
//
// Sensible SRE defaults:
//   min_severity:          Severity::Warning
//   dedup_window_secs:     300
//   max_alerts_per_service: 100
//   notify_slack:          true
//   notify_pagerduty:      false
//
// Python equivalent: __init__(self, min_severity="warning", ...)

// TODO: Implement Default for AlertConfig
// impl Default for AlertConfig {
//     fn default() -> Self {
//         todo!()
//     }
// }

// ── TODO 5: Implement Ord and PartialOrd for Alert ──────────────────────────
//
// Sort alerts by:
//   1. Severity DESCENDING (Critical first, Info last)
//   2. Service ASCENDING (alphabetical) as tiebreaker
//
// For PartialOrd, delegate to Ord: Some(self.cmp(other))
//
// Requires: PartialEq and Eq from TODO 3
//
// Hint: Reverse severity comparison with `other.severity.cmp(&self.severity)`
//       Chain with `.then_with(|| self.service.cmp(&other.service))`

// TODO: Implement Ord for Alert
// impl Ord for Alert {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         todo!()
//     }
// }
//
// impl PartialOrd for Alert {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         todo!()
//     }
// }

// ── TODO 6: Implement dedup_alerts ──────────────────────────────────────────
//
// Deduplicate a list of alerts, keeping the EARLIEST occurrence of each
// unique (service, severity, message) combination.
//
// Use a HashSet<(String, Severity, String)> to track seen combinations.
// For each alert, create a tuple (service.clone(), severity, message.clone())
// and check if it's been seen before. If not, add it to the result.
//
// Signature:
//   fn dedup_alerts(alerts: Vec<Alert>) -> Vec<Alert>

// TODO: Implement dedup_alerts
// fn dedup_alerts(alerts: Vec<Alert>) -> Vec<Alert> {
//     todo!()
// }

// ============================================================================
// Tests — these will pass once all TODOs are complete
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ── TODO 1 tests: Display for Severity ──────────────────────────────

    #[test]
    fn test_severity_display_info() {
        assert_eq!(format!("{}", Severity::Info), "INFO");
    }

    #[test]
    fn test_severity_display_critical() {
        assert_eq!(format!("{}", Severity::Critical), "CRITICAL");
    }

    #[test]
    fn test_severity_to_string() {
        // Display automatically provides .to_string()
        assert_eq!(Severity::Warning.to_string(), "WARNING");
        assert_eq!(Severity::Error.to_string(), "ERROR");
    }

    // ── TODO 2 tests: Display for Alert ─────────────────────────────────

    #[test]
    fn test_alert_display() {
        let alert = Alert {
            id: "a1".into(),
            service: "auth".into(),
            severity: Severity::Critical,
            message: "Connection pool exhausted".into(),
            timestamp: 1000,
        };
        assert_eq!(
            alert.to_string(),
            "[CRITICAL] auth: Connection pool exhausted"
        );
    }

    #[test]
    fn test_alert_display_info() {
        let alert = Alert {
            id: "a2".into(),
            service: "web".into(),
            severity: Severity::Info,
            message: "Deployment complete".into(),
            timestamp: 2000,
        };
        assert_eq!(alert.to_string(), "[INFO] web: Deployment complete");
    }

    // ── TODO 3 tests: PartialEq for Alert ───────────────────────────────

    #[test]
    fn test_alert_eq_same_content() {
        let a1 = Alert {
            id: "a1".into(),
            service: "db".into(),
            severity: Severity::Error,
            message: "replication lag".into(),
            timestamp: 1000,
        };
        let a2 = Alert {
            id: "a2".into(),
            service: "db".into(),
            severity: Severity::Error,
            message: "replication lag".into(),
            timestamp: 5000,
        };
        assert_eq!(a1, a2); // Same service+severity+message
    }

    #[test]
    fn test_alert_ne_different_severity() {
        let a1 = Alert {
            id: "a1".into(),
            service: "db".into(),
            severity: Severity::Error,
            message: "replication lag".into(),
            timestamp: 1000,
        };
        let a2 = Alert {
            id: "a2".into(),
            service: "db".into(),
            severity: Severity::Warning,
            message: "replication lag".into(),
            timestamp: 1000,
        };
        assert_ne!(a1, a2); // Different severity
    }

    #[test]
    fn test_alert_ne_different_service() {
        let a1 = Alert {
            id: "a1".into(),
            service: "db".into(),
            severity: Severity::Error,
            message: "timeout".into(),
            timestamp: 1000,
        };
        let a2 = Alert {
            id: "a2".into(),
            service: "cache".into(),
            severity: Severity::Error,
            message: "timeout".into(),
            timestamp: 1000,
        };
        assert_ne!(a1, a2); // Different service
    }

    // ── TODO 4 tests: Default for AlertConfig ───────────────────────────

    #[test]
    fn test_config_defaults() {
        let config = AlertConfig::default();
        assert_eq!(config.min_severity, Severity::Warning);
        assert_eq!(config.dedup_window_secs, 300);
        assert_eq!(config.max_alerts_per_service, 100);
        assert!(config.notify_slack);
        assert!(!config.notify_pagerduty);
    }

    #[test]
    fn test_config_struct_update_syntax() {
        // Override just pagerduty, keep all other defaults
        let config = AlertConfig {
            notify_pagerduty: true,
            ..Default::default()
        };
        assert!(config.notify_pagerduty);
        assert_eq!(config.min_severity, Severity::Warning); // still default
        assert!(config.notify_slack); // still default
    }

    // ── TODO 5 tests: Ord for Alert ─────────────────────────────────────

    #[test]
    fn test_alert_sort_by_severity_desc() {
        let mut alerts = vec![
            Alert {
                id: "1".into(),
                service: "web".into(),
                severity: Severity::Info,
                message: "ok".into(),
                timestamp: 0,
            },
            Alert {
                id: "2".into(),
                service: "web".into(),
                severity: Severity::Critical,
                message: "down".into(),
                timestamp: 0,
            },
            Alert {
                id: "3".into(),
                service: "web".into(),
                severity: Severity::Warning,
                message: "slow".into(),
                timestamp: 0,
            },
        ];
        alerts.sort();
        assert_eq!(alerts[0].severity, Severity::Critical);
        assert_eq!(alerts[1].severity, Severity::Warning);
        assert_eq!(alerts[2].severity, Severity::Info);
    }

    #[test]
    fn test_alert_sort_tiebreak_by_service() {
        let mut alerts = vec![
            Alert {
                id: "1".into(),
                service: "web".into(),
                severity: Severity::Critical,
                message: "down".into(),
                timestamp: 0,
            },
            Alert {
                id: "2".into(),
                service: "auth".into(),
                severity: Severity::Critical,
                message: "down".into(),
                timestamp: 0,
            },
            Alert {
                id: "3".into(),
                service: "db".into(),
                severity: Severity::Critical,
                message: "down".into(),
                timestamp: 0,
            },
        ];
        alerts.sort();
        assert_eq!(alerts[0].service, "auth");
        assert_eq!(alerts[1].service, "db");
        assert_eq!(alerts[2].service, "web");
    }

    // ── TODO 6 tests: dedup_alerts ──────────────────────────────────────

    #[test]
    fn test_dedup_removes_duplicates() {
        let alerts = vec![
            Alert {
                id: "a1".into(),
                service: "db".into(),
                severity: Severity::Error,
                message: "timeout".into(),
                timestamp: 1000,
            },
            Alert {
                id: "a2".into(),
                service: "db".into(),
                severity: Severity::Error,
                message: "timeout".into(),
                timestamp: 2000,
            },
            Alert {
                id: "a3".into(),
                service: "db".into(),
                severity: Severity::Error,
                message: "timeout".into(),
                timestamp: 3000,
            },
        ];
        let deduped = dedup_alerts(alerts);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].id, "a1"); // Keeps earliest
    }

    #[test]
    fn test_dedup_keeps_different_alerts() {
        let alerts = vec![
            Alert {
                id: "a1".into(),
                service: "db".into(),
                severity: Severity::Error,
                message: "timeout".into(),
                timestamp: 1000,
            },
            Alert {
                id: "a2".into(),
                service: "auth".into(),
                severity: Severity::Error,
                message: "timeout".into(),
                timestamp: 1000,
            },
            Alert {
                id: "a3".into(),
                service: "db".into(),
                severity: Severity::Warning,
                message: "slow query".into(),
                timestamp: 1000,
            },
        ];
        let deduped = dedup_alerts(alerts);
        assert_eq!(deduped.len(), 3); // All different
    }

    #[test]
    fn test_dedup_empty() {
        let deduped = dedup_alerts(vec![]);
        assert!(deduped.is_empty());
    }
}

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc skeleton.rs --edition 2021 --test && ./skeleton");
}
