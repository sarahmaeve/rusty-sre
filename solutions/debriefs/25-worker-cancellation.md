# 25 — Observe worker cancellation

Patch: [25-worker-cancellation.patch](../25-worker-cancellation.patch)

## Contract

The worker exits when either shutdown is cancelled or every job sender closes. It
returns the jobs completed before termination.

## Root cause

The cancellation token was ignored; the loop waited only on `jobs.recv()`.

## Why the symptom follows

An open, empty channel keeps `recv` pending. Cancelling a separate token cannot wake
a future that is never polled as part of the worker's control flow.

## Repair strategy

Use `tokio::select!` to wait on cancellation and channel input concurrently, ending
the loop on either cancellation or receiver closure.

## Verification

Run `make ex N=25`. Cover cancellation with an open empty channel, channel closure,
and processing a job before shutdown.

## Tempting wrong fix

Dropping one sender during shutdown is insufficient when other sender clones remain
alive and obscures the explicit cancellation contract.

## References

Tokio's [`select!`](https://docs.rs/tokio/latest/tokio/macro.select.html),
[`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html),
and Tokio's [shutdown guide](https://tokio.rs/tokio/topics/shutdown).
