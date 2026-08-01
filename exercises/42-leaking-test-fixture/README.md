# 42 — A panicking test fixture leaks state

## Symptom

After a test callback panics in maintenance mode, later code still observes
maintenance mode instead of the original value.

## Contract

`with_mode` restores the previous process state after normal return and unwinding.
Nested overrides work, and overrides from different threads do not overlap. Cleanup
does not depend on the callback reaching its final statement.

## Reproduce

Run `make ex N=42`. The test catches the simulated callback panic, then inspects
the shared mode.

## Task

Model temporary state as a resource with one owner. Move restoration to a mechanism
that runs on every scope exit, and define how concurrent owners are serialized.

## What you learn

You will apply RAII to test fixtures and distinguish unwind-safe cleanup from
linear setup/callback/teardown code.

Read [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html),
[`UnwindSafe`](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html), and
Cargo's [test support source](https://github.com/rust-lang/cargo/tree/master/crates/cargo-test-support).
