# 48 — Build-time configuration shadows runtime input

## Symptom

A deployment supplies `canary` at runtime, but the service reports the value baked
into the binary—or `stable` when none was present during compilation.

## Contract

An explicit runtime deployment mode takes precedence. A compile-time value may be
a fallback, followed by the documented stable default.

## Reproduce

Run `make ex N=48`. Compare the function argument with the value selected by
`option_env!`.

## Task

Label each configuration source by evaluation phase. Restore explicit precedence
without turning compilation environment into an invisible runtime override.

## What you learn

You will distinguish compiler environment macros from runtime input and review
configuration provenance across Cargo builds.

Read [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html), Cargo's
[environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html),
and Cargo's [build-script contract](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
