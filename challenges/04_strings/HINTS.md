# Challenge 04: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Hostname Normalizer

**Task 1 — trim_whitespace.** `&str::trim()` returns a `&str`; use
`.to_string()` (or `.to_owned()`) to get the owned `String`.

**Task 2 — strip_port.** `str::split_once(':')` returns
`Option<(&str, &str)>`. `map` out the first half, `unwrap_or` back to the
original input.

**Task 3 — to_lower.** `str::to_lowercase()`.

**Task 4 — trim_trailing_dots.** `str::trim_end_matches('.')` is exactly
this.

**Task 5 — normalize_hostname.** Feed each helper's output into the next,
then check `is_empty()` to decide between `Ok` and `Err`.

## Debug: Log-Line Redactor

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — no method named `push_str` found for
  reference `&str` (E0599), plus the function returns nothing while the
  tests expect a `String`.
- **Where:** `add_redacted_marker`.
- **Fix:** A `&str` is an immutable borrow — you can't grow it. Return a
  new owned value instead: change the signature to `-> String` and build
  the result with `format!("{s} [REDACTED]")`.

### Bug 2

- **Symptom:** Doesn't compile — mismatched types: expected `&str`, found
  `String` (E0308) on the right side of `+`.
- **Where:** `build_redacted_line`.
- **Fix:** Rust's `+` for `String` takes a `&str` on the right (it moves
  the left side and appends the borrowed right side). Write
  `prefix + &suffix`, or sidestep the rule with `format!("{prefix}{suffix}")`.

### Bug 3

- **Symptom:** `first_three_chars_unicode` panics: "byte index 3 is not a
  char boundary".
- **Where:** `first_three_chars`.
- **Fix:** `s[0..3]` slices *bytes*, and UTF-8 characters can span several
  bytes — the slice boundary lands inside the `é` of `"café"`. Iterate by
  character instead: `s.chars().take(3).collect()`.

### Bug 4

- **Symptom:** `extract_service_lowercase` (and the integration test) fail —
  no service is ever extracted from normal lowercase logs.
- **Where:** `extract_service`.
- **Fix:** `key == "SERVICE"` only matches the all-caps form. Compare
  case-insensitively: `key.eq_ignore_ascii_case("service")`.
