# Challenge 07: Numbers and Conversions

Learn the integer types, what `as` actually does, and where Python's "numbers just work" assumptions break in Rust.

## Goal

Python integers are arbitrary-precision and silently widen across operations. Rust integers have fixed widths, can overflow, and refuse to mix types implicitly. This challenge teaches:

- The integer-type ladder: `i8/i16/i32/i64/i128`, `u8/u16/u32/u64/u128`, `isize/usize`
- Overflow handling: `wrapping_*`, `checked_*`, `saturating_*`, `overflowing_*`
- The `as` keyword — what it truncates, what it bit-casts, what to use instead
- `From`/`Into` for lossless conversions (compiler-verified)
- `TryFrom`/`TryInto` for fallible conversions (Result-typed)
- Why `f64 == f64` is rarely what you want
- Why `usize` and `u64` are different types

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `x = 10_000_000_000_000` | `let x: i64 = 10_000_000_000_000;` |
| `x = int(s)` | `let x: i32 = s.parse()?;` |
| `x = int(y)` (truncating) | `let x = y as i32;` |
| `x = int(y)` (safe) | `let x: i32 = y.try_into()?;` |
| `if a == b:` (floats) | `(a - b).abs() < EPSILON` |
| `len(xs)` | `xs.len()` (returns `usize`) |
| Python int never overflows | `x.saturating_add(y)` or `x.checked_add(y)` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 10 sections covering integer types, overflow, casts, From/TryFrom, floats |
| `skeleton.rs` | Byte-rate calculator — fill in six TODOs |
| `debug.rs` | Buggy capacity planner — find and fix 4 bugs |
| `HINTS.md` | Progressive hints for the skeleton tasks and the debug bugs |
| `solution/skeleton_solution.rs` | Reference implementation of `skeleton.rs` |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |

## How to Run

```bash
rustc concept.rs --edition 2024 --test && ./concept
rustc skeleton.rs --edition 2024 --test && ./skeleton
rustc debug.rs --edition 2024 --test && ./debug
cd solution && rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
cd solution && rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

Or, from the repo root:

```bash
make CH=07_numbers concept
make CH=07_numbers skeleton
make CH=07_numbers debug
make CH=07_numbers solution
```

## Skeleton Challenge: Byte-Rate Calculator

Six tasks build helpers for a metrics dashboard:

1. **`bytes_to_kb`** — `u64 → f64` conversion with `as`
2. **`count_to_u32`** — `usize → u32` via `try_from`
3. **`increment_counter`** — `saturating_add` so counters don't wrap
4. **`percent`** — division-by-zero handling, return `Option<f64>`
5. **`rates_close`** — float comparison with an absolute epsilon
6. **`shrink_to_u8`** — `u32 → u8` via `try_from`, return the original on overflow

## Debug Challenge: Capacity Planner

Four bugs that all stem from numeric-type misuse — some stop the program
compiling, some misbehave at runtime. Read the compiler errors and test
failures, and work backwards to the cause.

If you get stuck, [`HINTS.md`](HINTS.md) reveals each bug in stages: symptom,
then location, then the fix.

## Concepts Covered

1. The integer-type ladder and when to use which
2. The overflow behavior split: debug panic vs release wrap
3. `wrapping_*`, `checked_*`, `saturating_*`, `overflowing_*` — explicit overflow policies
4. `as` casts — what they truncate, sign-cast, and float-saturate
5. `From`/`Into` for lossless, compiler-verified conversions
6. `TryFrom`/`TryInto` for conversions that can fail
7. Why `usize` is its own type — for collection lengths and indices
8. Float `==` is a trap — use epsilon (or `total_cmp`/`partial_cmp` for ordering)
9. No implicit widening — every cross-type arithmetic operation needs a conversion
10. SRE patterns: saturating counters, divide-by-zero handling, percent calculations
