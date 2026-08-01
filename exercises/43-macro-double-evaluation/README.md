# 43 — A macro evaluates an argument twice

## Symptom

A sampler is called twice when its first value falls below a cap. Pure closures
hide the defect, while stateful or expensive samplers expose it.

## Contract

`sample_with_cap` invokes its sampler exactly once and caps that one observation.

## Reproduce

Run `make ex N=43`. The test counts closure invocations independently of the
returned value.

## Task

Expand the macro by hand. Preserve expression behavior while ensuring each input
expression has one evaluation point and temporary names cannot capture caller code.

## What you learn

You will review macro expansion, expression fragments, evaluation count, blocks,
and macro hygiene.

Read the Rust Reference on
[`macro_rules!`](https://doc.rust-lang.org/reference/macros-by-example.html) and
the compiler's [macro expansion guide](https://rustc-dev-guide.rust-lang.org/macro-expansion.html).
