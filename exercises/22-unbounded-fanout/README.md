# 22 — Unbounded fanout

## Symptom

A large input causes a sudden rise in tasks, memory, open connections, and latency;
small tests remain fast.

## Contract

Concurrency is bounded independently of input size. Backpressure prevents work
from being admitted faster than the system can complete it.

## Reproduce

Run `make ex N=22`. Measure peak in-flight operations for small and large inputs,
not just total completion time.

## Task

Locate the admission point, choose a meaningful capacity, and prove with an
instrumented test that the limit is never exceeded.

## What you learn

You will distinguish concurrency from parallelism and use bounded task/stream
patterns.

Read [`StreamExt::buffer_unordered`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.buffer_unordered)
and Tokio's [`Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html).
