# Challenge 01: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: SRE Metrics Collector

**Task 2 — add and clean.** `retain()` keeps the elements matching a
predicate; it is the idiomatic way to delete-by-condition.

**Task 3 — compute stats.** `f64` doesn't implement `Ord` (NaN breaks total
ordering), so `Iterator::min()`/`max()` don't apply. Use `fold()` with
`f64::min`/`f64::max`, or track min/max in a plain loop.

**Task 4 — filter.** `iter().filter(...).collect()` builds a new vector;
`copied()` turns the `&f64` items back into `f64`.

**Task 5 — deduplicate and sort.** Two routes: collect into a `HashSet` and
sort the result, or `sort()` then `dedup()` — `dedup()` only removes
*consecutive* duplicates, so sort first.

## Debug: Log Analyzer

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** `test_most_recent_entry` panics with "index out of bounds".
- **Where:** `most_recent_entry`.
- **Fix:** Valid indices run from `0` to `len() - 1`, so
  `entries[entries.len()]` is one past the end. Index with `len() - 1` — or
  skip indexing entirely and use `entries.last()`, which also handles the
  empty case.

### Bug 2

- **Symptom:** `test_remove_debug_entries` panics with "index out of bounds".
- **Where:** `remove_debug_entries`.
- **Fix:** Removing elements while walking indices has two problems: each
  `remove(i)` shifts later elements left, so the element right after a
  removal is never examined; and the range `0..entries.len()` is computed
  once up front, so `i` outruns the shrinking vector and the indexing
  panics. Note that this *compiles* — the borrow checker only rejects
  mutation inside a `for entry in &entries` borrow, not indexed access.
  The idiomatic fix: `entries.retain(|e| e.level != "DEBUG")`.

### Bug 3

- **Symptom:** `test_unique_levels` sees 9 levels instead of 4.
- **Where:** `unique_levels`.
- **Fix:** Every level is pushed unconditionally, duplicates included.
  Either check `seen.contains(&level)` before pushing, or collect into a
  `HashSet<String>` and sort the result into a `Vec`.

### Bug 4

- **Symptom:** `test_longest_message_empty` panics on an `unwrap()`.
- **Where:** `longest_message`.
- **Fix:** `max_by_key` returns `None` for an empty iterator, and
  `unwrap()` turns that into a panic. Map the `Option` through instead:
  `entries.iter().max_by_key(|e| e.message.len()).map(|e| e.message.clone())`.
