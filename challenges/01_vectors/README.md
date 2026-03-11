# Challenge 01: Vectors

Learn about `Vec<T>` — Rust's growable array type — and when to choose it over arrays, `VecDeque`, `LinkedList`, and `HashSet`.

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | Heavily commented explainer covering all major vector operations |
| `skeleton.rs` | SRE metrics collector — fill in the TODOs to complete it |
| `debug.rs` | Buggy log analyzer — find and fix 4 vector-related bugs |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |
| `data/log_entries.txt` | Sample log data |

## How to Run

Each file is a standalone Rust program with built-in tests. No Cargo needed.

```bash
# Concept explainer — all tests should pass
rustc concept.rs --edition 2021 --test && ./concept

# Skeleton challenge — tests will fail until you complete the TODOs
rustc skeleton.rs --edition 2021 --test && ./skeleton

# Debug challenge — tests will fail until you fix the bugs
rustc debug.rs --edition 2021 --test && ./debug

# Solution — all tests should pass
cd solution && rustc debug_solution.rs --edition 2021 --test && ./debug_solution
```

## Skeleton Challenge: SRE Metrics Collector

Complete five tasks related to collecting and processing response time metrics:

1. **Initialize** a vector of response times
2. **Add and clean** — push new entries, remove outliers above 400ms
3. **Compute stats** — calculate min, max, and average from a slice
4. **Filter** — extract entries above a threshold into a new vector
5. **Deduplicate and sort** — remove duplicate service names and sort them

## Debug Challenge: Log Analyzer

The log analyzer has four bugs rooted in vector misuse:

1. **Off-by-one indexing** — accessing one past the end of the vector
2. **Mutating while iterating** — removing elements during a loop
3. **Wrong data structure** — using `Vec` where `HashSet` is needed for uniqueness
4. **Empty vector panic** — calling `.unwrap()` on an empty iterator result

Find and fix all four to make the tests pass.
