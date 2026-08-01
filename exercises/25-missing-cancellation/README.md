# 25 — Missing cancellation

## Symptom

Cancelling an idle worker does not wake it; it remains blocked on an open, empty
job channel.

## Contract

The worker exits when its cancellation token fires or when every job sender closes.
It returns the work completed before termination.

## Reproduce

Run `make ex N=25`. The test cancels the token while keeping the sender open and
bounds the worker join with a timeout.

## Task

Make the receive loop observe both channel input and cancellation. Preserve normal
channel-closure behavior.

## What you learn

You will practice structured concurrency, cancellation safety, and shutdown design.

Read Tokio's [graceful shutdown guide](https://tokio.rs/tokio/topics/shutdown) and
Tokio's [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html).
