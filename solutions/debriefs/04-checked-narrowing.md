# 04 — Checked integer narrowing

Patch: [04-checked-narrowing.patch](../04-checked-narrowing.patch)

## Contract

Every `u64` representable as `u32` converts exactly. Larger values return
`SizeError::OutOfRange` with the rejected value.

## Root cause

An `as u32` cast discarded high bits instead of using the function's error channel.

## Why the symptom follows

Rust defines narrowing integer casts by truncation. `u32::MAX + 1` therefore wraps
to zero even though `narrow_size` returns `Result`.

## Repair strategy

Use `u32::try_from` and translate its conversion error to the existing domain
error.

## Verification

Run `make ex N=04`. Test zero, `u32::MAX`, and the first value above it.

## Tempting wrong fix

Clamping to `u32::MAX` avoids wrapping but invents a value and conceals data loss.

## References

[`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html),
[numeric casts](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast),
and Clippy's [`cast_possible_truncation`](https://rust-lang.github.io/rust-clippy/master/#cast_possible_truncation).
