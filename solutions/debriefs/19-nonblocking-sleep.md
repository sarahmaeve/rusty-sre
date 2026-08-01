# 19 — Nonblocking async delay

Patch: [19-nonblocking-sleep.patch](../19-nonblocking-sleep.patch)

## Contract

While a batch waits for its settle delay, other ready futures on the runtime can
make progress.

## Root cause

The body of an `async fn` called `std::thread::sleep`.

## Why the symptom follows

An async function executes synchronously between suspension points. Thread sleep
parks the executor worker and provides no opportunity to poll the heartbeat future.

## Repair strategy

Await the runtime timer, which yields `Pending` until the deadline and releases the
worker to poll other tasks.

## Verification

Run `make ex N=19`. Confirm a short heartbeat runs during a longer flush and avoid
overly narrow wall-clock thresholds.

## Tempting wrong fix

Adding more runtime worker threads masks starvation, wastes capacity, and still
fails on a current-thread runtime.

## References

Tokio's [`sleep`](https://docs.rs/tokio/latest/tokio/time/fn.sleep.html),
[bridging synchronous code](https://tokio.rs/tokio/topics/bridging), and Tokio's
[runtime source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/runtime).
