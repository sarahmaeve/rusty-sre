# 18 — Release a lease in Drop

Patch: [18-release-lease.patch](../18-release-lease.patch)

## Contract

Acquiring a `Lease` increments the shared active count exactly once; dropping that
lease decrements it exactly once on every exit path.

## Root cause

`Drop` merely loaded the counter. It never undid the increment performed by
`acquire`.

## Why the symptom follows

The guard leaves scope normally, but its destructor has no state transition, so
the global count permanently includes dead leases.

## Repair strategy

Use RAII symmetrically: decrement in `Drop`. The patch uses the same strong ordering
as acquisition and observation, keeping this teaching counter straightforward.

## Verification

Run `make ex N=18`. Cover nested leases, early return or unwind, and a final count
of zero.

## Tempting wrong fix

Requiring callers to invoke `release` manually misses early returns and panics—the
paths RAII is designed to cover.

## References

[`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html),
[`AtomicUsize::fetch_sub`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_sub),
and Tokio's [semaphore permit source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/sync).
