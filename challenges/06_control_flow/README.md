# Challenge 06: Control Flow as Expressions

Learn how `if`, `match`, `loop`, and braced blocks all produce values — and why that changes how you write Rust.

## Goal

In Python, `if` is a statement and you have to assign inside both branches. In Rust, almost everything is an expression: the whole `if`/`else` (or `match`, or `loop`) produces a value you can `let`-bind directly. This challenge teaches:

- `if`/`else` and `match` as expressions
- The trailing-semicolon trap: `expr` vs `expr;`
- Range and guard patterns inside `match`
- `loop` with `break value` to return a value from a loop
- `if let`, `while let`, and `let else`

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `x = a if cond else b` | `let x = if cond { a } else { b };` |
| `match`/`case` returning value | `let s = match x { Pat1 => a, Pat2 => b };` |
| `while True: ... break v` | `let v = loop { ... break val; };` |
| `if x is not None: use(x)` | `if let Some(v) = x { use(v) }` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 9 sections covering expressions vs statements, match patterns, loops |
| `skeleton.rs` | HTTP status classifier — fill in five TODOs |
| `debug.rs` | Buggy response analyzer — find and fix 4 bugs |
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
make CH=06_control_flow concept
make CH=06_control_flow skeleton
make CH=06_control_flow debug
make CH=06_control_flow solution
```

## Skeleton Challenge: HTTP Status Classifier

Five tasks build helpers for analyzing HTTP responses:

1. **`classify`** — `match` with range patterns, returns a `Category` enum
2. **`is_retriable`** — `match` with alternation (`408 | 429`) and a range
3. **`short_label`** — composition; `match` returning `&'static str`
4. **`parse_status`** — `let else` for parse step, then range-check the value
5. **`first_error_code`** — `Iterator::find` returning `Option<u16>`

## Debug Challenge: Response Analyzer

Four bugs:

1. **`if`/`else` arms have different types** (compile error) — middle branch returns the integer `200` while the other branches return `&str`. Fix: return `"success"`.
2. **`match` arm returns the wrong type** (compile error) — the `"low"` arm returns `"low"` (&str) while the function signature is `u8`. Fix: return `4`.
3. **Exclusive vs inclusive range** (runtime) — `is_success` uses `200..299` which excludes 299. HTTP success codes go through 299 inclusive. Fix: `200..=299`.
4. **Discarded expression instead of mutation** (runtime) — `count_failures` writes `count + 1;` as a statement, which computes the new value and throws it away. Fix: `count += 1;`.

## Concepts Covered

1. `if`/`else` as an expression — both arms must agree on type
2. The trailing-semicolon trap: `expr` returns a value, `expr;` discards it
3. `match` as an expression with literal, range, alternation, and guard patterns
4. Inclusive ranges (`a..=b`) vs exclusive ranges (`a..b`)
5. `for`, `while`, and `loop` — and `loop` with `break value`
6. `if let` and `while let` for single-pattern matching
7. `let else` for early return on the negative pattern
8. Bare braced blocks `{ ... }` as expressions for inline scoping
9. Type unification across all arms — what the compiler is doing for you
