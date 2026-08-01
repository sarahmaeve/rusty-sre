# 22 — Bound fanout

Patch: [22-bounded-fanout.patch](../22-bounded-fanout.patch)

## Contract

Every target is scraped, while active scrapes never exceed `max_in_flight`. This API
treats zero as a minimum capacity of one.

## Root cause

The parameter was unused and the function spawned one immediately active scrape per
target.

## Why the symptom follows

Executor scheduling does not impose the domain's concurrency budget. All spawned
tasks can enter the measured scrape region together.

## Repair strategy

Share a semaphore, acquire an owned permit before entering the scrape, and let the
permit's destructor release capacity. Joining still preserves all results.

## Verification

Run `make ex N=22`. Measure the peak below, at, and above the limit and confirm all
target results are returned.

## Tempting wrong fix

Chunking inputs into fixed batches adds barriers: one slow scrape delays the next
batch even when other capacity is idle.

## References

Tokio's [`Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html),
Futures' [`buffer_unordered`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.buffer_unordered),
and Vector's [source](https://github.com/vectordotdev/vector).
