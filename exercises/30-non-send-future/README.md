# 30 — Future is not Send

## Symptom

Compilation rejects spawning a future because a value that cannot cross threads is
live across an `.await`.

## Contract

Futures submitted to a multi-threaded executor satisfy its `Send + 'static`
boundary, or are intentionally run on a local executor.

## Reproduce

Run `make ex N=30`. Read the diagnostic from the spawn bound through the value and
suspension point it identifies.

## Task

Determine whether the non-`Send` value can be scoped before suspension, replaced
with thread-safe ownership, or belongs on a local task set.

## What you learn

You will connect auto traits, generated future state, and executor requirements.

Read Tokio's [`spawn`](https://docs.rs/tokio/latest/tokio/fn.spawn.html),
[`LocalSet`](https://docs.rs/tokio/latest/tokio/task/struct.LocalSet.html), and
[`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html).
