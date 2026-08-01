# 17 — Deliberate poison recovery

Patch: [17-poison-recovery.patch](../17-poison-recovery.patch)

## Contract

`snapshot` can inspect the last stored counts after a worker panic. This container's
values remain valid because updates do not expose a multi-step invariant.

## Root cause

`Mutex::lock().unwrap()` converted the poison marker from an earlier panic into a
second panic.

## Why the symptom follows

A poisoned mutex returns `Err(PoisonError)` on later lock attempts even though it
still supplies the guard. Unwrapping propagates panic rather than policy.

## Repair strategy

For this simple snapshot state, recover the guard with `PoisonError::into_inner`
and clone the stored values. More complex state would require validation or repair.

## Verification

Run `make ex N=17`. Snapshot normal state, poison after a known value, and confirm
the value remains inspectable.

## Tempting wrong fix

Calling `clear_poison` without first validating or recovering state only erases the
warning; it does not restore invariants.

## References

[`Mutex` poisoning](https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning),
[`PoisonError::into_inner`](https://doc.rust-lang.org/std/sync/struct.PoisonError.html#method.into_inner),
and Rust's [sync source](https://github.com/rust-lang/rust/tree/master/library/std/src/sync).
