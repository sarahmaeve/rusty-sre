# 35 — Dyn-compatible trait boundary

## Symptom

Compilation fails with `E0038`: `Reporter` cannot be used as `dyn Reporter` because
one method has a generic type parameter.

## Contract

Callers can dynamically dispatch the reporter name. Concrete reporter values also
retain their generic encoding convenience method.

## Reproduce

Run `make ex N=35`.

Use the diagnostic's vtable explanation to identify which method prevents trait
object construction.

## Task

Separate operations available through dynamic dispatch from operations that require
a concrete implementor. Preserve the generic method for concrete callers.

## What you learn

You will review dyn compatibility, vtable-dispatchable methods, and per-method
constraints on otherwise object-safe APIs.

Read the Reference on [dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility),
[`E0038`](https://doc.rust-lang.org/error_codes/E0038.html), and
[erased-serde](https://github.com/dtolnay/erased-serde).
