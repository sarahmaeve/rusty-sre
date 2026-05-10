// ============================================================================
// Challenge 02a: HashSets & HashMap Performance — A Guide for SRE Engineers
// ============================================================================
//
// This challenge assumes you've completed Challenge 02 (HashMaps). We won't
// re-teach HashMap basics, Entry API, or counting/grouping patterns.
//
// Two focus areas:
//   Part A (Sections 1-6):  HashSet — Rust's equivalent of Python's set()
//   Part B (Sections 7-12): Performance — capacity, hashing costs, benchmarking
//
// Run tests:  rustc concept.rs --edition 2024 --test && ./concept

use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

// ============================================================================
// SECTION 1: CREATING HASHSETS
// ============================================================================
//
// Python:
//   s = set()                 # empty set
//   s = {1, 2, 3}             # literal
//   s = set(iterable)         # from iterable (deduplicates)
//
// Rust:
//   HashSet::new()            // empty — type inferred from first insert
//   HashSet::from([1, 2, 3])  // from array
//   iter.collect::<HashSet<_>>() // from iterator (deduplicates)

#[test]
fn test_hashset_from_array() {
    // HashSet::from() takes an array, just like HashMap::from() takes
    // an array of tuples. Duplicates are silently dropped.
    let statuses = HashSet::from(["healthy", "degraded", "down", "healthy"]);
    assert_eq!(statuses.len(), 3); // "healthy" appears once
    assert!(statuses.contains("degraded"));
}

