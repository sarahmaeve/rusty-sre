# 12 — Retry attempt off by one

## Symptom

An operation configured with two retries stops after its second call, before the
third call that would succeed.

## Contract

`max_retries` counts calls after the initial attempt. Zero means one call and two
means at most three calls.

## Reproduce

Run `make ex N=12`. Count calls for zero retries and for success on the final
allowed attempt.

## Task

Write the attempt sequence on paper, audit range bounds and counter updates, then
make execution and diagnostics reflect the documented budget.

## What you learn

You will reason about retry terminology, inclusive ranges, and boundary tests.

Read Tokio's [source and design discussions](https://github.com/tokio-rs/tokio)
and [`Range`](https://doc.rust-lang.org/std/ops/struct.Range.html).
