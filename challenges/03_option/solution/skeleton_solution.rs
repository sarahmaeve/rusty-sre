// =============================================================================
// Challenge 03: Option<T> — Service Status Lookup (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

use std::collections::HashMap;

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    service: String,
    severity: u8, // 1..=5
    message: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Stats {
    uptime_pct: f64,
    error_rate: f64,
}

// -----------------------------------------------------------------------------
// Task 1: find_owner
// -----------------------------------------------------------------------------
fn find_owner<'a>(owners: &'a HashMap<String, String>, service: &str) -> Option<&'a str> {
    owners.get(service).map(String::as_str)
}

// -----------------------------------------------------------------------------
// Task 2: parse_optional_severity
// -----------------------------------------------------------------------------
fn parse_optional_severity(input: &str) -> Option<u8> {
    input
        .trim()
        .parse::<u8>()
        .ok()
        .filter(|n| (1..=5).contains(n))
}

// -----------------------------------------------------------------------------
// Task 3: first_critical
// -----------------------------------------------------------------------------
fn first_critical(alerts: &[Alert]) -> Option<&Alert> {
    alerts.iter().find(|a| a.severity >= 4)
}

// -----------------------------------------------------------------------------
// Task 4: owner_team
// -----------------------------------------------------------------------------
// `?` propagates the None from the first lookup; and_then would work equally.
fn owner_team(
    owners: &HashMap<String, String>,
    teams: &HashMap<String, String>,
    service: &str,
) -> Option<String> {
    let owner = owners.get(service)?;
    teams.get(owner).cloned()
}

// -----------------------------------------------------------------------------
// Task 5: summary_or_default
// -----------------------------------------------------------------------------
fn summary_or_default(stats: Option<Stats>) -> (f64, f64) {
    stats.map_or((0.0, 1.0), |s| (s.uptime_pct, s.error_rate))
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_owners() -> HashMap<String, String> {
        HashMap::from([
            ("auth".to_string(), "alice".to_string()),
            ("payments".to_string(), "bob".to_string()),
            ("search".to_string(), "carol".to_string()),
        ])
    }

    fn sample_teams() -> HashMap<String, String> {
        HashMap::from([
            ("alice".to_string(), "platform".to_string()),
            ("bob".to_string(), "money".to_string()),
            // carol is intentionally absent — she has no team
        ])
    }

    fn sample_alerts() -> Vec<Alert> {
        vec![
            Alert { service: "auth".into(), severity: 2, message: "low memory warning".into() },
            Alert { service: "search".into(), severity: 4, message: "p99 latency spike".into() },
            Alert { service: "payments".into(), severity: 5, message: "checkout failures".into() },
        ]
    }

    // ----- Task 1 -----

    #[test]
    fn find_owner_present() {
        let owners = sample_owners();
        assert_eq!(find_owner(&owners, "auth"), Some("alice"));
    }

    #[test]
    fn find_owner_absent() {
        let owners = sample_owners();
        assert_eq!(find_owner(&owners, "missing-service"), None);
    }

    // ----- Task 2 -----

    #[test]
    fn parse_optional_severity_valid() {
        assert_eq!(parse_optional_severity("1"), Some(1));
        assert_eq!(parse_optional_severity("3"), Some(3));
        assert_eq!(parse_optional_severity("5"), Some(5));
        assert_eq!(parse_optional_severity(" 4 "), Some(4)); // whitespace ok
    }

    #[test]
    fn parse_optional_severity_empty() {
        assert_eq!(parse_optional_severity(""), None);
        assert_eq!(parse_optional_severity("   "), None);
    }

    #[test]
    fn parse_optional_severity_out_of_range() {
        assert_eq!(parse_optional_severity("0"), None);
        assert_eq!(parse_optional_severity("6"), None);
        assert_eq!(parse_optional_severity("99"), None);
    }

    #[test]
    fn parse_optional_severity_garbage() {
        assert_eq!(parse_optional_severity("high"), None);
        assert_eq!(parse_optional_severity("-1"), None);
    }

    // ----- Task 3 -----

    #[test]
    fn first_critical_found() {
        let alerts = sample_alerts();
        let first = first_critical(&alerts).unwrap();
        assert_eq!(first.service, "search");
    }

    #[test]
    fn first_critical_none() {
        let alerts = vec![
            Alert { service: "x".into(), severity: 1, message: "ok".into() },
            Alert { service: "y".into(), severity: 3, message: "warn".into() },
        ];
        assert_eq!(first_critical(&alerts), None);
    }

    #[test]
    fn first_critical_empty() {
        let alerts: Vec<Alert> = vec![];
        assert_eq!(first_critical(&alerts), None);
    }

    // ----- Task 4 -----

    #[test]
    fn owner_team_full_chain() {
        let owners = sample_owners();
        let teams = sample_teams();
        assert_eq!(owner_team(&owners, &teams, "auth"), Some("platform".to_string()));
        assert_eq!(owner_team(&owners, &teams, "payments"), Some("money".to_string()));
    }

    #[test]
    fn owner_team_owner_missing() {
        let owners = sample_owners();
        let teams = sample_teams();
        assert_eq!(owner_team(&owners, &teams, "no-such-service"), None);
    }

    #[test]
    fn owner_team_team_missing() {
        let owners = sample_owners();
        let teams = sample_teams();
        // search → carol exists, but carol has no team
        assert_eq!(owner_team(&owners, &teams, "search"), None);
    }

    // ----- Task 5 -----

    #[test]
    fn summary_or_default_present() {
        let stats = Stats { uptime_pct: 99.95, error_rate: 0.001 };
        assert_eq!(summary_or_default(Some(stats)), (99.95, 0.001));
    }

    #[test]
    fn summary_or_default_missing() {
        // SRE-conservative: 0% uptime, 100% error rate when stats are unknown.
        assert_eq!(summary_or_default(None), (0.0, 1.0));
    }
}
