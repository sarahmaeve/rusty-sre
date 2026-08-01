# 17 — Poisoned Mutex unwrap

## Symptom

After one worker panics while holding shared state, later unrelated requests also
panic when attempting to acquire the lock.

## Contract

Poisoning is handled according to the state's invariants: recover deliberately,
rebuild, or return an error. A prior panic does not trigger an undocumented panic
cascade.

## Reproduce

Run `make ex N=17`. Trigger the first panic, then inspect the result of the next
lock acquisition.

## Task

Determine what may be inconsistent after unwinding and choose an explicit recovery
policy. Test both normal access and access after poisoning.

## What you learn

You will understand mutex poisoning, `LockResult`, and invariant recovery.

Read [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) and the standard
library's [poison implementation](https://github.com/rust-lang/rust/tree/master/library/std/src/sync).
