# Challenge 05: `Option<T>` and Nullability

Learn how Rust forces missing values to be explicit at the type level.

## Goal

In Python, any variable can be `None`, and you find out the hard way. In Rust, optionality is a different type — `Option<T>` — and the compiler refuses to let you treat it as `T` until you handle the `None` case.

This challenge teaches:

- `Some(T)` and `None`, and the API around them
- Combinators (`map`, `and_then`, `or_else`, `filter`, `unwrap_or*`)
- The `?` operator on `Option` (works in functions returning `Option`)
- Converting between `Option` and `Result` (`ok_or`, `Result::ok`)
- Why `unwrap_or(default)` can be dangerous: choose defaults that fail loudly
- The subtle difference between `Option<&T>` and `&Option<T>`

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `x = None` | `let x: Option<T> = None;` |
| `if x is None:` | `if x.is_none() { ... }` |
| `if x is not None: use(x)` | `if let Some(v) = x { use(v) }` |
| `x or default` | `x.unwrap_or(default)` |
| `x or compute()` | `x.unwrap_or_else(compute)` |
| `x.field if x else default` | `x.map(\|v\| v.field).unwrap_or(default)` |
| `raise KeyError if x is None` | `x.ok_or(MyError::Missing)?` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | Explainer with 10 sections covering every common Option pattern |
| `skeleton.rs` | Service status lookup — fill in five TODOs |
| `debug.rs` | Buggy on-call dashboard — find and fix 4 bugs |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |

## How to Run

```bash
rustc concept.rs --edition 2024 --test && ./concept
rustc skeleton.rs --edition 2024 --test && ./skeleton
rustc debug.rs --edition 2024 --test && ./debug
cd solution && rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

Or, from the repo root:

```bash
make CH=05_option concept
make CH=05_option skeleton
make CH=05_option debug
make CH=05_option solution
```

## Skeleton Challenge: Service Status Lookup

Five tasks build helpers for an on-call dashboard:

1. **`find_owner`** — `HashMap` lookup returning `Option<&str>`
2. **`parse_optional_severity`** — empty/whitespace → `None`, valid `1..=5` → `Some(n)`, garbage → `None`
3. **`first_critical`** — find the first alert with severity ≥ 4
4. **`owner_team`** — chain two `HashMap` lookups with `and_then`
5. **`summary_or_default`** — `Option<Stats>` → tuple, with SRE-conservative defaults

## Debug Challenge: On-Call Dashboard

Four bugs that all stem from misusing `Option`:

1. **Return-type mismatch** (compile error) — `lookup_owner` claims to return `&str` but produces `Option<&str>`
2. **`?` outside an `Option`-returning function** (compile error) — `first_severity` uses `?` while declared to return `u8`
3. **Panic on missing key** (runtime) — `owner_or_unknown` calls `.unwrap()` and crashes for unknown services
4. **Unsafe default** (runtime) — `worst_severity` uses `unwrap_or(0)`, which masks missing severities as "fine" when SRE convention says treat them as worst-case (5)

## Concepts Covered

1. `Option<T>` — the Some/None enum
2. `if let Some(v)` and `let else` for early-return on None
3. `unwrap`, `expect`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`
4. Combinators: `map`, `and_then`, `or`, `or_else`, `filter`
5. The `?` operator on `Option`
6. `ok_or` / `ok_or_else` to convert `Option` into `Result`
7. `take`, `replace`, `get_or_insert_with` — in-place mutation
8. Iterator methods that return `Option`: `find`, `max`, `min`, `position`
9. `Option<&T>` vs `&Option<T>` — using `as_ref` and `as_deref`
10. SRE pattern: choose defaults that fail loudly, not silently
