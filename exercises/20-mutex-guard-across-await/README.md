# 20 — Mutex guard across await

## Symptom

Two independent target refreshes run serially even though their simulated remote
lookups could overlap.

## Contract

The generations mutex protects only the map update. Independent lookup latency is
outside its critical section.

## Reproduce

Run `make ex N=20`. The probe records peak concurrent lookups; compare its value
with the two refreshes started by `tokio::join!`.

## Task

Mark the mutex guard's lifetime across suspension. Move only the remote lookup out
of the critical section, then acquire briefly to commit the generation.

## What you learn

You will review lexical guard lifetimes, async suspension, and lock scope.

Read Tokio's [shared-state tutorial](https://tokio.rs/tokio/tutorial/shared-state)
and [`MutexGuard`](https://docs.rs/tokio/latest/tokio/sync/struct.MutexGuard.html).
