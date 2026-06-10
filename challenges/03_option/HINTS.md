# Challenge 03: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Service Status Lookup

**Task 1 — find_owner.** `HashMap::get` returns `Option<&V>`. To turn
`Option<&String>` into `Option<&str>`, use `.map(String::as_str)`.

**Task 2 — parse_optional_severity.** Chain `.trim()`,
`.parse::<u8>().ok()`, and `.filter(|n| (1..=5).contains(n))`.

**Task 3 — first_critical.** `Iterator::find` returns `Option<&T>` — exactly
the signature you need.

**Task 4 — owner_team.** A textbook `Option::and_then` chain. Or use `?` on
each lookup — it works here because the function itself returns `Option`.

**Task 5 — summary_or_default.** `Option::map_or(default, f)`, or
`.map(...).unwrap_or((0.0, 1.0))`. No `unwrap()` — graceful handling is the
whole point.

## Debug: On-Call Dashboard

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — mismatched types: expected `&str`, found
  `Option<&str>` (E0308).
- **Where:** `lookup_owner`.
- **Fix:** `HashMap::get` returns `Option<&V>`, so the body produces
  `Option<&str>` while the signature claims a plain `&str`. There is no
  "always present" answer for a lookup that can miss — change the return
  type to `Option<&'a str>` (the test expects exactly that).

### Bug 2

- **Symptom:** Doesn't compile — "the `?` operator can only be used in a
  function that returns `Result` or `Option`" (E0277).
- **Where:** `first_severity`.
- **Fix:** `?` needs the surrounding function to have a "no value" branch to
  propagate into, and `u8` has none. Change the return type to `Option<u8>`
  and wrap the final value in `Some(...)` — or return `sev` directly since
  `alert.severity?` already yields `u8`... but then the empty-slice case
  still needs `None`, so `Option<u8>` it is.

### Bug 3

- **Symptom:** `owner_or_unknown_handles_missing` panics on `unwrap()` for
  the `search` service.
- **Where:** `owner_or_unknown`.
- **Fix:** The spec says unknown services display `"unknown"`. Replace the
  panicking `unwrap()` with a fallback:
  `owners.get(service).cloned().unwrap_or_else(|| "unknown".to_string())`.

### Bug 4

- **Symptom:** `worst_severity_treats_missing_as_5` fails — it gets 3 where
  5 is expected.
- **Where:** `worst_severity`.
- **Fix:** `unwrap_or(0)` treats a *missing* severity as the *lowest*
  priority, silently hiding it. The SRE-conservative default is the
  opposite: `a.severity.unwrap_or(5)` so missing data surfaces loudly.
  Choosing defaults that fail loudly is the lesson of this whole challenge.
