# 18 — Drop forgets to release a lease

## Symptom

The active-lease gauge increases on every acquisition but never returns to zero
after guards leave scope.

## Contract

Acquiring a `Lease` increments the shared counter exactly once. Dropping it
decrements the counter exactly once on every exit path.

## Reproduce

Run `make ex N=18`. Observe the counter before acquisition, while the guard is
alive, and after its scope ends.

## Task

Compare the state transition in `Lease::acquire` with the destructor. Restore the
missing inverse operation without requiring callers to release manually.

## What you learn

You will reason about RAII, destructor timing, atomics, and symmetric lifecycle
transitions.

Read [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) and
[`AtomicUsize::fetch_sub`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_sub).
