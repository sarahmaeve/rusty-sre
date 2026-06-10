// ============================================================================
// Challenge 11: derive — Skeleton Solution: "Alert Pipeline" (all 6 done)
// ============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution

use std::collections::HashSet;
use std::fmt;

// ── Provided types ───────────────────────────────────────────────────────────

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

// ── TODO 1: Display for Severity ─────────────────────────────────────────────

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

// ── TODO 2: Display for Alert ────────────────────────────────────────────────

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.service, self.message)
    }
}

// ── TODO 3: PartialEq and Eq for Alert ───────────────────────────────────────
// Two alerts are equal if service, severity, and message match — id and
// timestamp are ignored so repeats of the same problem count as duplicates.

impl PartialEq for Alert {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service
            && self.severity == other.severity
            && self.message == other.message
    }
}
impl Eq for Alert {}

// ── TODO 4: Default for AlertConfig ──────────────────────────────────────────

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            min_severity: Severity::Warning,
            dedup_window_secs: 300,
            max_alerts_per_service: 100,
            notify_slack: true,
            notify_pagerduty: false,
        }
    }
}

// ── TODO 5: Ord and PartialOrd for Alert ─────────────────────────────────────
// Severity descending (note the swapped sides), service ascending tiebreak.

impl Ord for Alert {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .severity
            .cmp(&self.severity)
            .then_with(|| self.service.cmp(&other.service))
    }
}

impl PartialOrd for Alert {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ── TODO 6: dedup_alerts ─────────────────────────────────────────────────────

fn dedup_alerts(alerts: Vec<Alert>) -> Vec<Alert> {
    let mut seen: HashSet<(String, Severity, String)> = HashSet::new();
    let mut result = Vec::new();
    for alert in alerts {
        let key = (alert.service.clone(), alert.severity, alert.message.clone());
        if seen.insert(key) {
            result.push(alert);
        }
    }
    result
}

// ============================================================================
// Tests — identical to skeleton.rs
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
    println!("Reference solution — run with --test to execute the tests.");
}
