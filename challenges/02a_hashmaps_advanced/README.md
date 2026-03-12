# Challenge 02a: HashSets & HashMap Performance

## Goal

Learn Rust's `HashSet` for set operations and understand HashMap performance characteristics. This challenge assumes you've completed Challenge 02 (HashMap basics, Entry API, counting/grouping patterns).

Two focus areas:
- **Part A:** HashSet — Rust's equivalent of Python's `set()`, including set algebra
- **Part B:** Performance — capacity planning, hashing costs, `drain()`/`retain()`, benchmarking

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `s = set()` | `HashSet::new()` |
| `s = {1, 2, 3}` | `HashSet::from([1, 2, 3])` |
| `s.add(x)` → `None` | `set.insert(x)` → `bool`! |
| `s.discard(x)` | `set.remove(&x)` → `bool` |
| `x in s` | `set.contains(&x)` |
| `a \| b` (union) | `&a \| &b` |
| `a & b` (intersection) | `&a & &b` |
| `a - b` (difference) | `&a - &b` |
| `a ^ b` (symmetric diff) | `&a ^ &b` |
| `list(set(items))` | `items.collect::<HashSet<_>>()` |
| `{k:v for k,v in d.items() if cond}` | `map.retain(\|k, v\| cond)` |

## Files

### `concept.rs` — Commented Explainer
12 sections with 33 tests. Sections 1-6 cover HashSet, sections 7-12 cover performance.

```bash
rustc concept.rs --edition 2024 --test && ./concept
```

### `skeleton.rs` — Infrastructure Diff Engine (YOUR CHALLENGE)
6 tasks comparing infrastructure state across fleet inventories and monitoring systems.

```bash
rustc skeleton.rs --edition 2024 --test && ./skeleton
```

**Tasks:**
1. Find hosts in inventory but not monitored (set difference)
2. Find hosts present in ALL three data sources (multi-set intersection)
3. Detect configuration drift — added, removed, changed keys
4. Deduplicate alert stream preserving first-seen order
5. Partition IPs by allowlist (O(1) HashSet lookups)
6. Pre-sized metric aggregation with `with_capacity()`

### `debug.rs` — Fleet Health Analyzer (FIND THE BUGS)
4 bugs in a fleet analysis system. 2 won't compile, 2 produce wrong results.

```bash
rustc debug.rs --edition 2024 --test && ./debug
```

**Bugs to find:**
1. Use-after-move — sets consumed by a function, then used again (compile error)
2. `symmetric_difference` used where `difference` was intended (runtime — wrong results)
3. Struct missing `Hash`/`Eq` derives for HashSet usage (compile error)
4. `retain()` with inverted logic — keeps what it should remove (runtime — wrong results)

### `data/`
- `fleet_inventory.txt` — 30 hostnames (the "source of truth")
- `monitoring_hosts.txt` — 25 hostnames (overlapping but different)

### `solution/debug_solution.rs`
The fixed version of `debug.rs`. Try to fix it yourself first!

```bash
cd solution && rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

## Key Takeaways

- **`insert()` returns `bool`** — unlike HashMap's `Option<V>`. Use this for one-step "insert and check if new" patterns.
- **Set operators (`&a | &b`)** return owned sets; methods (`.union()`) return iterators. Choose based on whether you need a new set or just want to iterate.
- **`with_capacity()`** avoids rehashing when you know the size. Free performance.
- **`retain()`** filters in-place without reallocating. Watch the condition — it keeps items where the closure returns `true`.
