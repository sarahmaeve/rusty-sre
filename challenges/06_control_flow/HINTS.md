# Challenge 06: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: HTTP Status Classifier

**Task 1 — classify.** A `match` with inclusive range patterns
(`100..=199 => ...`). Make the whole match the function's return expression —
no `return`, no semicolon.

**Task 2 — is_retriable.** `match` (or `matches!`) with alternation and a
range: `408 | 429 | 500..=599`.

**Task 3 — short_label.** Build on `classify()` and `match` on the
`Category`. All arms return `&'static str` literals.

**Task 4 — parse_status.** `let Ok(code) = s.parse::<u16>() else { return None; };`
for the parse step, then a range check (`(100..=599).contains(&code)`) to
pick `Some`/`None`.

**Task 5 — first_error_code.** `Iterator::find` returns `Option<&u16>`;
`.copied()` turns it into `Option<u16>`.

## Debug: Response Analyzer

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "`if` and `else` have incompatible types"
  (E0308).
- **Where:** `classify_response`.
- **Fix:** An `if`/`else` used as an expression must produce the same type
  in every branch. The middle branch returns the integer `200` where the
  others return `&str`. The tests show both 2xx and 3xx label as
  `"success"` — return that.

### Bug 2

- **Symptom:** Doesn't compile — mismatched types: expected `u8`, found
  `&str` (E0308) in one match arm.
- **Where:** `priority`.
- **Fix:** Every `match` arm must produce the function's return type. The
  `"low"` arm returns the string `"low"` instead of its priority number —
  the tests expect `4`.

### Bug 3

- **Symptom:** `is_success_inclusive` fails on 299.
- **Where:** `is_success`.
- **Fix:** `200..299` is an *exclusive* range — it stops at 298. HTTP
  success codes run through 299 inclusive, which is spelled `200..=299`.

### Bug 4

- **Symptom:** `count_failures_counts` fails — the count is always 0.
- **Where:** `count_failures`.
- **Fix:** `count + 1;` computes a new value and immediately discards it —
  the trailing semicolon turns the expression into a statement whose result
  goes nowhere, and nothing assigns back to `count`. Mutate instead:
  `count += 1;`.
