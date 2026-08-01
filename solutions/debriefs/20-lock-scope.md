# 20 — Keep the lock out of the await

Patch: [20-lock-scope.patch](../20-lock-scope.patch)

## Contract

Independent target lookups overlap. The generations mutex protects only the final
map update, not simulated remote lookup latency.

## Root cause

`refresh_target` acquired the mutex before sleeping and retained its guard across
the `.await`.

## Why the symptom follows

Suspension preserves local variables. Every concurrent refresh waits for the first
guard, serializing unrelated lookups and making the probe peak at one.

## Repair strategy

Own the target name before suspension, perform the lookup without the map lock,
then acquire briefly to commit the generation. End instrumentation at the lookup
boundary.

## Verification

Run `make ex N=20`. Confirm a peak of two for distinct refreshes and verify both
generations are stored.

## Tempting wrong fix

Replacing Tokio's mutex with another mutex does not help; the defect is guard
scope, not the lock implementation.

## References

Tokio's [shared-state tutorial](https://tokio.rs/tokio/tutorial/shared-state),
[`MutexGuard`](https://docs.rs/tokio/latest/tokio/sync/struct.MutexGuard.html), and
Clippy's [`await_holding_lock`](https://rust-lang.github.io/rust-clippy/master/#await_holding_lock).
