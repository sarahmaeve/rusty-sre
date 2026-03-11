// ============================================================================
// Challenge 02 — Skeleton: Metrics Aggregator
// ============================================================================
//
// You're building an SRE monitoring dashboard that aggregates HTTP access log
// entries. Complete the 6 TODO tasks below. Each function has tests that
// describe the expected behavior.
//
// Run tests:  rustc skeleton.rs --edition 2021 --test && ./skeleton
//
// Hint: The Entry API is your best friend here. Refer to concept.rs Section 4.

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
//
// Return a HashMap mapping each status code to the number of times it appears.
//
// Python equivalent:
//   from collections import Counter
//   Counter(entry["status"] for entry in entries)
//
// Hint: Use the Entry API counting pattern — entry().or_insert(0), then += 1.

fn count_by_status(entries: &[AccessEntry]) -> HashMap<u16, usize> {
    // TODO: Implement this function
    todo!()
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
//
// Return a HashMap mapping each HTTP method to a sorted, deduplicated Vec of
// endpoint paths that were called with that method.
//
// Python equivalent:
//   from collections import defaultdict
//   groups = defaultdict(set)
//   for e in entries: groups[e["method"]].add(e["endpoint"])
//   {k: sorted(v) for k, v in groups.items()}
//
// Hint: Group with entry().or_insert_with(Vec::new), then dedup after.

fn group_endpoints_by_method(entries: &[AccessEntry]) -> HashMap<String, Vec<String>> {
    // TODO: Implement this function
    // Remember to sort and deduplicate the endpoint lists!
    todo!()
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
//
// Return a HashMap mapping each endpoint to its average latency in ms (f64).
//
// Python equivalent:
//   from collections import defaultdict
//   sums = defaultdict(lambda: [0, 0])
//   for e in entries:
//       sums[e["endpoint"]][0] += e["latency_ms"]
//       sums[e["endpoint"]][1] += 1
//   {k: s/c for k, (s, c) in sums.items()}
//
// Hint: Accumulate (total_ms, count) tuples, then compute the averages.

fn avg_latency_per_endpoint(entries: &[AccessEntry]) -> HashMap<String, f64> {
    // TODO: Implement this function
    todo!()
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
//
// Return the top `n` endpoints sorted by request count (descending).
// Ties are broken alphabetically by endpoint name (ascending).
//
// Python equivalent:
//   Counter(e["endpoint"] for e in entries).most_common(n)
//
// Hint: Build a HashMap, collect into a Vec, sort, then truncate.

fn top_endpoints(entries: &[AccessEntry], n: usize) -> Vec<(String, usize)> {
    // TODO: Implement this function
    todo!()
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
//
// Return a sorted Vec of endpoint names where the error rate exceeds the
// given threshold. Error = status >= 400.
// Error rate = error_count / total_count for that endpoint.
//
// Python equivalent:
//   totals = Counter(e["endpoint"] for e in entries)
//   errors = Counter(e["endpoint"] for e in entries if e["status"] >= 400)
//   [ep for ep in sorted(totals) if errors.get(ep, 0) / totals[ep] > threshold]
//
// Hint: Build two maps (total count and error count), then compare ratios.

fn error_heavy_endpoints(entries: &[AccessEntry], threshold: f64) -> Vec<String> {
    // TODO: Implement this function
    todo!()
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
//
// Given two slices of AccessEntry, return a single HashMap with the combined
// status code counts.
//
// Python equivalent:
//   Counter(e["status"] for e in set1) + Counter(e["status"] for e in set2)
//
// Hint: Use entry().and_modify().or_insert() to merge without overwriting.

fn merge_status_counts(
    set1: &[AccessEntry],
    set2: &[AccessEntry],
) -> HashMap<u16, usize> {
    // TODO: Implement this function
    todo!()
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
    println!("Run with: rustc skeleton.rs --edition 2021 --test && ./skeleton");
    println!("Complete the TODO tasks, then run the tests to check your work!");
}
