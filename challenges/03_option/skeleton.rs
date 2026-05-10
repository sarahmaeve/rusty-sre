// =============================================================================
// Challenge 03: Option<T> — Service Status Lookup
// =============================================================================
//
// You are building helper functions for an on-call dashboard. The dashboard
// queries multiple lookup tables, parses optional fields, and combines them
// into a unified view. Many of the lookups can fail to find a match — that's
// what Option is for.
//
// Complete each function by replacing the TODO comments with working code.
// Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

use std::collections::HashMap;

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
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
// Look up the owner of a service. Return Option<&str> — None if the service
// is not in the map.
//
// HINT: HashMap::get returns Option<&V>. To turn Option<&String> into
// Option<&str>, use .map(String::as_str) or .map(|s| s.as_str()).
fn find_owner<'a>(_owners: &'a HashMap<String, String>, _service: &str) -> Option<&'a str> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 2: parse_optional_severity
// -----------------------------------------------------------------------------
// Parse an optional severity from text. Rules:
//   - empty or whitespace-only        → None
//   - parses as a number in 1..=5     → Some(n)
//   - any other case (out of range,
//     garbage, negative)              → None
//
// HINT: chain .trim(), .parse::<u8>().ok(), and .filter().
fn parse_optional_severity(_input: &str) -> Option<u8> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 3: first_critical
// -----------------------------------------------------------------------------
// Return the first alert in the slice with severity >= 4. None if none qualify.
//
// HINT: Iterator::find returns Option<&T>.
fn first_critical(_alerts: &[Alert]) -> Option<&Alert> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: owner_team
// -----------------------------------------------------------------------------
// Two-step lookup: service → owner, owner → team. Return Some(team) only if
// BOTH lookups succeed. None if either step is missing.
//
// HINT: this is a textbook use case for Option::and_then. You may also use
// `?` because this function returns Option.
fn owner_team(
    _owners: &HashMap<String, String>,
    _teams: &HashMap<String, String>,
    _service: &str,
) -> Option<String> {
    // TODO
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: summary_or_default
// -----------------------------------------------------------------------------
// Given Option<Stats>, return a non-optional summary tuple (uptime_pct, error_rate).
// If None, return SRE-conservative defaults: (0.0, 1.0) — i.e. "we don't know,
// assume the worst" so the dashboard surfaces the missing data.
//
// HINT: Option::map_or, or .map(...).unwrap_or((0.0, 1.0)). Avoid .unwrap();
// the whole point is graceful handling.
fn summary_or_default(_stats: Option<Stats>) -> (f64, f64) {
    // TODO
    todo!()
}

// =============================================================================
// TESTS — Do not modify below this line
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
