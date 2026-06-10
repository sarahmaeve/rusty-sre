# Challenge 02: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Metrics Aggregator

The Entry API is your best friend here — see `concept.rs` Section 4.

**Task 1 — count by status.** The counting pattern:
`*counts.entry(e.status).or_insert(0) += 1`.

**Task 2 — group endpoints.** Group with
`entry(method).or_insert_with(Vec::new)` (or `or_default()`), push each
endpoint, then sort + `dedup()` every Vec at the end.

**Task 3 — average latency.** Accumulate `(total_ms, count)` tuples per
endpoint in one pass, then map each to `total as f64 / count as f64`.

**Task 4 — top N.** Build a count map, collect into a `Vec<(String, usize)>`,
`sort_by` count descending with `then_with` on the name for ties, `truncate(n)`.

**Task 5 — error-heavy endpoints.** Build two maps (total count, error
count). `errors.get(...)` returns an `Option` — `copied().unwrap_or(0)`
gets you a plain number for the ratio.

**Task 6 — merge counts.** `entry(status).and_modify(|c| *c += 1).or_insert(1)`
merges without overwriting.

## Debug: Incident Correlation Engine

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** `test_count_errors_per_service` reports 1 error for `auth`
  instead of 5.
- **Where:** `count_errors_per_service`.
- **Fix:** `insert()` overwrites the existing value every time — like
  Python's `d[k] = 1` instead of `d[k] = d.get(k, 0) + 1`. Count with the
  Entry API: `*error_counts.entry(record.service.clone()).or_insert(0) += 1`.

### Bug 2

- **Symptom:** `test_worst_service` panics at runtime even though the input
  is valid.
- **Where:** `worst_service` — the loop over `known_services`.
- **Fix:** `map[&key]` panics when the key is missing (Python's `d[k]`
  raising `KeyError`), and `cache`/`scheduler` have no errors so they're not
  in the map. Use the non-panicking lookup:
  `error_counts.get(service).copied().unwrap_or(0)`.

### Bug 3

- **Symptom:** Doesn't compile — "cannot borrow `*vec` as mutable, as it is
  behind a `&` reference" (E0596).
- **Where:** `build_service_groups`.
- **Fix:** `get()` returns `Option<&Vec<String>>` — an immutable reference
  you can't `push` to. (In Python, `d.get(k)` hands you the actual mutable
  list; in Rust it doesn't.) `get_mut()` would let you push, but the
  insert-in-the-`None`-arm shape still fights the borrow checker. The Entry
  API does the whole dance in one line:
  `groups.entry(record.service.clone()).or_default().push(formatted)`.

### Bug 4

- **Symptom:** Doesn't compile — mismatched types comparing
  `Option<&usize>` with `usize` (E0308).
- **Where:** `generate_correlation_report`.
- **Fix:** `get()` returns `Option<&usize>`, not a number; Rust won't
  coerce it the way Python's truthiness does. Extract a plain count first —
  `let count = error_counts.get(service).copied().unwrap_or(0);` — then
  compare `count > threshold` and drop the later `count.unwrap()`.
