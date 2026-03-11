// =============================================================================
// Challenge 01a: Ownership & Borrowing with Vectors — SRE Alert Pipeline
// =============================================================================
//
// You are building an alert pipeline for an SRE team. Alerts flow through
// stages: ingestion, deduplication, enrichment, and dispatch. Each stage
// exercises a different aspect of ownership and borrowing with vectors.
//
// Complete each function by replacing the TODO comments with working code.
// Run the tests to verify your solutions:
//     rustc skeleton.rs --edition 2021 --test && ./skeleton
// =============================================================================

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
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
// This function receives ownership of a Vec of raw alert messages (Strings)
// and must return a Vec<Alert>, parsing each message.
//
// Format of each raw message: "service:severity:message text"
// Example: "auth:critical:Login service unreachable"
//
// If a message doesn't have exactly 3 parts when split on ':', skip it.
// The function CONSUMES the input — use into_iter() to avoid cloning.
fn parse_alerts(raw_messages: Vec<String>) -> Vec<Alert> {
    // TODO: Use into_iter() to consume raw_messages and parse each one.
    // Split each string on ':' using splitn(3, ':') to get service, severity, message.
    // Filter out any that don't have all 3 parts.
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: Borrow immutably to analyze
// -----------------------------------------------------------------------------
// Given a borrowed slice of Alerts, return a Vec of references to only the
// critical alerts. The original vector must NOT be consumed or modified.
//
// Note the lifetime annotation: the returned references live as long as
// the input slice.
fn critical_alerts<'a>(alerts: &'a [Alert]) -> Vec<&'a Alert> {
    // TODO: Use iter() and filter() to collect references to alerts
    // where severity == "critical". Do NOT clone.
    todo!()
}

// -----------------------------------------------------------------------------
// Task 3: Borrow mutably to modify in place
// -----------------------------------------------------------------------------
// Escalate all "warning" alerts to "critical" if their service matches
// the given service name. Modify the alerts in place using iter_mut().
fn escalate_warnings(alerts: &mut Vec<Alert>, service: &str) {
    // TODO: Use iter_mut() to find alerts where severity is "warning"
    // AND service matches the given service name. Change their severity
    // to "critical".
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: Drain a subset to transfer ownership
// -----------------------------------------------------------------------------
// Remove all "info" severity alerts from the vector and return them as
// a separate owned Vec<Alert>. The original vector should only contain
// non-info alerts afterward.
//
// Hint: You can't easily use drain() with a predicate. Consider an
// alternative approach: partition the elements, or use a two-vec strategy
// where you take all elements out and put back only the ones you want to keep.
fn extract_info_alerts(alerts: &mut Vec<Alert>) -> Vec<Alert> {
    // TODO: Remove all info alerts from the input vec and return them.
    // After this function, `alerts` should contain no info-level alerts.
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: Build a summary without cloning
// -----------------------------------------------------------------------------
// Given a borrowed slice of Alerts, build a summary string WITHOUT cloning
// any Alert or String. Use only references.
//
// Format:
//   "[{severity}] {service}: {message}" for each alert, joined by newlines.
//
// Example output for one alert:
//   "[critical] auth: Login service down"
fn build_summary(alerts: &[Alert]) -> String {
    // TODO: Iterate over alerts using iter(), format each one, and
    // join them with newlines. Use & references only — no .clone().
    todo!()
}

// -----------------------------------------------------------------------------
// Task 6: Split a vector into two owned halves
// -----------------------------------------------------------------------------
// Split the alerts at the given index, returning two owned Vecs.
// The first Vec gets elements [0..index), the second gets [index..].
// The original Vec is consumed (taken by value).
//
// Hint: Look at split_off() — it works on &mut Vec.
fn split_alerts(mut alerts: Vec<Alert>, index: usize) -> (Vec<Alert>, Vec<Alert>) {
    // TODO: Split the alerts vec at the given index and return both halves.
    // Handle the case where index >= alerts.len() (return all in first, empty second).
    todo!()
}

// =============================================================================
// TESTS — Do not modify below this line
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
