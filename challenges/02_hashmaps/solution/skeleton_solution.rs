// ============================================================================
// Challenge 02 — Skeleton Solution: Metrics Aggregator (all 6 tasks done)
// ============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution

use std::collections::HashMap;

/// A single access log entry from a reverse proxy.
#[derive(Debug, Clone)]
struct AccessEntry {
    method: String,   // "GET", "POST", "PUT", "DELETE"
    endpoint: String, // "/api/v1/users", "/healthz", etc.
    status: u16,      // 200, 404, 500, etc.
    latency_ms: u64,  // response time in milliseconds
}

// Helper to build test data so tests are self-contained.
fn sample_entries() -> Vec<AccessEntry> {
    vec![
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/users".into(), status: 200, latency_ms: 45 },
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/users".into(), status: 200, latency_ms: 52 },
        AccessEntry { method: "POST".into(), endpoint: "/api/v1/users".into(), status: 201, latency_ms: 120 },
        AccessEntry { method: "GET".into(), endpoint: "/healthz".into(), status: 200, latency_ms: 2 },
        AccessEntry { method: "GET".into(), endpoint: "/healthz".into(), status: 200, latency_ms: 1 },
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/orders".into(), status: 200, latency_ms: 88 },
        AccessEntry { method: "POST".into(), endpoint: "/api/v1/orders".into(), status: 500, latency_ms: 340 },
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/orders".into(), status: 404, latency_ms: 15 },
        AccessEntry { method: "DELETE".into(), endpoint: "/api/v1/users".into(), status: 403, latency_ms: 30 },
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/users".into(), status: 500, latency_ms: 210 },
        AccessEntry { method: "PUT".into(), endpoint: "/api/v1/orders".into(), status: 200, latency_ms: 75 },
        AccessEntry { method: "GET".into(), endpoint: "/api/v1/orders".into(), status: 500, latency_ms: 290 },
    ]
}

// ============================================================================
// TASK 1: Count requests by status code
// ============================================================================

fn count_by_status(entries: &[AccessEntry]) -> HashMap<u16, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.status).or_insert(0) += 1;
    }
    counts
}

#[test]
fn test_count_by_status() {
    let entries = sample_entries();
    let counts = count_by_status(&entries);

    assert_eq!(counts.get(&200), Some(&6));
    assert_eq!(counts.get(&201), Some(&1));
    assert_eq!(counts.get(&404), Some(&1));
    assert_eq!(counts.get(&403), Some(&1));
    assert_eq!(counts.get(&500), Some(&3));
    assert_eq!(counts.get(&999), None); // missing key returns None
}

// ============================================================================
// TASK 2: Group endpoints by HTTP method
// ============================================================================

fn group_endpoints_by_method(entries: &[AccessEntry]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for entry in entries {
        groups
            .entry(entry.method.clone())
            .or_default()
            .push(entry.endpoint.clone());
    }
    for endpoints in groups.values_mut() {
        endpoints.sort();
        endpoints.dedup();
    }
    groups
}

#[test]
fn test_group_endpoints_by_method() {
    let entries = sample_entries();
    let groups = group_endpoints_by_method(&entries);

    let get_endpoints = groups.get("GET").unwrap();
    assert_eq!(get_endpoints, &vec![
        "/api/v1/orders".to_string(),
        "/api/v1/users".to_string(),
        "/healthz".to_string(),
    ]);

    let post_endpoints = groups.get("POST").unwrap();
    assert_eq!(post_endpoints, &vec![
        "/api/v1/orders".to_string(),
        "/api/v1/users".to_string(),
    ]);

    let delete_endpoints = groups.get("DELETE").unwrap();
    assert_eq!(delete_endpoints, &vec!["/api/v1/users".to_string()]);
}

// ============================================================================
// TASK 3: Average latency per endpoint
// ============================================================================

fn avg_latency_per_endpoint(entries: &[AccessEntry]) -> HashMap<String, f64> {
    let mut sums: HashMap<String, (u64, u64)> = HashMap::new();
    for entry in entries {
        let acc = sums.entry(entry.endpoint.clone()).or_insert((0, 0));
        acc.0 += entry.latency_ms;
        acc.1 += 1;
    }
    sums.into_iter()
        .map(|(endpoint, (total, count))| (endpoint, total as f64 / count as f64))
        .collect()
}

#[test]
fn test_avg_latency_per_endpoint() {
    let entries = sample_entries();
    let avgs = avg_latency_per_endpoint(&entries);

    // /healthz: (2 + 1) / 2 = 1.5
    let healthz = avgs.get("/healthz").unwrap();
    assert!((healthz - 1.5).abs() < 0.01, "Expected ~1.5, got {}", healthz);

    // /api/v1/users: (45 + 52 + 120 + 30 + 210) / 5 = 91.4
    let users = avgs.get("/api/v1/users").unwrap();
    assert!((users - 91.4).abs() < 0.01, "Expected ~91.4, got {}", users);

    // /api/v1/orders: (88 + 340 + 15 + 75 + 290) / 5 = 161.6
    let orders = avgs.get("/api/v1/orders").unwrap();
    assert!((orders - 161.6).abs() < 0.01, "Expected ~161.6, got {}", orders);
}

