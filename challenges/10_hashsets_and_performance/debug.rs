// ============================================================================
// Challenge 10 — Debug: Fleet Health Analyzer
// ============================================================================
//
// This program analyzes fleet health data across data centers. It finds
// hosts needing attention, detects missing monitoring, groups hosts by
// datacenter, and removes decommissioned machines.
//
// There are 4 bugs hidden in this code — some stop it compiling, some make
// it misbehave at runtime. All involve common HashSet and HashMap-with-sets
// mistakes. Fix all 4 to make the tests pass.
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
//
// Run:  rustc debug.rs --edition 2024 --test && ./debug

use std::collections::{HashMap, HashSet};

/// Information about a single host in the fleet.
#[derive(Debug, Clone)]
struct HostInfo {
    hostname: String,
    datacenter: String,
    status: String, // "healthy", "degraded", "down"
}

/// Find hosts that are monitored but not healthy, and hosts that appear
/// in both the monitored and healthy sets.
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

    // Merge both lists to get "all known hosts".
    let _all_known = merge_host_lists(monitored, healthy);

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
fn find_missing_from_monitoring(
    inventory: &HashSet<String>,
    monitoring: &HashSet<String>,
) -> Vec<String> {
    let mut missing: Vec<String> = inventory
        .symmetric_difference(monitoring)
        .cloned()
        .collect();
    missing.sort();
    missing
}

/// Group hosts by datacenter, storing unique hosts per DC.
fn unique_hosts_by_datacenter(
    hosts: &[HostInfo],
) -> HashMap<String, HashSet<HostInfo>> {
    let mut by_dc: HashMap<String, HashSet<HostInfo>> = HashMap::new();

    for host in hosts {
        by_dc
            .entry(host.datacenter.clone())
            .or_insert_with(HashSet::new)
            .insert(host.clone());
    }

    by_dc
}

/// Remove decommissioned hosts from the active set.
fn remove_decommissioned(
    active_hosts: &mut HashSet<String>,
    decommissioned: &HashSet<String>,
) {
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
    println!("Run with: rustc debug.rs --edition 2024 --test && ./debug");
    println!("Fix the 4 bugs to make all tests pass!");
}
