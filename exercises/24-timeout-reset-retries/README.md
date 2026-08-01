# 24 — Timeout reset across retries

## Symptom

An operation documented with a one-second budget can run for several seconds when
each retry nearly reaches one second.

## Contract

One operation has one total deadline. Attempts consume the remaining budget rather
than receiving a fresh full budget.

## Reproduce

Run `make ex N=24`. Configure multiple slow failures and compare wall time with the
declared operation budget.

## Task

Record an absolute deadline at the ownership boundary, carry it through retries,
and test that total elapsed time remains bounded.

## What you learn

You will distinguish per-attempt timeout from end-to-end deadline.

Read [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html), Tokio's
[`timeout_at`](https://docs.rs/tokio/latest/tokio/time/fn.timeout_at.html), and
Tower's [timeout middleware](https://github.com/tower-rs/tower/tree/master/tower/src/timeout).
