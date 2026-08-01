# 41 — A panic test passes for the wrong reason

## Symptom

Zero input is rejected, but the panic reports division by zero instead of the
validation contract. A broad panic-only test would call this success.

## Contract

`require_positive` rejects zero at the validation boundary with the message
`value must be positive`. Later operations must not become accidental validators.

## Reproduce

Run `make ex N=41`. Read both the captured panic and the assertion that follows it.

## Task

Locate the first operation that assumes positivity. Make the intended precondition
fail before that operation and retain a test that distinguishes panic causes.

## What you learn

You will review `#[should_panic(expected = ...)]`, `catch_unwind`, and why merely
observing a panic is a weak behavioral contract.

Read the Rust Book's [panic test guidance](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic)
and the standard library's [`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html).
