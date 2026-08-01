# 19 — Blocking sleep in async code

## Symptom

Many nominally concurrent requests complete in batches or almost serially, and
runtime worker threads show long idle-looking gaps.

## Contract

Waiting inside an async task yields the runtime thread. Blocking operations are
isolated on an appropriate blocking pool when they cannot be made asynchronous.

## Reproduce

Run `make ex N=19`. Compare elapsed time for one task and several concurrent tasks.

## Task

Classify every wait as synchronous blocking, asynchronous suspension, or CPU work.
Change the inappropriate boundary and retain a timing test with generous tolerance.

## What you learn

You will understand cooperative scheduling and why `async fn` does not make its
body non-blocking.

Read Tokio's [bridging guide](https://tokio.rs/tokio/topics/bridging) and
[`spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html).
