# 47 — Wake after making a pending future ready

Patch: [47-wake-pending-future.patch](../47-wake-pending-future.patch)

## Contract

Every `Pending` return arranges a wake when another poll can make progress.

## Root cause

`YieldOnce` changed `yielded` to true but ignored the task's waker.

## Why the symptom follows

Executors poll ready tasks, not every pending future continuously. The private
state transition alone does not put the task back on an executor queue.

## Repair strategy

Notify the current waker after the transition and before returning `Pending`.

## Verification

Run `make ex N=47`, then await the future on an executor and ensure it completes
without a separate timer waking the task.

## Tempting wrong fix

Returning `Ready` on the first poll removes the intended yield semantics instead
of implementing the wake contract.

## References

[`Future::poll`](https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll),
[`Context`](https://doc.rust-lang.org/std/task/struct.Context.html), and
[`Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html).
