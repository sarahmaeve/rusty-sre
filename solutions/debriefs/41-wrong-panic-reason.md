# 41 — Validate before the accidental panic

Patch: [41-specific-panic.patch](../41-specific-panic.patch)

## Contract

Zero is rejected explicitly as `value must be positive` before arithmetic uses the
value as a divisor.

## Root cause

Division ran before the positivity assertion.

## Why the symptom follows

For zero, integer division panics immediately. Control never reaches the intended
validation, although a test that expects any panic still passes.

## Repair strategy

Move the precondition check before the first operation that relies on it. Keep the
test specific about the panic cause.

## Verification

Run `make ex N=41`. Also cover valid positive input and malformed text separately.

## Tempting wrong fix

Marking the test only `#[should_panic]` preserves the false positive by accepting
every unrelated panic in the function.

## References

The Rust Book on [`should_panic`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic)
and [`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html).
