// ============================================================================
// Challenge 10 — Skeleton Solution: Infrastructure Diff Engine (all 6 done)
// ============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution

use std::collections::{HashMap, HashSet};

// ============================================================================
// Test data helpers — these simulate data from different infrastructure sources.
// ============================================================================

fn inventory_hosts() -> HashSet<String> {
    HashSet::from([
        "web-01".into(), "web-02".into(), "web-03".into(),
        "api-01".into(), "api-02".into(),
        "db-01".into(), "db-02".into(),
        "cache-01".into(),
        "queue-01".into(), "queue-02".into(),
        "monitor-01".into(),
        "bastion-01".into(),
        "deploy-01".into(),
    ])
}

fn monitoring_hosts() -> HashSet<String> {
    HashSet::from([
        "web-01".into(), "web-02".into(), "web-03".into(),
        "api-01".into(), "api-02".into(),
        "db-01".into(), "db-02".into(),
        "cache-01".into(),
        "queue-01".into(),
        "monitor-01".into(),
        "legacy-web-01".into(),
        "canary-01".into(),
    ])
}

fn deployment_hosts() -> HashSet<String> {
    HashSet::from([
        "web-01".into(), "web-02".into(), "web-03".into(),
        "api-01".into(), "api-02".into(),
        "db-01".into(),
        "cache-01".into(),
        "queue-01".into(), "queue-02".into(),
        "monitor-01".into(),
        "bastion-01".into(),
    ])
}

// ============================================================================
// TASK 1: Find unmonitored hosts
// ============================================================================

fn find_unmonitored_hosts(
    inventory: &HashSet<String>,
    monitoring: &HashSet<String>,
) -> Vec<String> {
    let mut unmonitored: Vec<String> = inventory.difference(monitoring).cloned().collect();
    unmonitored.sort();
    unmonitored
}

#[test]
fn test_find_unmonitored_hosts() {
    let inventory = inventory_hosts();
    let monitoring = monitoring_hosts();
    let unmonitored = find_unmonitored_hosts(&inventory, &monitoring);

    assert_eq!(unmonitored, vec![
        "bastion-01".to_string(),
        "deploy-01".to_string(),
        "queue-02".to_string(),
    ]);
}

#[test]
fn test_find_unmonitored_hosts_all_covered() {
    let inventory = HashSet::from(["a".to_string(), "b".to_string()]);
    let monitoring = HashSet::from([
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
    ]);
    let unmonitored = find_unmonitored_hosts(&inventory, &monitoring);
    assert!(unmonitored.is_empty());
}

// ============================================================================
// TASK 2: Find hosts present in ALL sources
// ============================================================================

fn find_hosts_in_all_sources(
    inventory: &HashSet<String>,
    monitoring: &HashSet<String>,
    deployment: &HashSet<String>,
) -> Vec<String> {
    let mut consistent: Vec<String> = inventory
        .iter()
        .filter(|host| monitoring.contains(*host) && deployment.contains(*host))
        .cloned()
        .collect();
    consistent.sort();
    consistent
}

#[test]
fn test_find_hosts_in_all_sources() {
    let inventory = inventory_hosts();
    let monitoring = monitoring_hosts();
    let deployment = deployment_hosts();

    let consistent = find_hosts_in_all_sources(&inventory, &monitoring, &deployment);

    assert_eq!(consistent, vec![
        "api-01", "api-02", "cache-01", "db-01",
        "monitor-01", "queue-01", "web-01", "web-02", "web-03",
    ]);
}

#[test]
fn test_find_hosts_in_all_sources_no_overlap() {
    let a = HashSet::from(["x".to_string()]);
    let b = HashSet::from(["y".to_string()]);
    let c = HashSet::from(["z".to_string()]);
    assert!(find_hosts_in_all_sources(&a, &b, &c).is_empty());
}

// ============================================================================
// TASK 3: Detect configuration drift
// ============================================================================

#[derive(Debug, PartialEq)]
struct ConfigDrift {
    added: Vec<String>,
    removed: Vec<String>,
    changed: Vec<String>,
}

fn detect_config_drift(
    old_config: &HashMap<String, String>,
    new_config: &HashMap<String, String>,
) -> ConfigDrift {
    let old_keys: HashSet<String> = old_config.keys().cloned().collect();
    let new_keys: HashSet<String> = new_config.keys().cloned().collect();

    let mut added: Vec<String> = new_keys.difference(&old_keys).cloned().collect();
    let mut removed: Vec<String> = old_keys.difference(&new_keys).cloned().collect();
    let mut changed: Vec<String> = old_keys
        .intersection(&new_keys)
        .filter(|key| old_config.get(*key) != new_config.get(*key))
        .cloned()
        .collect();

    added.sort();
    removed.sort();
    changed.sort();

    ConfigDrift { added, removed, changed }
}

#[test]
fn test_detect_config_drift_basic() {
    let old = HashMap::from([
        ("timeout".into(), "30s".into()),
        ("retries".into(), "3".into()),
        ("log_level".into(), "info".into()),
        ("deprecated_flag".into(), "true".into()),
    ]);
    let new = HashMap::from([
        ("timeout".into(), "60s".into()),   // changed
        ("retries".into(), "3".into()),      // unchanged
        ("log_level".into(), "debug".into()), // changed
        ("new_feature".into(), "on".into()), // added
    ]);

    let drift = detect_config_drift(&old, &new);

    assert_eq!(drift.added, vec!["new_feature"]);
    assert_eq!(drift.removed, vec!["deprecated_flag"]);
    assert_eq!(drift.changed, vec!["log_level", "timeout"]);
}

