# 38 — Match a complete path fragment

Patch: [38-macro-path-fragment.patch](../38-macro-path-fragment.patch)

## Contract

`is_state!` accepts an expression and a qualified enum-variant path such as
`State::Ready`, then expands to a pattern match.

## Root cause

The matcher declared `$expected:ident`, which consumes one identifier but not the
`::` and following segment.

## Why the symptom follows

Macro matching is syntactic and occurs before body expansion. After consuming
`State`, the rule expects the invocation to end and rejects the remaining tokens.

## Repair strategy

Use the `path` fragment specifier for the complete qualified variant. The fragment
remains structured and narrower than accepting arbitrary token trees.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/38-macro-path-fragment/src/main.rs`. Check both enum variants and a
longer module-qualified variant path.

## Tempting wrong fix

Changing the matcher to `$($expected:tt)*` accepts far more syntax, weakens error
locality, and can move failures into confusing expansion diagnostics.

## References

[macro metavariables](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables),
[macro expansion](https://rustc-dev-guide.rust-lang.org/macro-expansion.html), and
Tracing’s [macros](https://github.com/tokio-rs/tracing/tree/master/tracing/src/macros.rs).