// ============================================================================
// TASK 4: Top N endpoints by request count
// ============================================================================

fn top_endpoints(entries: &[AccessEntry], n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for entry in entries {
        *counts.entry(entry.endpoint.clone()).or_insert(0) += 1;
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    // Count descending, ties broken alphabetically ascending.
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.truncate(n);
    ranked
}

#[test]
fn test_top_endpoints() {
    let entries = sample_entries();

    let top2 = top_endpoints(&entries, 2);
    assert_eq!(top2.len(), 2);
    // /api/v1/users: 5 requests, /api/v1/orders: 5 requests (tie broken alphabetically)
    assert_eq!(top2[0], ("/api/v1/orders".to_string(), 5));
    assert_eq!(top2[1], ("/api/v1/users".to_string(), 5));

    let top1 = top_endpoints(&entries, 1);
    assert_eq!(top1.len(), 1);
    assert_eq!(top1[0], ("/api/v1/orders".to_string(), 5));
}

// ============================================================================
// TASK 5: Detect error-heavy endpoints
// ============================================================================

fn error_heavy_endpoints(entries: &[AccessEntry], threshold: f64) -> Vec<String> {
    let mut totals: HashMap<String, usize> = HashMap::new();
    let mut errors: HashMap<String, usize> = HashMap::new();
    for entry in entries {
        *totals.entry(entry.endpoint.clone()).or_insert(0) += 1;
        if entry.status >= 400 {
            *errors.entry(entry.endpoint.clone()).or_insert(0) += 1;
        }
    }
    let mut heavy: Vec<String> = totals
        .into_iter()
        .filter(|(endpoint, total)| {
            let error_count = errors.get(endpoint).copied().unwrap_or(0);
            error_count as f64 / *total as f64 > threshold
        })
        .map(|(endpoint, _)| endpoint)
        .collect();
    heavy.sort();
    heavy
}

#[test]
fn test_error_heavy_endpoints() {
    let entries = sample_entries();

    // /api/v1/orders: 3 errors (500, 404, 500) out of 5 = 0.6
    // /api/v1/users: 2 errors (403, 500) out of 5 = 0.4
    // /healthz: 0 errors out of 2 = 0.0

    let high_error = error_heavy_endpoints(&entries, 0.5);
    assert_eq!(high_error, vec!["/api/v1/orders".to_string()]);

    let moderate_error = error_heavy_endpoints(&entries, 0.3);
    assert_eq!(moderate_error, vec![
        "/api/v1/orders".to_string(),
        "/api/v1/users".to_string(),
    ]);

    let no_errors = error_heavy_endpoints(&entries, 0.9);
    assert!(no_errors.is_empty());
}

// ============================================================================
// TASK 6: Merge status counts from two entry sets
// ============================================================================

fn merge_status_counts(
    set1: &[AccessEntry],
    set2: &[AccessEntry],
) -> HashMap<u16, usize> {
    let mut merged: HashMap<u16, usize> = HashMap::new();
    for entry in set1.iter().chain(set2) {
        merged
            .entry(entry.status)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    merged
}

#[test]
fn test_merge_status_counts() {
    let set1 = vec![
        AccessEntry { method: "GET".into(), endpoint: "/a".into(), status: 200, latency_ms: 10 },
        AccessEntry { method: "GET".into(), endpoint: "/b".into(), status: 200, latency_ms: 20 },
        AccessEntry { method: "GET".into(), endpoint: "/c".into(), status: 500, latency_ms: 30 },
    ];
    let set2 = vec![
        AccessEntry { method: "GET".into(), endpoint: "/d".into(), status: 200, latency_ms: 10 },
        AccessEntry { method: "GET".into(), endpoint: "/e".into(), status: 404, latency_ms: 15 },
        AccessEntry { method: "GET".into(), endpoint: "/f".into(), status: 500, latency_ms: 50 },
        AccessEntry { method: "GET".into(), endpoint: "/g".into(), status: 500, latency_ms: 60 },
    ];

    let merged = merge_status_counts(&set1, &set2);

    assert_eq!(merged.get(&200), Some(&3)); // 2 + 1
    assert_eq!(merged.get(&500), Some(&3)); // 1 + 2
    assert_eq!(merged.get(&404), Some(&1)); // 0 + 1
    assert_eq!(merged.get(&201), None);     // not present in either
}

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}
