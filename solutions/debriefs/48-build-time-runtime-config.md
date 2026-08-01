# 48 — Give runtime configuration precedence

Patch: [48-runtime-configuration.patch](../48-runtime-configuration.patch)

## Contract

Explicit runtime input wins, then an optional build-time fallback, then `stable`.

## Root cause

The function discarded its runtime argument and returned only `option_env!` or the
literal default.

## Why the symptom follows

`option_env!` expands during compilation. Its value cannot observe a deployment's
process environment or function argument after the binary exists.

## Repair strategy

Express the precedence chain over the runtime option first, followed by the
compile-time option and final default.

## Verification

Run `make ex N=48`. Cover explicit runtime input, compile-time fallback, and
complete absence in separately built binaries.

## Tempting wrong fix

Replacing `option_env!` with `env!` makes the build fail when the variable is absent
and still does not read runtime configuration.

## References

[`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html), Cargo
[environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html),
and [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
