// ============================================================================
// Challenge 02a — Debug: Fleet Health Analyzer
// ============================================================================
//
// This program analyzes fleet health data across data centers. It finds
// hosts needing attention, detects missing monitoring, groups hosts by
// datacenter, and removes decommissioned machines.
//
// There are 4 bugs hidden in this code:
//   - 2 compile errors (the code won't build until you fix them)
//   - 2 runtime bugs (the code builds but produces wrong results)
//
// All bugs involve common HashSet and HashMap-with-sets mistakes.
//
// Run:  rustc debug.rs --edition 2021 --test && ./debug

use std::collections::{HashMap, HashSet};

/// Information about a single host in the fleet.
/// NOTE: This struct is missing some trait derives it needs...
#[derive(Debug, Clone)]
struct HostInfo {
    hostname: String,
    datacenter: String,
    status: String, // "healthy", "degraded", "down"
}

/// Find hosts that are monitored but not healthy, and hosts that appear
/// in both the monitored and healthy sets.
///
/// BUG #1: This function does NOT compile.
/// Values are moved into merge_host_lists(), then used again afterward.
///
/// Think about what happens when you pass owned values to a function,
/// then try to use them on the next line...
/// Python parallel: Python passes references, so this just works.
/// In Rust, passing by value MOVES the data.
fn find_hosts_needing_attention(
    monitored: HashSet<String>,
    healthy: HashSet<String>,
) -> (Vec<String>, Vec<String>) {
    // Find hosts that are monitored but not healthy.
    let mut unhealthy: Vec<String> = monitored
        .difference(&healthy)
        .cloned()
        .collect();
    unhealthy.sort();

    // Merge both lists to get "all known hosts" — this consumes both sets!
    let _all_known = merge_host_lists(monitored, healthy);

    // BUG: monitored and healthy were MOVED into merge_host_lists above.
    // This line tries to use them after the move — won't compile.
    let mut overlap: Vec<String> = monitored
        .intersection(&healthy)
        .cloned()
        .collect();
    overlap.sort();

    (unhealthy, overlap)
}

fn merge_host_lists(a: HashSet<String>, b: HashSet<String>) -> HashSet<String> {
    &a | &b
}

/// Find hosts that are in the inventory but missing from monitoring.
///
/// BUG #2: This function compiles and runs, but returns WRONG results.
/// It uses symmetric_difference when it should use difference.
///
/// symmetric_difference = items in EITHER set but not both (Python: a ^ b)
/// difference = items in first set but not second (Python: a - b)
///
/// The code finds hosts that are unmatched in EITHER direction,
/// but we only want hosts missing FROM monitoring.
fn find_missing_from_monitoring(
    inventory: &HashSet<String>,
    monitoring: &HashSet<String>,
) -> Vec<String> {
    // BUG: symmetric_difference gives us items in EITHER set but not both.
    // We want items in inventory but NOT in monitoring — that's difference.
    let mut missing: Vec<String> = inventory
        .symmetric_difference(monitoring)
        .cloned()
        .collect();
    missing.sort();
    missing
}

/// Group hosts by datacenter, storing unique hostnames per DC.
///
/// BUG #3: This function does NOT compile.
/// HostInfo is used as a value in a HashSet, but it doesn't implement
/// the Hash and Eq traits required for HashSet membership.
///
/// Python parallel: Any object can go in a Python set if it defines
/// __hash__ and __eq__. Rust requires explicit trait implementations.
fn unique_hosts_by_datacenter(
    hosts: &[HostInfo],
) -> HashMap<String, HashSet<HostInfo>> {
    let mut by_dc: HashMap<String, HashSet<HostInfo>> = HashMap::new();

    for host in hosts {
        by_dc
            .entry(host.datacenter.clone())
            .or_insert_with(HashSet::new)
            // BUG: HostInfo doesn't derive Hash, Eq, or PartialEq.
            // The compiler will refuse to insert it into a HashSet.
            .insert(host.clone());
    }

    by_dc
}

/// Remove decommissioned hosts from the active set.
///
/// BUG #4: This function compiles and runs, but does the OPPOSITE of
/// what's intended — it KEEPS decommissioned hosts and REMOVES active ones.
///
/// retain() keeps items where the closure returns true.
/// The logic is inverted: it returns true for decommissioned hosts.
///
/// Python parallel: {h for h in active if h not in decommissioned}
/// — note the "not in". Easy to forget the negation in Rust too.
fn remove_decommissioned(
    active_hosts: &mut HashSet<String>,
    decommissioned: &HashSet<String>,
) {
    // BUG: retain() keeps items where closure returns true.
    // This keeps hosts that ARE in the decommissioned set — backwards!
    active_hosts.retain(|host| decommissioned.contains(host));
}

// ============================================================================
// TESTS
// ============================================================================