#[test]
fn test_hashset_from_iterator() {
    // Collecting an iterator into a HashSet deduplicates automatically.
    // Python: set(["auth", "auth", "gateway", "auth"])
    let services = vec!["auth", "auth", "gateway", "auth", "gateway", "db"];
    let unique: HashSet<&str> = services.into_iter().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn test_hashset_with_capacity() {
    // Pre-allocate when you know the approximate set size.
    let hosts: HashSet<String> = HashSet::with_capacity(1000);
    assert!(hosts.capacity() >= 1000);
    assert!(hosts.is_empty());
}

// ============================================================================
// SECTION 2: BASIC HASHSET OPERATIONS — insert() RETURNS bool!
// ============================================================================
//
// Python:
//   s.add(x)         → returns None (always)
//   s.remove(x)      → raises KeyError if missing
//   s.discard(x)     → silent if missing
//   x in s           → bool
//
// Rust:
//   set.insert(x)    → returns bool! true = new, false = already present
//   set.remove(&x)   → returns bool (no panic)
//   set.contains(&x) → bool
//
// The bool return from insert() is unlike HashMap::insert() which returns
// Option<V>. This is useful for "insert-and-check-if-new" in one call.

#[test]
fn test_insert_returns_bool() {
    let mut hosts = HashSet::new();

    // First insert: returns true (the value was new).
    assert!(hosts.insert("web-prod-01"));

    // Second insert of same value: returns false (already present).
    // Python's set.add() returns None regardless — Rust gives you info.
    assert!(!hosts.insert("web-prod-01"));

    assert_eq!(hosts.len(), 1);
}

#[test]
fn test_remove_and_contains() {
    let mut regions = HashSet::from(["us-east-1", "us-west-2", "eu-west-1"]);

    assert!(regions.contains("us-east-1"));

    // remove() returns true if the item was present, false otherwise.
    // Python: s.remove(x) raises KeyError; s.discard(x) is silent.
    // Rust's remove() is like discard() but tells you what happened.
    assert!(regions.remove("us-east-1"));
    assert!(!regions.remove("us-east-1")); // already gone
    assert!(!regions.remove("ap-south-1")); // never existed

    assert_eq!(regions.len(), 2);
}

#[test]
fn test_len_is_empty_clear() {
    let mut alerts = HashSet::from(["disk_full", "high_cpu", "oom"]);
    assert_eq!(alerts.len(), 3);
    assert!(!alerts.is_empty());

    alerts.clear();
    assert!(alerts.is_empty());
}

// ============================================================================
// SECTION 3: SET ALGEBRA — union, intersection, difference, symmetric_difference
// ============================================================================
//
// Python:                          Rust method:              Rust operator:
//   a | b    (union)               a.union(&b)              &a | &b
//   a & b    (intersection)        a.intersection(&b)       &a & &b
//   a - b    (difference)          a.difference(&b)         &a - &b
//   a ^ b    (symmetric diff)      a.symmetric_difference(&b)  &a ^ &b
//
// IMPORTANT: The methods return lazy iterators of &T references.
// The operators return owned HashSet<T> values.
// You must .collect() the method results to get a new set.

#[test]
fn test_union() {
    let inventory = HashSet::from(["web-01", "api-01", "db-01"]);
    let monitoring = HashSet::from(["web-01", "api-01", "cache-01"]);

    // Method: returns iterator of references. Must collect.
    let all_known: HashSet<&&str> = inventory.union(&monitoring).collect();
    assert_eq!(all_known.len(), 4);

    // Operator: returns owned HashSet. Often more convenient.
    // Python: inventory | monitoring
    let all_known_op: HashSet<&str> = &inventory | &monitoring;
    assert_eq!(all_known_op.len(), 4);
    assert!(all_known_op.contains("cache-01"));
}

#[test]
fn test_intersection() {
    let inventory = HashSet::from(["web-01", "api-01", "db-01"]);
    let monitoring = HashSet::from(["web-01", "api-01", "cache-01"]);

    // Items in BOTH sets.
    // Python: inventory & monitoring
    let both: HashSet<&str> = &inventory & &monitoring;
    assert_eq!(both.len(), 2);
    assert!(both.contains("web-01"));
    assert!(both.contains("api-01"));
}

#[test]
fn test_difference() {
    let inventory = HashSet::from(["web-01", "api-01", "db-01"]);
    let monitoring = HashSet::from(["web-01", "api-01", "cache-01"]);

    // Items in inventory but NOT in monitoring.
    // Python: inventory - monitoring
    let unmonitored: HashSet<&str> = &inventory - &monitoring;
    assert_eq!(unmonitored, HashSet::from(["db-01"]));

    // Items in monitoring but NOT in inventory (stale entries).
    let stale: HashSet<&str> = &monitoring - &inventory;
    assert_eq!(stale, HashSet::from(["cache-01"]));
}

#[test]
fn test_symmetric_difference() {
    let inventory = HashSet::from(["web-01", "api-01", "db-01"]);
    let monitoring = HashSet::from(["web-01", "api-01", "cache-01"]);

    // Items in EITHER set but not BOTH — the "mismatch" set.
    // Python: inventory ^ monitoring
    let mismatch: HashSet<&str> = &inventory ^ &monitoring;
    assert_eq!(mismatch, HashSet::from(["db-01", "cache-01"]));
}

#[test]
fn test_set_algebra_sre_scenario() {
    // Real-world: comparing host lists across data sources.
    let inventory = HashSet::from([
        "web-01", "web-02", "api-01", "db-01", "db-02",
    ]);
    let monitoring = HashSet::from([
        "web-01", "web-02", "api-01", "db-01", "legacy-01",
    ]);
    let deployment = HashSet::from([
        "web-01", "web-02", "api-01", "db-01", "db-02", "canary-01",
    ]);

    // Hosts in all three systems (fully consistent):
    let consistent: HashSet<&str> = &(&inventory & &monitoring) & &deployment;
    let mut consistent_sorted: Vec<&str> = consistent.into_iter().collect();
    consistent_sorted.sort();
    assert_eq!(consistent_sorted, vec!["api-01", "db-01", "web-01", "web-02"]);

    // Hosts deployed but not monitored (coverage gap):
    let unmonitored: HashSet<&str> = &deployment - &monitoring;
    assert!(unmonitored.contains("db-02"));
    assert!(unmonitored.contains("canary-01"));
}

// ============================================================================
// SECTION 4: HASHSET AS A DEDUPLICATION TOOL
// ============================================================================
//
// Python:
//   list(set(items))                  # dedup, loses order
//   list(dict.fromkeys(items))        # dedup, preserves insertion order (3.7+)
//
// Rust:
//   items.into_iter().collect::<HashSet<_>>()  # dedup, no order
//   seen-set + Vec pattern                     # dedup, preserves order

#[test]
fn test_dedup_basic() {
    let alerts = vec![
        "disk full on node-3",
        "high CPU on node-1",
        "disk full on node-3",
        "OOM on node-2",
        "high CPU on node-1",
    ];

    let unique: HashSet<&&str> = alerts.iter().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn test_dedup_preserve_order() {
    // SRE pattern: deduplicate an alert stream, keeping first occurrence order.
    // Python: list(dict.fromkeys(alerts))
    let alerts = vec![
        "disk full on node-3",
        "high CPU on node-1",
        "disk full on node-3",
        "OOM on node-2",
        "high CPU on node-1",
        "high CPU on node-1",
    ];

    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for alert in &alerts {
        // insert() returns true if the item was new — perfect for this pattern.
        if seen.insert(alert) {
            deduped.push(*alert);
        }
    }

    assert_eq!(deduped, vec![
        "disk full on node-3",
        "high CPU on node-1",
        "OOM on node-2",
    ]);
}

// ============================================================================
// SECTION 5: HASHSET WITH CUSTOM TYPES
// ============================================================================
//
// Like HashMap keys, HashSet values must implement Hash + Eq + PartialEq.
// f64 CANNOT be in a HashSet (NaN != NaN breaks Eq).

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Endpoint {
    host: String,
    port: u16,
}

#[test]
fn test_custom_type_in_hashset() {
    let mut endpoints = HashSet::new();

    endpoints.insert(Endpoint {
        host: "api-01".to_string(),
        port: 8080,
    });
    endpoints.insert(Endpoint {
        host: "api-01".to_string(),
        port: 8443,
    });
    // Duplicate — same host AND port:
    endpoints.insert(Endpoint {
        host: "api-01".to_string(),
        port: 8080,
    });

    assert_eq!(endpoints.len(), 2); // deduped by (host, port) pair
}

#[test]
fn test_tuple_as_set_element() {
    // Tuples of hashable types work too — lighter than a struct for ad-hoc use.
    let mut connections: HashSet<(&str, u16)> = HashSet::new();
    connections.insert(("10.0.0.1", 443));
    connections.insert(("10.0.0.2", 443));
    connections.insert(("10.0.0.1", 443)); // duplicate

    assert_eq!(connections.len(), 2);
}

// ============================================================================
// SECTION 6: CONVERTING BETWEEN HASHMAP AND HASHSET
// ============================================================================
//
// Python:
//   set(d.keys())                              → extract keys as set
//   {k: v for k, v in d.items() if k in allow} → filter map by set membership
//
// Rust:
//   map.keys().cloned().collect::<HashSet<_>>() → extract keys as set
//   map.retain(|k, _| allowed.contains(k))      → filter in-place by set

#[test]
fn test_keys_as_hashset() {
    let error_counts: HashMap<&str, usize> = HashMap::from([
        ("auth", 5),
        ("gateway", 1),
        ("db", 3),
    ]);

    let services_with_errors: HashSet<&str> =
        error_counts.keys().copied().collect();
    assert_eq!(services_with_errors.len(), 3);
    assert!(services_with_errors.contains("auth"));
}

#[test]
fn test_filter_map_by_set() {
    let all_configs: HashMap<&str, &str> = HashMap::from([
        ("auth", "config_a"),
        ("gateway", "config_b"),
        ("db", "config_c"),
        ("legacy", "config_d"),
    ]);

    let active_services = HashSet::from(["auth", "gateway", "db"]);

    // Filter to only active services.
    // Python: {k: v for k, v in d.items() if k in active_services}
    let active_configs: HashMap<&str, &str> = all_configs
        .into_iter()
        .filter(|(k, _)| active_services.contains(k))
        .collect();

    assert_eq!(active_configs.len(), 3);
    assert!(!active_configs.contains_key("legacy"));
}

#[test]
fn test_values_as_hashset() {
    // Collect unique values from a HashMap.
    // Python: set(d.values())
    let host_datacenter: HashMap<&str, &str> = HashMap::from([
        ("web-01", "us-east-1"),
        ("web-02", "us-east-1"),
        ("api-01", "us-west-2"),
        ("db-01", "eu-west-1"),
    ]);

    let datacenters: HashSet<&&str> = host_datacenter.values().collect();
    assert_eq!(datacenters.len(), 3); // 3 unique DCs from 4 hosts
}

// ============================================================================
// SECTION 7: CAPACITY, LOAD FACTOR, AND REHASHING
// ============================================================================
//
// HashMap (and HashSet) use hashbrown internally. When the load factor
// exceeds ~87.5%, the table rehashes into a larger allocation — copying
// every entry. with_capacity() avoids this when you know the size upfront.
//
// Python: CPython dicts rehash at 2/3 load factor. You can't control
//         initial capacity (dict doesn't accept a size hint).

#[test]
fn test_capacity_grows_on_insert() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let initial_cap = map.capacity();

    // Insert enough items to trigger at least one rehash.
    for i in 0..100 {
        map.insert(i, i * 2);
    }

    // Capacity has grown beyond initial (which was 0).
    assert!(map.capacity() > initial_cap);
    assert!(map.capacity() >= 100);
}

#[test]
fn test_with_capacity_avoids_realloc() {
    // Pre-allocate for 1000 items — no rehashing during inserts.
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(1000);
    let initial_cap = map.capacity();
    assert!(initial_cap >= 1000);

    for i in 0..1000 {
        map.insert(i, i);
    }

    // Capacity should not have grown — we pre-allocated enough.
    assert_eq!(map.capacity(), initial_cap);

    // To see the performance difference, uncomment and run with:
    //   rustc concept.rs --edition 2024 && ./concept
    //
    // fn bench_capacity() {
    //     let n = 100_000;
    //
    //     let start = Instant::now();
    //     let mut m1: HashMap<i32, i32> = HashMap::new();
    //     for i in 0..n { m1.insert(i, i); }
    //     println!("  Without capacity: {:?}", start.elapsed());
    //
    //     let start = Instant::now();
    //     let mut m2: HashMap<i32, i32> = HashMap::with_capacity(n as usize);
    //     for i in 0..n { m2.insert(i, i); }
    //     println!("  With capacity:    {:?}", start.elapsed());
    // }
}

// ============================================================================
// SECTION 8: CHOOSING THE RIGHT COLLECTION FOR THE JOB
// ============================================================================
//
// Collection         Lookup   Insert   Ordered?  Cache-friendly?
// ─────────────────  ───────  ───────  ────────  ───────────────
// HashMap            O(1)*    O(1)*    No        Moderate
// BTreeMap           O(lg n)  O(lg n)  Yes       Good (B-tree)
// Vec<(K,V)> + scan  O(n)     O(1)**   Insert    Excellent
// Vec<(K,V)> + bsearch O(lg n) O(n)    Yes       Excellent
//
// * amortized, assuming good hash distribution
// ** append is O(1), but lookup is O(n)
//
// Rule of thumb: for N < ~20 items, a sorted Vec with binary_search
// often beats HashMap because of cache locality. Profile to be sure.
//
// Python doesn't expose this choice — dict is your only built-in mapping.

#[test]
fn test_vec_as_small_map() {
    // For tiny maps (e.g., 5 status codes), a Vec can be simpler and faster.
    let status_names: Vec<(u16, &str)> = vec![
        (200, "OK"),
        (301, "Moved"),
        (404, "Not Found"),
        (500, "Internal Server Error"),
        (503, "Service Unavailable"),
    ];

    // Linear scan — fine for 5 items.
    let found = status_names.iter().find(|&&(code, _)| code == 404);
    assert_eq!(found, Some(&(404, "Not Found")));

    // Binary search on a sorted Vec — O(log n) after the sort.
    let found_idx = status_names.binary_search_by_key(&503, |&(code, _)| code);
    assert_eq!(found_idx, Ok(4));
    assert_eq!(status_names[4].1, "Service Unavailable");
}

#[test]
fn test_btreemap_for_range_queries() {
    // BTreeMap shines when you need ordered iteration or range queries.
    // HashMap cannot do this at all.
    let mut latencies = BTreeMap::new();
    latencies.insert(50u64, "fast");
    latencies.insert(200, "moderate");
    latencies.insert(500, "slow");
    latencies.insert(1000, "critical");
    latencies.insert(5000, "timeout");

    // "Give me all latency categories between 100ms and 1000ms."
    let in_range: Vec<(&u64, &&str)> = latencies.range(100..=1000).collect();
    assert_eq!(in_range.len(), 3); // 200, 500, 1000

    // Iteration is always sorted by key.
    let all_keys: Vec<&u64> = latencies.keys().collect();
    assert_eq!(all_keys, vec![&50, &200, &500, &1000, &5000]);
}

#[test]
fn test_all_three_same_semantics() {
    // All three give the same results for basic key→value lookups.
    let hashmap = HashMap::from([("auth", 5), ("db", 3)]);
    let btreemap = BTreeMap::from([("auth", 5), ("db", 3)]);
    let vec_map: Vec<(&str, i32)> = vec![("auth", 5), ("db", 3)];

    assert_eq!(hashmap.get("auth"), Some(&5));
    assert_eq!(btreemap.get("auth"), Some(&5));
    assert_eq!(
        vec_map.iter().find(|&&(k, _)| k == "auth").map(|(_, v)| v),
        Some(&5)
    );
}

// ============================================================================
// SECTION 9: THE COST OF HASHING — STRING vs INTEGER KEYS
// ============================================================================
//
// Hashing a String processes every byte: O(len).
// Hashing a u32/u64 is nearly free: O(1).
//
// For SRE metrics tables where services are a known enum, consider using
// integer IDs or a Rust enum as keys instead of Strings.
//
// Python: strings are also hashed byte-by-byte, but Python caches the hash
//         value on first computation. Rust recomputes each time.

#[test]
fn test_integer_keys() {
    // Instead of HashMap<String, usize>, use an integer-keyed map
    // when you have a known, small set of services.
    const AUTH: u8 = 0;
    const GATEWAY: u8 = 1;
    const DB: u8 = 2;

    let mut error_counts: HashMap<u8, usize> = HashMap::new();
    *error_counts.entry(AUTH).or_insert(0) += 5;
    *error_counts.entry(GATEWAY).or_insert(0) += 1;
    *error_counts.entry(DB).or_insert(0) += 3;

    assert_eq!(error_counts[&AUTH], 5);

    // For a fixed set of keys, you could also just use an array:
    let mut counts_array = [0usize; 3]; // [auth, gateway, db]
    counts_array[AUTH as usize] = 5;
    counts_array[DB as usize] = 3;
    assert_eq!(counts_array[AUTH as usize], 5);
}

#[test]
fn test_enum_keys() {
    // Rust enums as HashMap keys — type-safe and fast to hash.
    #[derive(Hash, Eq, PartialEq, Debug)]
    enum Service {
        Auth,
        Gateway,
        Database,
    }

    let mut counts: HashMap<Service, usize> = HashMap::new();
    *counts.entry(Service::Auth).or_insert(0) += 5;
    *counts.entry(Service::Gateway).or_insert(0) += 1;

    assert_eq!(counts[&Service::Auth], 5);
    assert_eq!(counts.get(&Service::Database), None);
}

// ============================================================================
// SECTION 10: drain() AND retain() — IN-PLACE FILTERING
// ============================================================================
//
// Python:
//   d = {k: v for k, v in d.items() if condition}   # creates a NEW dict
//
// Rust:
//   map.retain(|k, v| condition)    # modifies IN-PLACE, no reallocation
//   map.drain()                     # removes ALL, yields owned (K, V) pairs
//   set.retain(|v| condition)       # same for HashSet
//   set.drain()                     # removes all, yields owned values

#[test]
fn test_retain_hashmap() {
    let mut error_counts = HashMap::from([
        ("auth", 15),
        ("gateway", 1),
        ("db", 8),
        ("cache", 0),
        ("monitoring", 2),
    ]);

    // Keep only services with more than 5 errors.
    // Python: error_counts = {k: v for k, v in error_counts.items() if v > 5}
    error_counts.retain(|_, &mut v| v > 5);

    assert_eq!(error_counts.len(), 2);
    assert!(error_counts.contains_key("auth"));
    assert!(error_counts.contains_key("db"));
}

#[test]
fn test_retain_hashset() {
    let mut hosts = HashSet::from([
        "web-prod-01.us-east-1",
        "web-prod-02.us-east-1",
        "api-prod-01.us-west-2",
        "db-prod-01.us-east-1",
        "legacy-web-01.us-east-1",
    ]);

    // Remove all legacy hosts.
    // Python: hosts = {h for h in hosts if not h.startswith("legacy")}
    hosts.retain(|h| !h.starts_with("legacy"));

    assert_eq!(hosts.len(), 4);
    assert!(!hosts.contains("legacy-web-01.us-east-1"));
}

#[test]
fn test_drain_hashmap() {
    let mut metrics = HashMap::from([("cpu", 85.0), ("mem", 72.5), ("disk", 91.0)]);

    // drain() removes all entries AND yields them as owned (K, V) pairs.
    // The map is empty afterward, but retains its allocated capacity.
    let drained: Vec<(&str, f64)> = metrics.drain().collect();
    assert_eq!(drained.len(), 3);
    assert!(metrics.is_empty());
    assert!(metrics.capacity() > 0); // memory still allocated
}

#[test]
fn test_drain_for_batch_processing() {
    // SRE pattern: accumulate metrics in a buffer, drain to send to backend.
    let mut buffer: HashMap<String, Vec<f64>> = HashMap::new();

    buffer
        .entry("cpu_usage".to_string())
        .or_insert_with(Vec::new)
        .push(85.0);
    buffer
        .entry("cpu_usage".to_string())
        .or_insert_with(Vec::new)
        .push(87.5);
    buffer
        .entry("mem_usage".to_string())
        .or_insert_with(Vec::new)
        .push(72.0);

    // "Flush" the buffer: take ownership of all entries.
    let batch: HashMap<String, Vec<f64>> = buffer.drain().collect();
    assert_eq!(batch["cpu_usage"].len(), 2);
    assert!(buffer.is_empty()); // ready for next batch
}

// ============================================================================
// SECTION 11: ENTRY API vs CONTAINS_KEY — SINGLE vs DOUBLE LOOKUP
// ============================================================================
//
// The "check-then-insert" pattern does TWO hash lookups:
//   if !map.contains_key(&k) {   // lookup #1
//       map.insert(k, default);  // lookup #2 (rehashes k)
//   }
//
// The Entry API does ONE lookup:
//   map.entry(k).or_insert(default);  // single lookup
//
// For hot paths processing millions of entries, this 2x reduction matters.
// Python: d.setdefault(k, v) is a single internal lookup, like Entry API.

#[test]
fn test_double_lookup_pattern() {
    // The "bad" way — two lookups per key.
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let services = ["auth", "gateway", "auth", "db", "auth"];

    for &s in &services {
        if counts.contains_key(s) {
            // Lookup #1: contains_key hashes s, finds slot.
            let count = counts.get_mut(s).unwrap();
            // Lookup #2: get_mut hashes s again, finds same slot.
            *count += 1;
        } else {
            counts.insert(s, 1);
        }
    }

    assert_eq!(counts["auth"], 3);
}

#[test]
fn test_single_lookup_pattern() {
    // The "good" way — one lookup per key.
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let services = ["auth", "gateway", "auth", "db", "auth"];

    for &s in &services {
        // entry() hashes once, returns a mutable reference to the slot.
        *counts.entry(s).or_insert(0) += 1;
    }

    assert_eq!(counts["auth"], 3);
    // Same result, half the hashing work.
}

// ============================================================================
// SECTION 12: PRACTICAL — BENCHMARK HARNESS WITH std::time::Instant
// ============================================================================
//
// Without Cargo, we can't use the criterion benchmarking crate.
// std::time::Instant gives wall-clock measurements. Not as rigorous
// as criterion (no statistical analysis, warm-up, outlier removal),
// but sufficient for ballpark comparisons.
//
// NEVER use timing assertions in #[test] — they're flaky on CI.
// Instead, test the harness infrastructure and put timing in main().

/// Simple benchmark result — just a label and duration.
struct BenchResult {
    label: String,
    iterations: usize,
    total: std::time::Duration,
}

impl BenchResult {
    fn per_op_ns(&self) -> f64 {
        self.total.as_nanos() as f64 / self.iterations as f64
    }
}

/// Run a closure `iterations` times and measure total wall-clock time.
fn bench<F: FnMut()>(label: &str, iterations: usize, mut f: F) -> BenchResult {
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    BenchResult {
        label: label.to_string(),
        iterations,
        total: start.elapsed(),
    }
}

#[test]
fn test_instant_basic() {
    // Verify Instant works and elapsed > zero after real work.
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..10_000 {
        sum += i;
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_nanos() > 0);
    assert!(sum > 0); // prevent optimization
}

#[test]
fn test_bench_harness() {
    // Verify our bench() helper returns sensible results.
    let result = bench("test_op", 1000, || {
        let _ = HashMap::<i32, i32>::with_capacity(10);
    });

    assert_eq!(result.label, "test_op");
    assert_eq!(result.iterations, 1000);
    assert!(result.total.as_nanos() > 0);
    assert!(result.per_op_ns() > 0.0);
}

// This function is meant to be run from main(), not from tests.
// Uncomment the call in main() to see comparative benchmarks.
#[allow(dead_code)]
fn run_benchmarks() {
    let n = 50_000;
    println!("\n--- HashMap Benchmarks ({n} operations) ---\n");

    // Benchmark: with_capacity vs without
    let r1 = bench("HashMap::new() + insert", n, || {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for i in 0..100 {
            m.insert(i, i);
        }
    });

    let r2 = bench("with_capacity(100) + insert", n, || {
        let mut m: HashMap<usize, usize> = HashMap::with_capacity(100);
        for i in 0..100 {
            m.insert(i, i);
        }
    });

    println!("  {}: {:.1} ns/op", r1.label, r1.per_op_ns());
    println!("  {}: {:.1} ns/op", r2.label, r2.per_op_ns());

    // Benchmark: String keys vs integer keys
    let r3 = bench("String key lookup", n, || {
        let m: HashMap<String, i32> = HashMap::from([
            ("auth-service".to_string(), 1),
            ("gateway-service".to_string(), 2),
        ]);
        let _ = m.get("auth-service");
    });

    let r4 = bench("u32 key lookup", n, || {
        let m: HashMap<u32, i32> = HashMap::from([(0, 1), (1, 2)]);
        let _ = m.get(&0);
    });

    println!("  {}: {:.1} ns/op", r3.label, r3.per_op_ns());
    println!("  {}: {:.1} ns/op", r4.label, r4.per_op_ns());

    // Benchmark: HashSet::contains vs Vec::contains
    let set: HashSet<i32> = (0..1000).collect();
    let vec: Vec<i32> = (0..1000).collect();

    let r5 = bench("HashSet::contains (1000 items)", n, || {
        let _ = set.contains(&999);
    });

    let r6 = bench("Vec::contains (1000 items)", n, || {
        let _ = vec.contains(&999);
    });

    println!("  {}: {:.1} ns/op", r5.label, r5.per_op_ns());
    println!("  {}: {:.1} ns/op", r6.label, r6.per_op_ns());
}

fn main() {
    println!("Run with: rustc concept.rs --edition 2024 --test && ./concept");
    println!("This file is meant to be run as tests, not as a binary.");
    println!();
    println!("To run benchmarks, uncomment the line below and run:");
    println!("  rustc concept.rs --edition 2024 && ./concept");
    // run_benchmarks();
}
