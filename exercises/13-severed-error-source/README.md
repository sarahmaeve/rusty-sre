# 13 — Severed error source

## Symptom

The displayed top-level error is useful, but error reporting and telemetry cannot
walk back to the underlying I/O failure.

## Contract

Wrapping adds operational context without discarding the machine-readable cause.
Callers can inspect the source chain while users see a concise message.

## Reproduce

Run `make ex N=13`. Compare formatted output with traversal through
`std::error::Error::source`.

## Task

Follow construction of the error value and compare the stored typed cause with what
the `Error` implementation exposes. Preserve context, ownership, and the source chain.

## What you learn

You will distinguish error display from error structure and review custom error
implementations.

Read [`Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
and Cargo's [error infrastructure](https://github.com/rust-lang/cargo/tree/master/crates/cargo-util/src).
