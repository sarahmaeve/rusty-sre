// ============================================================================
// Challenge 02: HashMaps in Rust — A Comprehensive Guide for SRE Engineers
// ============================================================================
//
// If you know Python dicts, you already know the "why" of HashMaps.
// This guide focuses on the "how" — Rust's ownership model, the Entry API,
// and patterns that replace Python's defaultdict/Counter.
//
// Run tests:  rustc concept.rs --edition 2024 --test && ./concept

use std::collections::{BTreeMap, HashMap};

// ============================================================================
// SECTION 1: CREATING HASHMAPS
// ============================================================================
//
// Python:
//   d = {}                        # empty dict
//   d = {"key": "value"}          # literal
//
// Rust:
//   let mut m = HashMap::new();   // empty — type inferred from first insert
//   let m = HashMap::from([...])  // from array of (key, value) tuples
//   let m = HashMap::with_capacity(100); // pre-allocate like dict hint

#[test]
fn test_creating_empty() {
    let mut map: HashMap<String, i32> = HashMap::new();
    // Type annotation needed here because there's no insert to infer from.
    assert!(map.is_empty());
    map.insert("requests".to_string(), 42);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_creating_from_literal() {
    // HashMap::from() takes an array of (K, V) tuples.
    // Python: d = {"auth": 3, "gateway": 1}
    let error_counts = HashMap::from([
        ("auth", 3),
        ("gateway", 1),
        ("payments", 0),
    ]);
    assert_eq!(error_counts.len(), 3);
    assert_eq!(error_counts["auth"], 3);
}

#[test]
fn test_creating_with_capacity() {
    // Pre-allocate to avoid rehashing when you know the approximate size.
    // Python doesn't expose this, but CPython does it internally.
    let map: HashMap<String, Vec<String>> = HashMap::with_capacity(100);
    assert!(map.capacity() >= 100);
    assert!(map.is_empty()); // capacity != length
}

// ============================================================================
// SECTION 2: CREATING FROM ITERATORS
// ============================================================================
//
// Python:
//   dict(zip(keys, values))
//   {k: v for k, v in iterable}
//
// Rust:
//   keys.iter().zip(values.iter()).collect()
//   iterable.into_iter().collect()

#[test]
fn test_from_zip() {
    let services = vec!["auth", "gateway", "db"];
    let statuses = vec!["healthy", "degraded", "healthy"];

    // .zip() pairs elements; .collect() gathers into a HashMap.
    // Python: dict(zip(services, statuses))
    let health: HashMap<&str, &str> = services
        .iter()
        .copied()
        .zip(statuses.iter().copied())
        .collect();

    assert_eq!(health["auth"], "healthy");
    assert_eq!(health["gateway"], "degraded");
}

#[test]
fn test_from_iter_of_tuples() {
    // Any iterator of (K, V) tuples can be collected into a HashMap.
    // Python: {s: len(s) for s in services}
    let services = vec!["auth", "gateway", "db"];
    let name_lengths: HashMap<&str, usize> = services
        .iter()
        .map(|&s| (s, s.len()))
        .collect();

    assert_eq!(name_lengths["auth"], 4);
    assert_eq!(name_lengths["gateway"], 7);
}

// ============================================================================
// SECTION 3: BASIC OPERATIONS
// ============================================================================
//
// Python:                          Rust:
//   d[k] = v                       map.insert(k, v)  — returns Option<old_value>!
//   d[k]            (KeyError)     map[&k]            (panics!)
//   d.get(k)        (None)         map.get(&k)        (Option<&V>)
//   del d[k]                       map.remove(&k)     — returns Option<V>
//   k in d                         map.contains_key(&k)
//   len(d)                         map.len()

#[test]
fn test_insert_returns_old_value() {
    let mut map = HashMap::new();

    // First insert returns None — there was no previous value.
    let old = map.insert("auth", 1);
    assert_eq!(old, None);

    // Second insert returns Some(old_value) — this is how Rust tells you
    // that you just overwrote something. Python silently overwrites.
    let old = map.insert("auth", 5);
    assert_eq!(old, Some(1));
    assert_eq!(map["auth"], 5);
}

#[test]
fn test_get_vs_index() {
    let map = HashMap::from([("auth", 3), ("gateway", 1)]);

    // map[&key] panics if key is missing — like Python's d[k] raising KeyError.
    assert_eq!(map["auth"], 3);

    // map.get(&key) returns Option<&V> — like Python's d.get(k) returning None.
    assert_eq!(map.get("auth"), Some(&3));
    assert_eq!(map.get("unknown"), None);
}

#[test]
fn test_remove_and_contains() {
    let mut map = HashMap::from([("auth", 3), ("gateway", 1)]);

    assert!(map.contains_key("auth"));

    // remove() returns the value if present — useful for "take" semantics.
    let removed = map.remove("auth");
    assert_eq!(removed, Some(3));
    assert!(!map.contains_key("auth"));

    // Removing a missing key returns None (no panic).
    assert_eq!(map.remove("nonexistent"), None);
}

// ============================================================================
// SECTION 4: THE ENTRY API
// ============================================================================
//
// The Entry API is HashMap's killer feature. It eliminates the
// "check-then-insert" pattern that's clunky in every language.
//
// Python equivalents:
//   d.setdefault(k, default)       → entry(k).or_insert(default)
//   defaultdict(factory)           → entry(k).or_insert_with(factory)
//   Counter / d[k] += 1           → *entry(k).or_insert(0) += 1
//
// There is no clean Python equivalent for and_modify().or_insert() —
// it handles "update existing OR create new" in one atomic-looking call.

#[test]
fn test_or_insert() {
    let mut config: HashMap<&str, &str> = HashMap::new();

    // or_insert: insert default if key is absent, return &mut V in both cases.
    // Python: d.setdefault("timeout", "30s")
    config.entry("timeout").or_insert("30s");
    config.entry("timeout").or_insert("60s"); // no-op: key already exists

    assert_eq!(config["timeout"], "30s");
}

#[test]
fn test_or_insert_with() {
    let mut caches: HashMap<String, Vec<String>> = HashMap::new();

    // or_insert_with: call closure ONLY if key is absent.
    // Python: defaultdict(list) — but explicit and per-call.
    caches
        .entry("auth".to_string())
        .or_insert_with(Vec::new)
        .push("token_cache".to_string());

    caches
        .entry("auth".to_string())
        .or_insert_with(Vec::new)
        .push("session_cache".to_string());

    assert_eq!(caches["auth"].len(), 2);
}

#[test]
fn test_and_modify_or_insert() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    // and_modify: runs closure if key exists. or_insert: sets default if absent.
    // No clean Python one-liner for this pattern.
    for service in &["auth", "gateway", "auth", "auth"] {
        scores
            .entry(service)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    assert_eq!(scores["auth"], 3);
    assert_eq!(scores["gateway"], 1);
}

// ============================================================================
// SECTION 5: COUNTING PATTERN
// ============================================================================
//
// Python: collections.Counter(words)
// Rust: fold/loop with entry(word).or_insert(0), then += 1

#[test]
fn test_word_frequency() {
    let log_line = "ERROR auth ERROR auth ERROR gateway INFO auth";
    let words: Vec<&str> = log_line.split_whitespace().collect();

    // Python: Counter(words) → {"ERROR": 3, "auth": 3, ...}
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in &words {
        *freq.entry(word).or_insert(0) += 1;
    }

    assert_eq!(freq["ERROR"], 3);
    assert_eq!(freq["auth"], 3);
    assert_eq!(freq["gateway"], 1);
    assert_eq!(freq["INFO"], 1);
}

#[test]
fn test_status_code_counting() {
    let status_codes = vec![200, 200, 404, 200, 500, 404, 200];

    let mut counts: HashMap<u16, usize> = HashMap::new();
    for &code in &status_codes {
        *counts.entry(code).or_insert(0) += 1;
    }

    assert_eq!(counts[&200], 4);
    assert_eq!(counts[&404], 2);
    assert_eq!(counts[&500], 1);
}

// ============================================================================
// SECTION 6: GROUPING PATTERN
// ============================================================================
//
// Python: from collections import defaultdict
//         groups = defaultdict(list)
//         for item in items: groups[key(item)].append(item)
//
// Rust: entry(key).or_insert_with(Vec::new).push(item)

#[test]
fn test_group_by_level() {
    let logs = vec![
        ("auth", "ERROR"),
        ("gateway", "INFO"),
        ("auth", "INFO"),
        ("db", "ERROR"),
        ("gateway", "ERROR"),
    ];

    // Group services by their log level.
    // Python: defaultdict(list) then append
    let mut by_level: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(service, level) in &logs {
        by_level.entry(level).or_insert_with(Vec::new).push(service);
    }

    assert_eq!(by_level["ERROR"], vec!["auth", "db", "gateway"]);
    assert_eq!(by_level["INFO"], vec!["gateway", "auth"]);
}

#[test]
fn test_group_errors_by_service() {
    let errors = vec![
        ("auth", "token expired"),
        ("auth", "invalid creds"),
        ("db", "connection lost"),
        ("auth", "rate limited"),
    ];

    let mut by_service: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(service, msg) in &errors {
        by_service.entry(service).or_insert_with(Vec::new).push(msg);
    }

    assert_eq!(by_service["auth"].len(), 3);
    assert_eq!(by_service["db"].len(), 1);
}

// ============================================================================
// SECTION 7: ITERATION
// ============================================================================
//
// Python:
//   for k, v in d.items():    → for (k, v) in &map
//   for k in d:               → for k in map.keys()
//   for v in d.values():      → for v in map.values()
//
// KEY DIFFERENCE: HashMap iteration order is NOT guaranteed!
// Python 3.7+ dicts preserve insertion order. Rust HashMaps do NOT.
// If you need ordering, use BTreeMap or collect into a Vec and sort.

#[test]
fn test_iteration_by_reference() {
    let map = HashMap::from([("auth", 3), ("gateway", 1), ("db", 2)]);

    // Iterate by reference — map is not consumed.
    let mut total = 0;
    for (_service, &count) in &map {
        total += count;
    }
    assert_eq!(total, 6);

    // map is still usable after iteration by reference.
    assert_eq!(map.len(), 3);
}

#[test]
fn test_keys_and_values() {
    let map = HashMap::from([("auth", 3), ("gateway", 1)]);

    let mut keys: Vec<&&str> = map.keys().collect();
    keys.sort(); // Must sort! HashMap order is not guaranteed.
    assert_eq!(keys, vec![&"auth", &"gateway"]);

    let mut values: Vec<&i32> = map.values().collect();
    values.sort();
    assert_eq!(values, vec![&1, &3]);
}

#[test]
fn test_iter_mut() {
    let mut map = HashMap::from([("auth", 3), ("gateway", 1)]);

    // iter_mut() gives (&K, &mut V) — mutate values in place.
    for (_service, count) in map.iter_mut() {
        *count *= 2;
    }

    assert_eq!(map["auth"], 6);
    assert_eq!(map["gateway"], 2);
}

#[test]
fn test_into_iter_consumes() {
    let map = HashMap::from([("auth", 3), ("gateway", 1)]);

    // into_iter() consumes the map — you own the keys and values.
    let mut pairs: Vec<(&str, i32)> = map.into_iter().collect();
    pairs.sort_by_key(|&(k, _)| k);

    assert_eq!(pairs, vec![("auth", 3), ("gateway", 1)]);
    // map is now consumed — can't use it anymore.
}

// ============================================================================
// SECTION 8: OWNERSHIP WITH HASHMAPS
// ============================================================================
//
// Python: dicts hold references to objects. Reassigning a variable doesn't
//         affect the dict entry.
// Rust:   HashMap OWNS its keys and values. insert() MOVES data into the map.
//         You must use String (owned) for keys, not &str (borrowed), when
//         the map needs to outlive the source data.

#[test]
fn test_ownership_move() {
    let mut map = HashMap::new();
    let service = String::from("auth");

    map.insert(service, 42);
    // `service` has been MOVED into the map. This would not compile:
    // println!("{}", service); // error: value used after move

    // To keep using the original, clone before inserting:
    let service2 = String::from("gateway");
    map.insert(service2.clone(), 10);
    assert_eq!(map["gateway"], 10);
    // service2 is still valid because we cloned it.
    assert_eq!(service2, "gateway");
}

#[test]
fn test_get_returns_reference() {
    let map = HashMap::from([("auth".to_string(), 42)]);

    // get() returns Option<&V>, not Option<V>.
    // You're borrowing, not taking ownership.
    let val: Option<&i32> = map.get("auth");
    assert_eq!(val, Some(&42));

    // To get an owned copy, use .copied() (for Copy types) or .cloned().
    let owned: Option<i32> = map.get("auth").copied();
    assert_eq!(owned, Some(42));
}

#[test]
fn test_str_vs_string_keys() {
    // With &str keys, the map borrows — data must outlive the map.
    let map_borrowed: HashMap<&str, i32> = HashMap::from([("auth", 1)]);
    assert_eq!(map_borrowed["auth"], 1);

    // With String keys, the map owns — no lifetime concerns.
    let map_owned: HashMap<String, i32> =
        HashMap::from([("auth".to_string(), 1)]);
    assert_eq!(map_owned["auth"], 1);
}

// ============================================================================
// SECTION 9: MERGING AND UPDATING
// ============================================================================
//
// Python:
//   d1.update(d2)        → last-write-wins merge
//   d1 | d2              → Python 3.9+ merge operator
//
// Rust:
//   map1.extend(map2)    → last-write-wins (like Python's update)
//   Entry API            → custom merge logic (sum, max, etc.)

#[test]
fn test_extend_last_write_wins() {
    let mut counts = HashMap::from([("auth", 3), ("gateway", 1)]);
    let new_counts = HashMap::from([("auth", 10), ("db", 5)]);

    // extend() overwrites existing keys — like Python's d.update().
    counts.extend(new_counts);

    assert_eq!(counts["auth"], 10); // overwritten
    assert_eq!(counts["gateway"], 1); // unchanged
    assert_eq!(counts["db"], 5); // added
}

#[test]
fn test_merge_with_entry_api() {
    let mut total = HashMap::from([("auth", 3), ("gateway", 1)]);
    let batch = HashMap::from([("auth", 10), ("db", 5)]);

    // Sum values instead of overwriting — no Python one-liner for this.
    for (service, count) in batch {
        *total.entry(service).or_insert(0) += count;
    }

    assert_eq!(total["auth"], 13); // 3 + 10
    assert_eq!(total["gateway"], 1); // unchanged
    assert_eq!(total["db"], 5); // new
}

// ============================================================================
// SECTION 10: HASHMAP vs BTREEMAP
// ============================================================================
//
// HashMap: unordered, O(1) average lookup — your default choice.
// BTreeMap: sorted by key, O(log n) lookup — use when you need ordering
//           or range queries.
//
// Python: dict (insertion-ordered since 3.7) doesn't have a built-in
//         sorted equivalent. You'd sort keys manually.

#[test]
fn test_btreemap_is_sorted() {
    let mut bt = BTreeMap::new();
    bt.insert("gateway", 1);
    bt.insert("auth", 3);
    bt.insert("db", 2);

    // BTreeMap iterates in key order (alphabetical for strings).
    let keys: Vec<&&str> = bt.keys().collect();
    assert_eq!(keys, vec![&"auth", &"db", &"gateway"]);
}

#[test]
fn test_btreemap_range_queries() {
    let mut latencies = BTreeMap::new();
    latencies.insert(100u64, "fast");
    latencies.insert(250, "moderate");
    latencies.insert(500, "slow");
    latencies.insert(1000, "critical");
    latencies.insert(2000, "timeout");

    // Range query: find all latencies between 200 and 1000 (inclusive).
    // HashMap cannot do this efficiently.
    let slow_range: Vec<(&u64, &&str)> = latencies.range(200..=1000).collect();
    assert_eq!(slow_range.len(), 3); // 250, 500, 1000
}

// ============================================================================
// SECTION 11: HASHING REQUIREMENTS
// ============================================================================
//
// HashMap keys must implement Hash + Eq.
//   - All integer types, bool, char, String, &str: yes.
//   - f32/f64: NO! NaN != NaN breaks the Eq contract.
//   - Custom structs: derive Hash, Eq, PartialEq.
//
// Python: dict keys must be hashable (immutable). Same idea, less explicit.

#[derive(Hash, Eq, PartialEq, Debug)]
struct ServiceEndpoint {
    service: String,
    path: String,
}

#[test]
fn test_custom_struct_as_key() {
    let mut latencies: HashMap<ServiceEndpoint, u64> = HashMap::new();

    let endpoint = ServiceEndpoint {
        service: "auth".to_string(),
        path: "/login".to_string(),
    };
    latencies.insert(endpoint, 150);

    let lookup = ServiceEndpoint {
        service: "auth".to_string(),
        path: "/login".to_string(),
    };
    assert_eq!(latencies[&lookup], 150);
}

#[test]
fn test_tuple_keys() {
    // Tuples of hashable types are also hashable — convenient for compound keys.
    let mut error_rates: HashMap<(&str, u16), f64> = HashMap::new();
    error_rates.insert(("auth", 500), 0.05);
    error_rates.insert(("auth", 404), 0.12);
    error_rates.insert(("gateway", 500), 0.01);

    assert_eq!(error_rates[&("auth", 500)], 0.05);
}

// ============================================================================
// SECTION 12: PRACTICAL SRE PATTERNS
// ============================================================================
//
// Real-world SRE patterns combining everything above.

#[test]
fn test_log_level_counting() {
    // Pattern: Count log entries by level (like a simplified Splunk query).
    let log_levels = vec!["INFO", "ERROR", "INFO", "WARN", "ERROR", "ERROR", "INFO"];

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &level in &log_levels {
        *counts.entry(level).or_insert(0) += 1;
    }

    assert_eq!(counts["INFO"], 3);
    assert_eq!(counts["ERROR"], 3);
    assert_eq!(counts["WARN"], 1);
}

#[test]
fn test_group_errors_by_service_sre() {
    // Pattern: Group error messages by service for incident triage.
    let incidents = vec![
        ("auth", "token expired"),
        ("auth", "rate limited"),
        ("db", "connection reset"),
        ("auth", "cert invalid"),
        ("db", "timeout"),
    ];

    let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(service, msg) in &incidents {
        grouped.entry(service).or_insert_with(Vec::new).push(msg);
    }

    // Sort for deterministic assertions (HashMap has no guaranteed order).
    let mut auth_errors = grouped.remove("auth").unwrap();
    auth_errors.sort();
    assert_eq!(
        auth_errors,
        vec!["cert invalid", "rate limited", "token expired"]
    );
}

#[test]
fn test_dedup_and_count() {
    // Pattern: Deduplicate alerts while counting occurrences.
    // "We got 47 alerts but how many are unique?"
    let alerts = vec![
        "disk full on node-3",
        "high CPU on node-1",
        "disk full on node-3",
        "OOM on node-2",
        "high CPU on node-1",
        "high CPU on node-1",
        "disk full on node-3",
    ];

    let mut alert_counts: HashMap<&str, usize> = HashMap::new();
    for alert in &alerts {
        *alert_counts.entry(alert).or_insert(0) += 1;
    }

    assert_eq!(alert_counts.len(), 3); // 3 unique alerts
    assert_eq!(alert_counts["disk full on node-3"], 3);
    assert_eq!(alert_counts["high CPU on node-1"], 3);
    assert_eq!(alert_counts["OOM on node-2"], 1);

    // Find the noisiest alert:
    let noisiest = alert_counts
        .iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(&alert, _)| alert);

    // Both "disk full on node-3" and "high CPU on node-1" have count 3.
    // max_by_key returns one of them (unspecified which).
    assert!(
        noisiest == Some("disk full on node-3")
            || noisiest == Some("high CPU on node-1")
    );
}

#[test]
fn test_multi_map_correlation() {
    // Pattern: Correlate data across multiple maps (like joining tables).
    let error_counts = HashMap::from([("auth", 15), ("gateway", 3), ("db", 8)]);
    let request_counts = HashMap::from([("auth", 1000), ("gateway", 5000), ("db", 2000)]);

    // Compute error rate per service.
    let mut error_rates: HashMap<&str, f64> = HashMap::new();
    for (&service, &errors) in &error_counts {
        if let Some(&requests) = request_counts.get(service) {
            error_rates.insert(service, errors as f64 / requests as f64);
        }
    }

    assert!((error_rates["auth"] - 0.015).abs() < 0.001);
    assert!((error_rates["db"] - 0.004).abs() < 0.001);

    // Find services exceeding 1% error rate.
    let mut hot_services: Vec<&str> = error_rates
        .iter()
        .filter(|&(_, &rate)| rate > 0.01)
        .map(|(&service, _)| service)
        .collect();
    hot_services.sort();
    assert_eq!(hot_services, vec!["auth"]);
}

fn main() {
    println!("Run with: rustc concept.rs --edition 2024 --test && ./concept");
    println!("This file is meant to be run as tests, not as a binary.");
}
