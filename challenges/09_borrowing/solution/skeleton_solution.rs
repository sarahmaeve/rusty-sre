// =============================================================================
// Challenge 09: Ownership & Borrowing with Vectors — SRE Alert Pipeline
// (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    service: String,
    severity: String, // "critical", "warning", "info"
    message: String,
}

// -----------------------------------------------------------------------------
// Task 1: Take ownership and transform
// -----------------------------------------------------------------------------
fn parse_alerts(raw_messages: Vec<String>) -> Vec<Alert> {
    raw_messages
        .into_iter()
        .filter_map(|raw| {
            let parts: Vec<&str> = raw.splitn(3, ':').collect();
            if parts.len() == 3 {
                Some(Alert {
                    service: parts[0].to_string(),
                    severity: parts[1].to_string(),
                    message: parts[2].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Task 2: Borrow immutably to analyze
// -----------------------------------------------------------------------------
fn critical_alerts<'a>(alerts: &'a [Alert]) -> Vec<&'a Alert> {
    alerts.iter().filter(|a| a.severity == "critical").collect()
}

// -----------------------------------------------------------------------------
// Task 3: Borrow mutably to modify in place
// -----------------------------------------------------------------------------
fn escalate_warnings(alerts: &mut Vec<Alert>, service: &str) {
    for alert in alerts.iter_mut() {
        if alert.severity == "warning" && alert.service == service {
            alert.severity = "critical".to_string();
        }
    }
}

// -----------------------------------------------------------------------------
// Task 4: Drain a subset to transfer ownership
// -----------------------------------------------------------------------------
// drain() has no predicate form, so take the whole Vec out, partition it,
// and put the keepers back.
fn extract_info_alerts(alerts: &mut Vec<Alert>) -> Vec<Alert> {
    let (info, rest): (Vec<Alert>, Vec<Alert>) = std::mem::take(alerts)
        .into_iter()
        .partition(|a| a.severity == "info");
    *alerts = rest;
    info
}

// -----------------------------------------------------------------------------
// Task 5: Build a summary without cloning
// -----------------------------------------------------------------------------
fn build_summary(alerts: &[Alert]) -> String {
    alerts
        .iter()
        .map(|a| format!("[{}] {}: {}", a.severity, a.service, a.message))
        .collect::<Vec<String>>()
        .join("\n")
}

// -----------------------------------------------------------------------------
// Task 6: Split a vector into two owned halves
// -----------------------------------------------------------------------------
// split_off panics when index > len, so handle past-the-end explicitly.
fn split_alerts(mut alerts: Vec<Alert>, index: usize) -> (Vec<Alert>, Vec<Alert>) {
    if index >= alerts.len() {
        return (alerts, Vec::new());
    }
    let second = alerts.split_off(index);
    (alerts, second)
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_alert(service: &str, severity: &str, message: &str) -> Alert {
        Alert {
            service: service.to_string(),
            severity: severity.to_string(),
            message: message.to_string(),
        }
    }

    fn sample_alerts() -> Vec<Alert> {
        vec![
            make_alert("auth", "critical", "Login service unreachable"),
            make_alert("payments", "warning", "Latency above 2s"),
            make_alert("logging", "info", "Log rotation completed"),
            make_alert("auth", "warning", "High error rate on /login"),
            make_alert("db", "critical", "Replication lag > 30s"),
            make_alert("monitoring", "info", "Heartbeat OK"),
        ]
    }

    // --- Task 1: parse_alerts ---

    #[test]
    fn test_parse_alerts() {
        let raw = vec![
            "auth:critical:Login service unreachable".to_string(),
            "payments:warning:Latency above 2s".to_string(),
            "bad message with no colons".to_string(),
            "db:critical:Replication lag > 30s".to_string(),
        ];
        let alerts = parse_alerts(raw);
        assert_eq!(alerts.len(), 3);
        assert_eq!(alerts[0].service, "auth");
        assert_eq!(alerts[0].severity, "critical");
        assert_eq!(alerts[0].message, "Login service unreachable");
        assert_eq!(alerts[2].service, "db");
    }

    #[test]
    fn test_parse_alerts_with_colons_in_message() {
        let raw = vec![
            "db:critical:Connection to host:5432 refused".to_string(),
        ];
        let alerts = parse_alerts(raw);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].message, "Connection to host:5432 refused");
    }

    #[test]
    fn test_parse_alerts_empty() {
        let alerts = parse_alerts(vec![]);
        assert!(alerts.is_empty());
    }

    // --- Task 2: critical_alerts ---

    #[test]
    fn test_critical_alerts() {
        let alerts = sample_alerts();
        let critical = critical_alerts(&alerts);
        assert_eq!(critical.len(), 2);
        assert_eq!(critical[0].service, "auth");
        assert_eq!(critical[1].service, "db");
        // Original vector is unchanged (we only borrowed it)
        assert_eq!(alerts.len(), 6);
    }

    #[test]
    fn test_critical_alerts_none() {
        let alerts = vec![
            make_alert("web", "info", "All good"),
        ];
        let critical = critical_alerts(&alerts);
        assert!(critical.is_empty());
        assert_eq!(alerts.len(), 1);
    }

    // --- Task 3: escalate_warnings ---

    #[test]
    fn test_escalate_warnings() {
        let mut alerts = sample_alerts();
        escalate_warnings(&mut alerts, "auth");

        // The auth warning should now be critical
        let auth_alerts: Vec<&Alert> = alerts.iter().filter(|a| a.service == "auth").collect();
        assert!(auth_alerts.iter().all(|a| a.severity == "critical"));

        // The payments warning should be unchanged
        let payments = alerts.iter().find(|a| a.service == "payments").unwrap();
        assert_eq!(payments.severity, "warning");
    }

    #[test]
    fn test_escalate_warnings_no_match() {
        let mut alerts = sample_alerts();
        let original = alerts.clone();
        escalate_warnings(&mut alerts, "nonexistent");
        assert_eq!(alerts, original); // nothing changed
    }

    // --- Task 4: extract_info_alerts ---

    #[test]
    fn test_extract_info_alerts() {
        let mut alerts = sample_alerts();
        let info = extract_info_alerts(&mut alerts);

        assert_eq!(info.len(), 2);
        assert!(info.iter().all(|a| a.severity == "info"));
        assert_eq!(alerts.len(), 4);
        assert!(!alerts.iter().any(|a| a.severity == "info"));
    }

    #[test]
    fn test_extract_info_alerts_none() {
        let mut alerts = vec![
            make_alert("web", "critical", "Down"),
        ];
        let info = extract_info_alerts(&mut alerts);
        assert!(info.is_empty());
        assert_eq!(alerts.len(), 1);
    }

    // --- Task 5: build_summary ---

    #[test]
    fn test_build_summary() {
        let alerts = vec![
            make_alert("auth", "critical", "Login service unreachable"),
            make_alert("payments", "warning", "Latency above 2s"),
        ];
        let summary = build_summary(&alerts);
        let expected = "[critical] auth: Login service unreachable\n[warning] payments: Latency above 2s";
        assert_eq!(summary, expected);
        // Verify the alerts vec is still usable (not consumed)
        assert_eq!(alerts.len(), 2);
    }

    #[test]
    fn test_build_summary_empty() {
        let alerts: Vec<Alert> = vec![];
        let summary = build_summary(&alerts);
        assert_eq!(summary, "");
    }

    #[test]
    fn test_build_summary_single() {
        let alerts = vec![make_alert("db", "critical", "Disk full")];
        let summary = build_summary(&alerts);
        assert_eq!(summary, "[critical] db: Disk full");
    }

    // --- Task 6: split_alerts ---

    #[test]
    fn test_split_alerts() {
        let alerts = sample_alerts();
        let (first, second) = split_alerts(alerts, 2);
        assert_eq!(first.len(), 2);
        assert_eq!(second.len(), 4);
        assert_eq!(first[0].service, "auth");
        assert_eq!(second[0].service, "logging");
    }

    #[test]
    fn test_split_alerts_at_zero() {
        let alerts = sample_alerts();
        let (first, second) = split_alerts(alerts, 0);
        assert!(first.is_empty());
        assert_eq!(second.len(), 6);
    }

    #[test]
    fn test_split_alerts_past_end() {
        let alerts = sample_alerts();
        let (first, second) = split_alerts(alerts, 100);
        assert_eq!(first.len(), 6);
        assert!(second.is_empty());
    }
}
