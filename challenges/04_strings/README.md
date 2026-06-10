# Challenge 04: Strings — `String` vs `&str`

Learn the two main string types in Rust, the rules for converting between them, and the byte-vs-character traps that catch every newcomer.

## Goal

Python has one string type. Rust has two (well, three) — `String`, `&str`, and `&String` — and the choice between them shows up in every signature. This challenge teaches:

- When to use `String` (owned, mutable) vs `&str` (borrowed, immutable)
- How to convert: `to_string`, `to_owned`, `String::from`, `format!`
- Why `s[0]` doesn't compile — and what to do instead
- Why byte-slicing UTF-8 strings can panic at runtime
- Case-insensitive comparison with `eq_ignore_ascii_case`

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `s = "hello"` | `let s: &str = "hello";` |
| `s = str("hello")` | `let s = String::from("hello");` |
| `s.lower()` | `s.to_lowercase()` |
| `s += " world"` | `s.push_str(" world");` (for `String`) |
| `s + " world"` | `format!("{s} world")` or `s + " world"` |
| `s[0]` | `s.chars().next()` (NOT `s[0]`) |
| `int(s)` | `s.parse::<i32>()` |
| `a.lower() == b.lower()` | `a.eq_ignore_ascii_case(b)` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 10 sections covering the String / `&str` API |
| `skeleton.rs` | Hostname normalizer — fill in five TODOs |
| `debug.rs` | Buggy log-line redactor — find and fix 4 bugs |
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
make CH=04_strings concept
make CH=04_strings skeleton
make CH=04_strings debug
make CH=04_strings solution
```

## Skeleton Challenge: Hostname Normalizer

Five tasks chain together into a hostname-cleanup pipeline:

1. **`trim_whitespace`** — strip leading/trailing whitespace
2. **`strip_port`** — drop everything after the first colon
3. **`to_lower`** — lowercase the result
4. **`trim_trailing_dots`** — strip any number of `.` from the end
5. **`normalize_hostname`** — compose the four; return `Err` if the result is empty

## Debug Challenge: Log-Line Redactor

Four string-related bugs — some stop the program compiling, some misbehave
at runtime. Read the compiler errors and test failures, and work backwards
to the cause.

If you get stuck, [`HINTS.md`](HINTS.md) reveals each bug in stages: symptom,
then location, then the fix.

## Concepts Covered

1. `String` vs `&str` vs `&String` — when to use which
2. Three ways to make an owned `String` from a literal
3. Why functions should usually take `&str` parameters
4. `push_str`, `push`, `+`, `format!` for growing strings
5. `parse::<T>()` for type-safe conversion from text
6. `chars()` vs `bytes()` vs `as_bytes()` — and why `s[i]` is forbidden
7. Splitting (`split`, `split_once`, `split_whitespace`) and joining
8. `trim`, `trim_end_matches`, `strip_prefix`, `strip_suffix`
9. Returning `&str` from functions — when it's allowed and when it's not
10. When to clone — and when to borrow instead
