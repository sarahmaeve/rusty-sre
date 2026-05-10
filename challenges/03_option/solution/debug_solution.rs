// =============================================================================
// Challenge 03: Option<T> — Debug Solution
// =============================================================================
//
// Bugs fixed in this file:
//
//   1. lookup_owner: return type changed from `&str` to `Option<&str>` so it
//      matches the `Option<&str>` produced by `.get(...).map(String::as_str)`.
//   2. first_severity: return type changed from `u8` to `Option<u8>` so the
//      `?` operator on a None branch is legal.
//   3. owner_or_unknown: replaced .unwrap() with .cloned().unwrap_or_else(...)
//      so missing services render as "unknown" instead of panicking.
//   4. worst_severity: changed `unwrap_or(0)` to `unwrap_or(5)` — for SRE,
//      a missing severity should be treated as the worst case so it surfaces
//      on the dashboard rather than disappearing.
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

// FIX 1: return Option<&str>
fn lookup_owner<'a>(
    owners: &'a HashMap<String, String>,
    service: &str,
) -> Option<&'a str> {
    owners.get(service).map(String::as_str)
}

// FIX 2: return Option<u8>
fn first_severity(alerts: &[Alert]) -> Option<u8> {
    let alert = alerts.first()?;
    alert.severity
}

// FIX 3: graceful fallback instead of unwrap()
fn owner_or_unknown(owners: &HashMap<String, String>, service: &str) -> String {
    owners
        .get(service)
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
}

// FIX 4: SRE-conservative default — missing severity is treated as worst case
fn worst_severity(alerts: &[Alert]) -> u8 {
    alerts
        .iter()
        .map(|a| a.severity.unwrap_or(5))
        .max()
        .unwrap_or(0)
}

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
        Alert { service: "search".into(), severity: None, message: "missing".into() },
        Alert { service: "search".into(), severity: Some(3), message: "warn".into() },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_owner_returns_option() {
        let owners = sample_owners();
        assert_eq!(lookup_owner(&owners, "auth"), Some("alice"));
        assert_eq!(lookup_owner(&owners, "missing"), None);
    }

    #[test]
    fn first_severity_returns_option() {
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

        assert_eq!(by_service["search"].owner, "unknown");
        assert_eq!(by_service["search"].worst_severity, 5);
    }
}
