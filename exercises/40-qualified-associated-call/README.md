# 40 — Qualified associated call

## Symptom

Compilation fails with `E0790`: an associated trait function is called without
naming the implementation that supplies it.

## Contract

The command prints the label defined by `Compact`'s implementation of `Format`.

## Reproduce

Run `make ex N=40`.

Notice that `label` has no receiver from which Rust could infer an implementor.

## Task

Make both the concrete type and trait unambiguous at the call site. Retain the
associated function rather than adding a dummy `self` parameter.

## What you learn

You will use fully qualified syntax for associated items and interpret ambiguity
where method receiver inference is unavailable.

Read [`E0790`](https://doc.rust-lang.org/error_codes/E0790.html),
[fully qualified syntax](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-fully-qualified-syntax-to-disambiguate-methods-with-the-same-name),
and Tower’s [`Layer` source](https://github.com/tower-rs/tower/blob/master/tower-layer/src/lib.rs).
