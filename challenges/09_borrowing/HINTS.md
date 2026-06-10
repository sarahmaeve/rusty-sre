# Challenge 09: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: SRE Alert Pipeline

**Task 1 — parse_alerts.** `into_iter()` consumes the input so you can move
each `String` out. `splitn(3, ':')` keeps any extra colons inside the third
part. `filter_map` parses and skips malformed messages in one pass.

**Task 2 — critical_alerts.** `alerts.iter().filter(...).collect()` —
collecting `&Alert` references compiles directly thanks to the lifetime
already written on the signature.

**Task 3 — escalate_warnings.** `iter_mut()` hands out `&mut Alert`; check
both conditions and assign a new `severity` string.

**Task 4 — extract_info_alerts.** `drain()` doesn't take a predicate, so
take the whole Vec out and split it:
`std::mem::take(alerts).into_iter().partition(|a| a.severity == "info")`,
then put the keepers back into `*alerts`.

**Task 5 — build_summary.** `iter().map(|a| format!(...))`, then
`.collect::<Vec<_>>().join("\n")`. `format!` reads through references —
no clone needed.

**Task 6 — split_alerts.** `split_off(index)` returns the tail and works on
the `mut alerts` binding. It panics if `index > len`, so guard the
past-the-end case first and return `(alerts, Vec::new())`.

## Debug: Incident Tracker

Four bugs — all compile errors. Each hint reveals more: symptom → where to
look → the fix.

### Bug 1

- **Symptom:** "use of moved value: `incidents`" (E0382).
- **Where:** `triage_incidents`.
- **Fix:** `count_incidents(incidents)` takes the Vec by value, so the
  partition below uses a moved value. Counting doesn't need ownership —
  change `count_incidents` to borrow (`incidents: &[Incident]`) and call it
  with `&incidents`.

### Bug 2

- **Symptom:** "cannot borrow `*incidents` as mutable because it is also
  borrowed as immutable" (E0502).
- **Where:** `enrich_with_runbooks`.
- **Fix:** The `find()` result keeps an immutable borrow of `incidents`
  alive across the `iter_mut()` loop. The find is also redundant — it only
  rediscovers the service name the loop already has. Drop it and compare
  directly inside one mutable pass:
  `for inc in incidents.iter_mut() { if &inc.service == service { inc.runbook = Some(runbook_url.to_string()); } }`.

### Bug 3

- **Symptom:** "cannot return value referencing local variable
  `unresolved`" (E0515).
- **Where:** `get_unresolved_titles`.
- **Fix:** The returned `&str`s point into a Vec that dies at the end of
  the function. Borrow from the *input* instead — the intermediate cloned
  Vec is unnecessary:
  `incidents.iter().filter(|i| !i.resolved).map(|i| i.title.as_str()).collect()`.

### Bug 4

- **Symptom:** "borrow of moved value: `active`" (E0382) at the
  `active.len()` line.
- **Where:** `generate_report`.
- **Fix:** `for incident in active` calls `into_iter()` and consumes the
  Vec. Iterate by reference instead — `for incident in &active` — so
  `active` is still alive for the totals below.
