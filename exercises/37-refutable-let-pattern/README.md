# 37 — Refutable let pattern

## Symptom

Compilation fails with `E0005`: a plain `let` binding does not cover the
`Event::Shutdown` variant.

## Contract

A sample event prints its value. A shutdown event ends the command without trying
to invent a sample.

## Reproduce

Run `make ex N=37`.

Treat the compiler's uncovered variant as a missing control-flow decision.

## Task

Handle the non-matching event explicitly while retaining direct access to the sample
payload on the success path.

## What you learn

You will distinguish irrefutable bindings from refutable patterns and choose among
`let else`, `if let`, and `match` by control-flow shape.

Read [refutability](https://doc.rust-lang.org/book/ch19-02-refutability.html),
[`let` statements](https://doc.rust-lang.org/reference/statements.html#let-statements),
and [`E0005`](https://doc.rust-lang.org/error_codes/E0005.html).
