# 26 — Drain the ingestion channel

Patch: [26-drain-channel.patch](../26-drain-channel.patch)

## Contract

`ingest_batch` returns every input envelope in send order, independent of channel
capacity. Producer task failure is not silently ignored.

## Root cause

The receiver read one event, dropped, and closed the channel. Remaining sends failed
and their results were discarded.

## Why the symptom follows

A bounded producer suspends when capacity is full. Once the receiver is dropped,
pending and later sends return errors; only the first received value reaches output.

## Repair strategy

Receive until all senders are dropped, let the producer finish its loop, stop if
delivery becomes impossible, and join the producer to observe task panic.

## Verification

Run `make ex N=26`. Cover batches smaller than, equal to, and larger than capacity,
including empty input.

## Tempting wrong fix

Setting capacity to the input length hides the early receiver exit for this call but
removes backpressure and retains the broken protocol.

## References

Tokio's [`mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/),
[`Receiver::recv`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv),
and Tokio's [channels tutorial](https://tokio.rs/tokio/tutorial/channels).
