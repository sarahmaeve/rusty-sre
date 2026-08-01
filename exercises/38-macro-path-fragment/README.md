# 38 — Macro path fragment

## Symptom

Macro expansion never begins: the matcher reports that no rule expects `::` in
`State::Ready`.

## Contract

`is_state!` accepts a value expression and a qualified enum-variant path, then
reports whether the value matches that variant.

## Reproduce

Run `make ex N=38`.

Read the invocation and the highlighted metavariable matcher before inspecting the
expanded body.

## Task

Choose a macro fragment kind that describes the complete accepted syntax. Keep the
matcher narrow enough to reject unrelated token sequences.

## What you learn

You will distinguish matcher failures from expansion failures and interpret
`macro_rules!` fragment specifiers.

Read the Reference on [metavariables](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables),
[macro diagnostics](https://rustc-dev-guide.rust-lang.org/macro-expansion.html), and
Tracing’s [macro source](https://github.com/tokio-rs/tracing/tree/master/tracing/src/macros.rs).
