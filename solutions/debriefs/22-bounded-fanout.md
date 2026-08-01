# 22 — Bound fanout

Patch: [22-bounded-fanout.patch](../22-bounded-fanout.patch)

## Contract

Every target is scraped, while active scrapes never exceed `max_in_flight`. This API
treats zero as a minimum capacity of one.

## Root cause

The parameter was unused and the function eagerly admitted one task per target.

## Why the symptom follows

Executor scheduling does not impose the domain's concurrency or admission budget.
Every spawned task consumes scheduler and memory resources even while waiting.

## Repair strategy

Admit at most `max_in_flight` tasks into a `JoinSet`. Each completed task creates
capacity for one more target, so both live tasks and active scrapes remain bounded.

## Verification

Run `make ex N=22`. Measure the peak below, at, and above the limit, confirm every
target result is returned, and test an input much larger than the limit.

## Tempting wrong fix

Chunking inputs into fixed batches adds barriers: one slow scrape delays the next
batch even when other capacity is idle.

## References

Tokio's [`Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html),
Futures' [`buffer_unordered`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.buffer_unordered),
and Vector's [source](https://github.com/vectordotdev/vector).
