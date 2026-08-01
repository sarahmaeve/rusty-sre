# 04 — Lossy integer cast

## Symptom

Large non-negative counters become negative or wrap to small values when encoded
for a downstream interface.

## Contract

Values outside the destination representation must produce an explicit error.
In-range values, including both boundaries, must round-trip.

## Reproduce

Run `make ex N=04`. Pay attention to the first value beyond the destination type's
maximum and to behavior under release optimizations.

## Task

Find the representation boundary, make failure visible to callers, and test exact
boundary values rather than only ordinary inputs.

## What you learn

You will distinguish `as` casts from checked conversions and see why release-mode
behavior belongs in numeric debugging.

Read [numeric casts](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast)
and [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html).
