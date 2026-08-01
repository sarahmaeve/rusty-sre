# 43 — Evaluate each macro argument once

Patch: [43-single-evaluation-macro.patch](../43-single-evaluation-macro.patch)

## Contract

The sampler expression runs exactly once; its one value is compared and returned
or capped.

## Root cause

The macro substituted `$sample` into both the condition and one branch.

## Why the symptom follows

Macro expansion copies tokens. When the first comparison selects the sample branch,
the closure call appears and executes a second time.

## Repair strategy

Expand to a block that binds each input expression once, then performs the
conditional on those hygienic temporary bindings.

## Verification

Run `make ex N=43`. Cover values below, equal to, and above the cap while counting
calls.

## Tempting wrong fix

Documenting that arguments must be pure exports a surprising macro expansion
detail and still doubles expensive work.

## References

The Reference on [macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
and the rustc guide to [macro expansion](https://rustc-dev-guide.rust-lang.org/macro-expansion.html).
