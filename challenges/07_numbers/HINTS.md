# Challenge 07: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Byte-Rate Calculator

**Task 1 — bytes_to_kb.** Cast with `as` (`bytes as f64`), then divide by
`1024.0`.

**Task 2 — count_to_u32.** `u32::try_from(items.len())` already returns the
`Result<u32, TryFromIntError>` you need.

**Task 3 — increment_counter.** `current.saturating_add(delta)`.

**Task 4 — percent.** Check `denominator == 0` first, then cast both sides
to `f64` before dividing.

**Task 5 — rates_close.** `(a - b).abs() < 1e-6`.

**Task 6 — shrink_to_u8.** `u8::try_from(value).map_err(|_| value)` swaps
the opaque error for the original number.

## Debug: Capacity Planner

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "cannot add `u64` to `u32`" / mismatched
  types (E0308).
- **Where:** `project_utilization`.
- **Fix:** Rust never widens integers implicitly, even when it would be
  lossless — `u32 + u64` simply doesn't exist. Convert the narrow side
  explicitly: `u64::from(used) + max`. (`u64::from` is the lossless,
  compiler-verified conversion; `as` would also work but proves nothing.)

### Bug 2

- **Symptom:** Doesn't compile — mismatched types: expected `u32`, found
  `usize` (E0308).
- **Where:** `count_chunks`.
- **Fix:** `Vec::len()` returns `usize`, which is a distinct type from
  `u32` on every platform. Convert with `try_from` and pick an overflow
  policy — saturating keeps the spirit of this challenge:
  `u32::try_from(chunks.len()).unwrap_or(u32::MAX)`.

### Bug 3

- **Symptom:** `total_bytes_saturates_on_overflow` panics with "attempt to
  add with overflow" (debug builds; in release it would silently wrap
  instead — both are wrong).
- **Where:** `total_bytes`.
- **Fix:** Plain `+` has no overflow policy. For an SRE counter, pin at the
  ceiling so the report fails loudly: `total = total.saturating_add(c);`.

### Bug 4

- **Symptom:** `throughputs_agree_with_arith` fails — two mathematically
  equal throughputs are reported as different.
- **Where:** `throughputs_agree`.
- **Fix:** `(a + b) / 2.0` and `a/2.0 + b/2.0` are equal in algebra but not
  in f64 rounding, so `==` rejects them. Compare with an absolute epsilon:
  `(combined - summed_halves).abs() < 1e-9`.
