# Challenge 02: HashMaps — Text Processing & Incident Correlation

## Goal

Learn Rust's `HashMap<K, V>` for text processing and SRE data aggregation. If you know Python dicts, you already understand the concepts — this challenge teaches the Rust-specific patterns: ownership, the Entry API, and type-safe key lookups.

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `d = {}` | `HashMap::new()` |
| `d[k] = v` | `map.insert(k, v)` |
| `d.get(k)` → `None` | `map.get(&k)` → `Option<&V>` |
| `d[k]` → `KeyError` | `map[&k]` → **panic!** |
| `d.setdefault(k, v)` | `map.entry(k).or_insert(v)` |
| `defaultdict(list)` | `entry(k).or_insert_with(Vec::new)` |
| `Counter(items)` | `*entry(k).or_insert(0) += 1` |
| `d1.update(d2)` | `map1.extend(map2)` |

## Files

### `concept.rs` — Commented Explainer
12 sections with ~25 tests covering every HashMap pattern. Each section includes Python comparisons.

```bash
rustc concept.rs --edition 2024 --test && ./concept
```

### `skeleton.rs` — Metrics Aggregator (YOUR CHALLENGE)
6 progressively harder tasks building an SRE monitoring dashboard. Complete the `todo!()` stubs.

```bash
rustc skeleton.rs --edition 2024 --test && ./skeleton
```

**Tasks:**
1. Count requests by status code
2. Group endpoints by HTTP method (deduplicated)
3. Average latency per endpoint
4. Top N endpoints by request count
5. Detect error-heavy endpoints (error rate > threshold)
6. Merge status counts from two entry sets

### `debug.rs` — Incident Correlation Engine (FIND THE BUGS)
4 bugs hiding in a log correlation system — some won't compile, some produce
wrong results.

```bash
rustc debug.rs --edition 2024 --test && ./debug
```

Read the compiler errors and test failures, and work backwards to the cause.
If you get stuck, [`HINTS.md`](HINTS.md) reveals each bug in stages: symptom,
then location, then the fix.

### `HINTS.md`
Progressive hints for the skeleton tasks and the debug bugs.

### `data/access_log.txt`
Sample log data for experimentation. The skeleton tests use built-in test data.

### `solution/`
Reference solutions: `skeleton_solution.rs` (completed skeleton) and
`debug_solution.rs` (fixed debug). Try it yourself first!

```bash
cd solution
rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

## Key Takeaway

The **Entry API** (`map.entry(key).or_insert(...)`) is HashMap's killer feature. It replaces Python's `setdefault`, `defaultdict`, and `Counter` patterns with a single, type-safe, ownership-aware API. Master it and you'll rarely need anything else.