fn sample_hosts() -> Vec<HostInfo> {
    vec![
        HostInfo { hostname: "web-01".into(), datacenter: "us-east-1".into(), status: "healthy".into() },
        HostInfo { hostname: "web-02".into(), datacenter: "us-east-1".into(), status: "degraded".into() },
        HostInfo { hostname: "api-01".into(), datacenter: "us-east-1".into(), status: "healthy".into() },
        HostInfo { hostname: "api-02".into(), datacenter: "us-west-2".into(), status: "healthy".into() },
        HostInfo { hostname: "db-01".into(), datacenter: "us-west-2".into(), status: "down".into() },
        HostInfo { hostname: "db-02".into(), datacenter: "us-west-2".into(), status: "healthy".into() },
        HostInfo { hostname: "cache-01".into(), datacenter: "eu-west-1".into(), status: "healthy".into() },
        HostInfo { hostname: "cache-02".into(), datacenter: "eu-west-1".into(), status: "degraded".into() },
        // Duplicate entry — same host appearing twice (should be deduped).
        HostInfo { hostname: "web-01".into(), datacenter: "us-east-1".into(), status: "healthy".into() },
    ]
}

#[test]
fn test_find_hosts_needing_attention() {
    let monitored = HashSet::from([
        "web-01".to_string(),
        "web-02".to_string(),
        "api-01".to_string(),
        "db-01".to_string(),
    ]);
    let healthy = HashSet::from([
        "web-01".to_string(),
        "api-01".to_string(),
        "cache-01".to_string(),
    ]);

    let (unhealthy, overlap) = find_hosts_needing_attention(monitored, healthy);

    // Monitored but not healthy: db-01, web-02
    assert_eq!(unhealthy, vec!["db-01", "web-02"]);

    // In both sets: api-01, web-01
    assert_eq!(overlap, vec!["api-01", "web-01"]);
}

#[test]
fn test_find_missing_from_monitoring() {
    let inventory = HashSet::from([
        "web-01".to_string(),
        "api-01".to_string(),
        "db-01".to_string(),
        "bastion-01".to_string(),
    ]);
    let monitoring = HashSet::from([
        "web-01".to_string(),
        "api-01".to_string(),
        "legacy-01".to_string(),
    ]);

    let missing = find_missing_from_monitoring(&inventory, &monitoring);

    // Should be ONLY hosts in inventory but not in monitoring.
    // "legacy-01" is in monitoring but not inventory — should NOT appear.
    assert_eq!(missing, vec!["bastion-01", "db-01"]);
}

#[test]
fn test_find_missing_empty_monitoring() {
    let inventory = HashSet::from(["a".to_string(), "b".to_string()]);
    let monitoring: HashSet<String> = HashSet::new();

    let missing = find_missing_from_monitoring(&inventory, &monitoring);
    assert_eq!(missing, vec!["a", "b"]);
}

#[test]
fn test_unique_hosts_by_datacenter() {
    let hosts = sample_hosts();
    let by_dc = unique_hosts_by_datacenter(&hosts);

    assert_eq!(by_dc.len(), 3); // 3 datacenters

    let us_east = by_dc.get("us-east-1").unwrap();
    // web-01 appears twice in sample_hosts but should be deduped in the set.
    assert_eq!(us_east.len(), 3); // web-01, web-02, api-01

    let us_west = by_dc.get("us-west-2").unwrap();
    assert_eq!(us_west.len(), 3); // api-02, db-01, db-02

    let eu_west = by_dc.get("eu-west-1").unwrap();
    assert_eq!(eu_west.len(), 2); // cache-01, cache-02
}

#[test]
fn test_remove_decommissioned() {
    let mut active = HashSet::from([
        "web-01".to_string(),
        "web-02".to_string(),
        "api-01".to_string(),
        "legacy-01".to_string(),
        "legacy-02".to_string(),
    ]);
    let decommissioned = HashSet::from([
        "legacy-01".to_string(),
        "legacy-02".to_string(),
    ]);

    remove_decommissioned(&mut active, &decommissioned);

    assert_eq!(active.len(), 3);
    assert!(active.contains("web-01"));
    assert!(active.contains("web-02"));
    assert!(active.contains("api-01"));
    assert!(!active.contains("legacy-01"));
    assert!(!active.contains("legacy-02"));
}

#[test]
fn test_remove_decommissioned_empty() {
    let mut active = HashSet::from(["a".to_string(), "b".to_string()]);
    let decommissioned: HashSet<String> = HashSet::new();

    remove_decommissioned(&mut active, &decommissioned);

    // Nothing decommissioned — all hosts should remain.
    assert_eq!(active.len(), 2);
}

fn main() {
    println!("Run with: rustc debug.rs --edition 2021 --test && ./debug");
    println!("Fix the 4 bugs to make all tests pass!");
}