#[test]
fn test_detect_config_drift_identical() {
    let config = HashMap::from([("a".into(), "1".into()), ("b".into(), "2".into())]);
    let drift = detect_config_drift(&config, &config);

    assert!(drift.added.is_empty());
    assert!(drift.removed.is_empty());
    assert!(drift.changed.is_empty());
}

#[test]
fn test_detect_config_drift_complete_replacement() {
    let old = HashMap::from([("a".into(), "1".into()), ("b".into(), "2".into())]);
    let new = HashMap::from([("c".into(), "3".into()), ("d".into(), "4".into())]);
    let drift = detect_config_drift(&old, &new);

    assert_eq!(drift.added, vec!["c", "d"]);
    assert_eq!(drift.removed, vec!["a", "b"]);
    assert!(drift.changed.is_empty());
}

// ============================================================================
// TASK 4: Deduplicate an alert stream preserving order
// ============================================================================

fn dedup_alert_stream(alerts: &[String]) -> Vec<String> {
    let mut seen: HashSet<&String> = HashSet::new();
    let mut unique = Vec::new();
    for alert in alerts {
        if seen.insert(alert) {
            unique.push(alert.clone());
        }
    }
    unique
}

#[test]
fn test_dedup_alert_stream() {
    let alerts: Vec<String> = vec![
        "disk full on node-3".into(),
        "high CPU on node-1".into(),
        "disk full on node-3".into(),
        "OOM on node-2".into(),
        "high CPU on node-1".into(),
        "disk full on node-3".into(),
    ];

    let deduped = dedup_alert_stream(&alerts);
    assert_eq!(deduped, vec![
        "disk full on node-3".to_string(),
        "high CPU on node-1".to_string(),
        "OOM on node-2".to_string(),
    ]);
}

#[test]
fn test_dedup_alert_stream_all_unique() {
    let alerts: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let deduped = dedup_alert_stream(&alerts);
    assert_eq!(deduped, vec!["a", "b", "c"]);
}

#[test]
fn test_dedup_alert_stream_empty() {
    let alerts: Vec<String> = vec![];
    assert!(dedup_alert_stream(&alerts).is_empty());
}

// ============================================================================
// TASK 5: Partition IPs by allowlist
// ============================================================================

fn partition_by_allowlist(
    allowlist: &HashSet<String>,
    ip_stream: &[String],
) -> (Vec<String>, Vec<String>) {
    let mut allowed = Vec::new();
    let mut denied = Vec::new();
    for ip in ip_stream {
        if allowlist.contains(ip) {
            allowed.push(ip.clone());
        } else {
            denied.push(ip.clone());
        }
    }
    (allowed, denied)
}

#[test]
fn test_partition_by_allowlist() {
    let allowlist = HashSet::from([
        "10.0.0.1".to_string(),
        "10.0.0.2".to_string(),
        "10.0.0.5".to_string(),
    ]);
    let stream: Vec<String> = vec![
        "10.0.0.1".into(),
        "192.168.1.1".into(),
        "10.0.0.2".into(),
        "172.16.0.1".into(),
        "10.0.0.5".into(),
        "10.0.0.1".into(),
    ];

    let (allowed, denied) = partition_by_allowlist(&allowlist, &stream);
    assert_eq!(allowed, vec!["10.0.0.1", "10.0.0.2", "10.0.0.5", "10.0.0.1"]);
    assert_eq!(denied, vec!["192.168.1.1", "172.16.0.1"]);
}

#[test]
fn test_partition_by_allowlist_empty() {
    let allowlist: HashSet<String> = HashSet::new();
    let stream: Vec<String> = vec!["10.0.0.1".into(), "10.0.0.2".into()];

    let (allowed, denied) = partition_by_allowlist(&allowlist, &stream);
    assert!(allowed.is_empty());
    assert_eq!(denied.len(), 2);
}

// ============================================================================
// TASK 6: Pre-sized metric aggregation
// ============================================================================

fn aggregate_metrics(
    metrics: &[(String, f64)],
    estimated_keys: usize,
) -> HashMap<String, f64> {
    let mut agg: HashMap<String, f64> = HashMap::with_capacity(estimated_keys);
    for (name, value) in metrics {
        *agg.entry(name.clone()).or_insert(0.0) += *value;
    }
    agg
}

#[test]
fn test_aggregate_metrics() {
    let metrics: Vec<(String, f64)> = vec![
        ("cpu_usage".into(), 85.0),
        ("mem_usage".into(), 72.5),
        ("cpu_usage".into(), 90.0),
        ("disk_io".into(), 45.0),
        ("cpu_usage".into(), 78.0),
        ("mem_usage".into(), 68.0),
    ];

    let agg = aggregate_metrics(&metrics, 3);

    let cpu = agg.get("cpu_usage").unwrap();
    assert!((cpu - 253.0).abs() < 0.01, "Expected 253.0, got {}", cpu);

    let mem = agg.get("mem_usage").unwrap();
    assert!((mem - 140.5).abs() < 0.01, "Expected 140.5, got {}", mem);

    assert_eq!(agg.get("disk_io"), Some(&45.0));
}

#[test]
fn test_aggregate_metrics_capacity() {
    let metrics: Vec<(String, f64)> = vec![
        ("a".into(), 1.0),
        ("b".into(), 2.0),
        ("c".into(), 3.0),
    ];

    let agg = aggregate_metrics(&metrics, 100);

    // Verify the map was pre-allocated (capacity >= estimated_keys).
    assert!(
        agg.capacity() >= 100,
        "Expected capacity >= 100, got {}",
        agg.capacity()
    );
    assert_eq!(agg.len(), 3);
}

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}
