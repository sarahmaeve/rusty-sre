# 34 — Spawned value lifetime bound

## Symptom

Compilation fails with `E0310`: generic `T` may not live long enough for the thread
started by `spawn_drop`.

## Contract

The returned thread handle owns all state needed by its task. No value captured by
the spawned closure borrows storage that can disappear while the thread runs.

## Reproduce

Run `make ex N=34`.

Trace the requirement from `thread::spawn` back to the helper's public generic
bound.

## Task

Make the helper state the lifetime capability its implementation requires. Do not
confuse `Send`—permission to cross threads—with validity for the thread's lifetime.

## What you learn

You will interpret `'static` as a type bound, review generic wrappers around spawn,
and separate transfer safety from captured-borrow lifetime.

Read [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html),
[`E0310`](https://doc.rust-lang.org/error_codes/E0310.html), and Rayon’s
[scoped-thread source](https://github.com/rayon-rs/rayon/tree/main/rayon-core/src).
