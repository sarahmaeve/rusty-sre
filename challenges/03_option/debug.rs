// =============================================================================
// Challenge 03: Option<T> — Debug the On-Call Dashboard
// =============================================================================
//
// This dashboard summarizes the state of a fleet of services. It looks up
// owners, parses severities, and reports the worst alert per service.
//
// It contains FOUR bugs that all involve Option misuse — some stop it
// compiling, some misbehave at runtime. Find and fix all four so every test
// passes.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
// =============================================================================

use std::collections::HashMap;

fn main() {
    let owners = sample_owners();
    let alerts = sample_alerts();
    let report = build_report(&owners, &alerts);
    println!("{report:#?}");
}

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    service: String,
    severity: Option<u8>,
    message: String,
}

#[derive(Debug, PartialEq)]
struct ServiceReport {
    service: String,
    owner: String,
    worst_severity: u8,
}

// Look up the owner of a service in the table.
fn lookup_owner<'a>(owners: &'a HashMap<String, String>, service: &str) -> &'a str {
    owners.get(service).map(String::as_str)
}

// Severity of the first alert in the list.
fn first_severity(alerts: &[Alert]) -> u8 {
    let alert = alerts.first()?;
    let sev = alert.severity?;
    sev
}

// The dashboard should display "unknown" for any service whose owner is not
// in the table.
fn owner_or_unknown(owners: &HashMap<String, String>, service: &str) -> String {
    owners.get(service).unwrap().clone()
}

// Worst (highest) severity across the alerts. Operationally, a missing
// severity is suspicious — treat it as the worst case (5) so the dashboard
// surfaces it.
fn worst_severity(alerts: &[Alert]) -> u8 {
    alerts
        .iter()
        .map(|a| a.severity.unwrap_or(0))
        .max()
        .unwrap_or(0)
}

// -----------------------------------------------------------------------------
// Build a per-service report from the helpers above.
// -----------------------------------------------------------------------------

fn build_report(
    owners: &HashMap<String, String>,
    alerts: &[Alert],
) -> Vec<ServiceReport> {
    use std::collections::BTreeSet;
    let services: BTreeSet<&str> = alerts.iter().map(|a| a.service.as_str()).collect();

    services
        .into_iter()
        .map(|service| {
            let svc_alerts: Vec<Alert> = alerts
                .iter()
                .filter(|a| a.service == service)
                .cloned()
                .collect();
            ServiceReport {
                service: service.to_string(),
                owner: owner_or_unknown(owners, service),
                worst_severity: worst_severity(&svc_alerts),
            }
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Sample data
// -----------------------------------------------------------------------------

fn sample_owners() -> HashMap<String, String> {
    HashMap::from([
        ("auth".to_string(), "alice".to_string()),
        ("payments".to_string(), "bob".to_string()),
    ])
}

fn sample_alerts() -> Vec<Alert> {
    vec![
        Alert { service: "auth".into(), severity: Some(2), message: "warn".into() },
        Alert { service: "auth".into(), severity: Some(4), message: "p99".into() },
        Alert { service: "payments".into(), severity: Some(5), message: "fail".into() },
        // search has alerts but no owner in `sample_owners`.
        Alert { service: "search".into(), severity: None, message: "missing".into() },
        Alert { service: "search".into(), severity: Some(3), message: "warn".into() },
    ]
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_owner_returns_option() {
        // After the fix, lookup_owner should return Option<&str>.
        let owners = sample_owners();
        assert_eq!(lookup_owner(&owners, "auth"), Some("alice"));
        assert_eq!(lookup_owner(&owners, "missing"), None);
    }

    #[test]
    fn first_severity_returns_option() {
        // After the fix, first_severity should return Option<u8>.
        let alerts = sample_alerts();
        assert_eq!(first_severity(&alerts), Some(2));
        assert_eq!(first_severity(&[]), None);
    }

    #[test]
    fn owner_or_unknown_handles_missing() {
        let owners = sample_owners();
        assert_eq!(owner_or_unknown(&owners, "auth"), "alice");
        assert_eq!(owner_or_unknown(&owners, "search"), "unknown");
    }

    #[test]
    fn worst_severity_treats_missing_as_5() {
        // search has one None and one Some(3); the worst case for search is
        // 5 (missing severity counts as worst), not 3.
        let search_alerts = vec![
            Alert { service: "search".into(), severity: None, message: "x".into() },
            Alert { service: "search".into(), severity: Some(3), message: "y".into() },
        ];
        assert_eq!(worst_severity(&search_alerts), 5);
    }

    #[test]
    fn worst_severity_normal() {
        let alerts = vec![
            Alert { service: "x".into(), severity: Some(2), message: "a".into() },
            Alert { service: "x".into(), severity: Some(4), message: "b".into() },
            Alert { service: "x".into(), severity: Some(1), message: "c".into() },
        ];
        assert_eq!(worst_severity(&alerts), 4);
    }

    // Integration test
    #[test]
    fn build_report_handles_missing_owner_and_missing_severity() {
        let owners = sample_owners();
        let alerts = sample_alerts();
        let report = build_report(&owners, &alerts);

        let by_service: HashMap<String, ServiceReport> =
            report.into_iter().map(|r| (r.service.clone(), r)).collect();

        assert_eq!(by_service["auth"].owner, "alice");
        assert_eq!(by_service["auth"].worst_severity, 4);

        assert_eq!(by_service["payments"].owner, "bob");
        assert_eq!(by_service["payments"].worst_severity, 5);

        // search has no owner AND has a None severity:
        assert_eq!(by_service["search"].owner, "unknown");
        assert_eq!(by_service["search"].worst_severity, 5);
    }
}
