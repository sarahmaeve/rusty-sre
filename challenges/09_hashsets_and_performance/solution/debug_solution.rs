// ============================================================================
// Challenge 02a — Solution: Fleet Health Analyzer (all 4 bugs fixed)
// ============================================================================
//
// Run:  rustc debug_solution.rs --edition 2024 --test && ./debug_solution

use std::collections::{HashMap, HashSet};

/// FIX #3: Added Hash, Eq, PartialEq derives so HostInfo can be used in HashSet.
/// Before: #[derive(Debug, Clone)] — missing Hash, Eq, PartialEq.
/// After:  #[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct HostInfo {
    hostname: String,
    datacenter: String,
    status: String,
}

/// FIX #1: Compute overlap BEFORE calling merge_host_lists which moves the sets.
/// Before: overlap computed after merge_host_lists consumed monitored and healthy.
/// After:  overlap computed first, then merge_host_lists can consume safely.
fn find_hosts_needing_attention(
    monitored: HashSet<String>,
    healthy: HashSet<String>,
) -> (Vec<String>, Vec<String>) {
    let mut unhealthy: Vec<String> = monitored
        .difference(&healthy)
        .cloned()
        .collect();
    unhealthy.sort();

    // Compute overlap BEFORE consuming the sets.
    let mut overlap: Vec<String> = monitored
        .intersection(&healthy)
        .cloned()
        .collect();
    overlap.sort();

    // Now it's safe to consume the sets.
    let _all_known = merge_host_lists(monitored, healthy);

    (unhealthy, overlap)
}

fn merge_host_lists(a: HashSet<String>, b: HashSet<String>) -> HashSet<String> {
    &a | &b
}

/// FIX #2: Changed symmetric_difference to difference.
/// Before: symmetric_difference returns items in EITHER set but not both.
/// After:  difference returns items in first set but NOT in second.
fn find_missing_from_monitoring(
    inventory: &HashSet<String>,
    monitoring: &HashSet<String>,
) -> Vec<String> {
    let mut missing: Vec<String> = inventory
        .difference(monitoring)
        .cloned()
        .collect();
    missing.sort();
    missing
}

/// FIX #3 was applied at the struct level (adding derives to HostInfo).
/// This function now compiles because HostInfo implements Hash + Eq.
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

/// FIX #4: Negated the retain condition.
/// Before: active_hosts.retain(|host| decommissioned.contains(host))
///         — keeps decommissioned, removes active (backwards!)
/// After:  active_hosts.retain(|host| !decommissioned.contains(host))
///         — keeps active, removes decommissioned (correct)
fn remove_decommissioned(
    active_hosts: &mut HashSet<String>,
    decommissioned: &HashSet<String>,
) {
    active_hosts.retain(|host| !decommissioned.contains(host));
}

// ============================================================================
// TESTS (identical to debug.rs)
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

    assert_eq!(unhealthy, vec!["db-01", "web-02"]);
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

    assert_eq!(by_dc.len(), 3);

    let us_east = by_dc.get("us-east-1").unwrap();
    assert_eq!(us_east.len(), 3);

    let us_west = by_dc.get("us-west-2").unwrap();
    assert_eq!(us_west.len(), 3);

    let eu_west = by_dc.get("eu-west-1").unwrap();
    assert_eq!(eu_west.len(), 2);
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

    assert_eq!(active.len(), 2);
}

fn main() {
    println!("All bugs fixed! Run tests to verify.");
}
