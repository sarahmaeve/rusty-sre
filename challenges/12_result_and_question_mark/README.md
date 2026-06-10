# Challenge 12: `Result` and the `?` Operator

Learn how Rust handles fallibility with explicit return values instead of exceptions.

## Goal

Earlier challenges used `Result` and `?` here and there without explaining either head-on. This challenge teaches:

- The `Result<T, E>` enum and its core combinators
- The `?` operator for early-return-on-error
- Building custom error enums (the manual style behind `thiserror`)
- `From` conversions that let `?` cross error-type boundaries
- When to use `unwrap`, `expect`, `unwrap_or`, and friends — and when not to

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `raise ValueError("bad")` | `return Err(MyErr::Bad)` |
| `try: ... except X as e: raise Y(e)` | `impl From<X> for Y`, then `?` |
| `except X: pass` | `.ok()` (drops the error) |
| `except X: return default` | `.unwrap_or(default)` |
| `for x in xs: validate(x)` | `xs.iter().map(validate).collect::<Result<Vec<_>, _>>()` |
| `sys.exit(1)` from a CLI | `return Err(...)` from `main` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | Heavily commented explainer covering Result, `?`, combinators, custom error enums |
| `skeleton.rs` | Config loader pipeline — fill in the TODOs to complete it |
| `debug.rs` | Buggy alert ingestion pipeline — find and fix 4 bugs |
| `HINTS.md` | Progressive hints for the skeleton tasks and the debug bugs |
| `solution/skeleton_solution.rs` | Reference implementation of `skeleton.rs` |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |

## How to Run

```bash
# Concept explainer — all tests pass
rustc concept.rs --edition 2024 --test && ./concept

# Skeleton challenge — tests fail until you complete the TODOs
rustc skeleton.rs --edition 2024 --test && ./skeleton

# Debug challenge — won't compile until you fix the bugs
rustc debug.rs --edition 2024 --test && ./debug

# Reference solutions
cd solution && rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
cd solution && rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

Or, from the repo root with the Makefile:

```bash
make CH=12_result_and_question_mark concept
make CH=12_result_and_question_mark skeleton
make CH=12_result_and_question_mark debug
make CH=12_result_and_question_mark solution
```

## Skeleton Challenge: Config Loader

Six tasks build a startup-time config loader:

1. **Define `ConfigError`** — enum with four variants (`InvalidPort`, `InvalidThreshold`, `InvalidUrl`, `MissingField`)
2. **Implement `Display` and `Error`** for `ConfigError`
3. **`From<ParseIntError>`** so `?` can promote parse errors automatically
4. **`parse_port`**, **`parse_threshold`**, **`parse_url`** — three validators
5. **`load_config`** — combine the validators using `?` and `Option::ok_or`

## Debug Challenge: Alert Ingestion

Four bugs — some stop the program compiling, some misbehave at runtime. Read
the compiler errors and test failures, and work backwards to the cause.

If you get stuck, [`HINTS.md`](HINTS.md) reveals each bug in stages: symptom,
then location, then the fix.

## Concepts Covered

1. `Result<T, E>` — the success/failure enum
2. The `?` operator — desugaring and `From::from` conversion
3. `map`, `map_err`, `and_then`, `or_else`, `ok`, `err` — Result combinators
4. `unwrap`, `expect`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`
5. Implementing `Display` and `Error` for a custom enum
6. `From<X> for Y` to enable `?` across error boundaries
7. `Result<()> ` from `main` for proper exit codes
8. `Option` ↔ `Result` interconversion (`ok_or`, `ok`, `err`)
9. `collect::<Result<Vec<T>, E>>()` to short-circuit on first error
10. The retry-with-attempts SRE pattern as a generic function over closures
