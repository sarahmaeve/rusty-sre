# 45 — Extend baseline behavior with the feature

Patch: [45-additive-feature.patch](../45-additive-feature.patch)

## Contract

Schema and bounds checks exist in every build. `audit` adds the audit check.

## Root cause

Mutually exclusive `cfg` blocks each returned a complete, different vector.

## Why the symptom follows

Compiling with `audit` removes the non-feature block before type checking. The
feature build therefore has no code that adds baseline checks.

## Repair strategy

Construct the common behavior unconditionally and conditionally extend it. The
binding shape remains warning-free with and without the feature.

## Verification

Run `make ex N=45`, normal tests, and Clippy under both default and all features.

## Tempting wrong fix

Copying baseline strings into every feature branch scales poorly and lets future
feature combinations drift again.

## References

Cargo [features](https://doc.rust-lang.org/cargo/reference/features.html), feature
[unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification),
and [`cfg`](https://doc.rust-lang.org/reference/conditional-compilation.html).
