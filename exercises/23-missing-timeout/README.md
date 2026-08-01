# 23 — Missing timeout

## Symptom

If a dependency accepts a request but never completes it, the caller and its
resource permit remain occupied indefinitely.

## Contract

Every external wait is bounded by a documented deadline or cancellation source,
and expiration is distinguishable from dependency failure.

## Reproduce

Run `make ex N=23`. Use a dependency stub that remains pending and an outer test
timeout so the suite itself cannot hang.

## Task

Identify the correct ownership boundary for the time budget. Surface expiry as a
typed outcome and verify cleanup after it fires.

## What you learn

You will apply deadlines to futures and reason about cancellation on drop.

Read Tokio's [`timeout`](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html) and
Hyper's [client source](https://github.com/hyperium/hyper/tree/master/src/client).
