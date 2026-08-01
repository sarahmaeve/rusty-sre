# 47 — A future returns Pending without arranging a wake

## Symptom

`YieldOnce` becomes ready internally after its first poll, but a normal executor
may never poll it again.

## Contract

When `poll` returns `Pending`, the future arranges for the current task's waker to
be notified after progress can be made.

## Reproduce

Run `make ex N=47`. The test supplies an instrumented waker and polls exactly once.

## Task

Trace the future's state transition and the executor's only notification channel.
Make the pending transition observable without busy-polling.

## What you learn

You will read `Future::poll`, `Context`, `Waker`, `Pin`, and the wake contract
behind async scheduling.

Read [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html),
[`Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html), and Tokio's
[task source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/runtime/task).
