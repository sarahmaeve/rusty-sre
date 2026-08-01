# 26 — Incomplete channel drain

## Symptom

Batch ingestion returns only the first envelope when the batch is larger than one.

## Contract

`ingest_batch` returns every input envelope in send order, independent of channel
capacity. Producer task failure is observed.

## Reproduce

Run `make ex N=26`. Compare input and output for a batch larger than the bounded
channel's capacity.

## Task

Trace producer and consumer lifecycles together. Repair the drain loop and observe
the producer's task outcome.

## What you learn

You will read bounded channel APIs, closure, draining, backpressure, and task joins.

Read Tokio's [channels tutorial](https://tokio.rs/tokio/tutorial/channels) and
[`mpsc::Sender::send`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html#method.send).
