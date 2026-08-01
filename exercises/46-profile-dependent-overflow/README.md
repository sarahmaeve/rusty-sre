# 46 — Correctness depends on the build profile

## Symptom

An exhausted generation panics in a debug build but wraps in an optimized build.
Neither behavior matches the fallible return type.

## Contract

`next_generation(u32::MAX)` returns `None` in every build profile. Representable
values return their successor.

## Reproduce

Run `make ex N=46`, then run the focused test with `cargo test --release`. Compare
the failure mechanism, not only whether the test fails.

## Task

Separate diagnostic assertions from input validation. Use an arithmetic operation
whose return type expresses overflow consistently.

## What you learn

You will review debug assertions, integer overflow, Cargo profiles, and why release
behavior belongs in correctness testing.

Read the Reference on [overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow)
and Cargo's [profile settings](https://doc.rust-lang.org/cargo/reference/profiles.html).
