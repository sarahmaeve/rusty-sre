# Challenge 10: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Infrastructure Diff Engine

**Task 1 — find_unmonitored_hosts.** Set difference:
`inventory.difference(monitoring).cloned().collect()`, then sort the Vec for
deterministic output.

**Task 2 — find_hosts_in_all_sources.** Either chain two intersections, or
filter one set by membership in the other two:
`inventory.iter().filter(|h| monitoring.contains(*h) && deployment.contains(*h))`.

**Task 3 — detect_config_drift.** Build key sets from both maps
(`map.keys().cloned().collect::<HashSet<_>>()`), then `difference` for
added/removed and `intersection` filtered by `old.get(k) != new.get(k)` for
changed. Sort all three.

**Task 4 — dedup_alert_stream.** `HashSet::insert` returns `true` only when
the item was new — perfect as a filter while you push into the output Vec.

**Task 5 — partition_by_allowlist.** One pass over the stream, pushing into
`allowed`/`denied` based on `allowlist.contains(ip)` — O(1) per check.

**Task 6 — aggregate_metrics.** `HashMap::with_capacity(estimated_keys)` to
pre-allocate, then the Entry counting pattern with `f64`:
`*agg.entry(name.clone()).or_insert(0.0) += *value;`.

## Debug: Fleet Health Analyzer

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "borrow of moved value: `monitored`"
  (E0382).
- **Where:** `find_hosts_needing_attention` / `merge_host_lists`.
- **Fix:** `merge_host_lists` takes both sets by value, so the
  `intersection` below uses moved values. Merging doesn't need ownership —
  change the signature to `(a: &HashSet<String>, b: &HashSet<String>)`
  (the body `a | b` already works on references) and call it with
  `&monitored, &healthy`.

### Bug 2

- **Symptom:** `test_find_missing_from_monitoring` fails — `legacy-01`
  shows up in the result.
- **Where:** `find_missing_from_monitoring`.
- **Fix:** `symmetric_difference` is Python's `a ^ b` — unmatched items in
  *either* direction. You want items in inventory but not in monitoring,
  which is `difference` (Python's `a - b`).

### Bug 3

- **Symptom:** Doesn't compile — "the trait bound `HostInfo: Eq` is not
  satisfied" / `Hash` is not implemented (E0277) at the `insert` call.
- **Where:** the `HostInfo` struct definition.
- **Fix:** `HashSet` membership requires `Hash + Eq`. In Python any object
  with `__hash__`/`__eq__` qualifies; in Rust you opt in with derives:
  `#[derive(Debug, Clone, PartialEq, Eq, Hash)]`.

### Bug 4

- **Symptom:** `test_remove_decommissioned` fails — exactly the wrong hosts
  survive.
- **Where:** `remove_decommissioned`.
- **Fix:** `retain()` *keeps* items where the closure returns true, and the
  closure returns true for decommissioned hosts — inverted. Negate it:
  `active_hosts.retain(|host| !decommissioned.contains(host));`.
