# Challenge 01a: Ownership & Borrowing with Vectors

Learn how Rust's ownership system, borrowing rules, and lifetimes interact with `Vec<T>` — the most common place where learners encounter the borrow checker.

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 21 tests covering moves, clones, shared/mutable borrows, invalidation, iteration ownership, lifetimes, drain/split_off/swap_remove, and common mistakes |
| `skeleton.rs` | SRE alert pipeline — fill in 6 TODOs exercising different ownership patterns |
| `debug.rs` | Buggy incident tracker — find and fix 4 ownership/borrowing bugs (all are compile errors) |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |
| `data/incidents.txt` | Sample incident data |

## How to Run

```bash
# Concept explainer — all tests should pass
rustc concept.rs --edition 2021 --test && ./concept

# Skeleton challenge — tests will fail until you complete the TODOs
rustc skeleton.rs --edition 2021 --test && ./skeleton

# Debug challenge — will NOT compile until you fix the bugs
rustc debug.rs --edition 2021 --test && ./debug

# Solution — all tests should pass
cd solution && rustc debug_solution.rs --edition 2021 --test && ./debug_solution
```

## Skeleton Challenge: SRE Alert Pipeline

Six tasks, each exercising a different ownership/borrowing pattern:

1. **Parse alerts** — consume a `Vec<String>` with `into_iter()`, return `Vec<Alert>`
2. **Filter by reference** — borrow `&[Alert]`, return `Vec<&Alert>` with lifetime annotation
3. **Mutate in place** — use `&mut Vec<Alert>` and `iter_mut()` to escalate warnings
4. **Extract a subset** — remove info alerts from the Vec and return them as owned
5. **Build summary** — format a report string using only `&` references, no cloning
6. **Split ownership** — divide a Vec into two owned halves with `split_off()`

## Debug Challenge: Incident Tracker

Four bugs, all compile-time errors from Rust's ownership/borrowing rules:

1. **Use after move** — passing a Vec to a function that takes ownership, then using it again
2. **Borrow conflict** — holding an immutable reference from `.find()` while calling `.iter_mut()`
3. **Dangling reference** — returning references into a local Vec that gets dropped
4. **Consumed in for loop** — `for item in vec` moves the Vec, preventing later use

The compiler error messages themselves are educational — read them carefully before fixing.
