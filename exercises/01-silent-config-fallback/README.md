# 01 — Silent configuration fallback

## Symptom

The service starts after receiving a malformed value for a numeric setting. Its
startup summary shows the default, so the deployment appears healthy but behaves
differently from its configuration.

## Contract

An absent optional setting may use the documented default. A present but invalid
setting must be rejected with enough context to identify it.

## Reproduce

Run `make ex N=01` to reproduce the malformed case. Then inspect or run the nearby
unit tests for the absent and valid cases before changing the implementation.

## Task

Trace how presence, parsing, and defaulting are represented. Restore the contract
without changing the valid or absent cases, and add a regression assertion that
distinguishes absence from invalid input.

## What you learn

You will practice reading `Option<Result<T, E>>`, deciding where `transpose` is
useful, and spotting combinators that discard error information.

Read [Option](https://doc.rust-lang.org/std/option/enum.Option.html),
[Result](https://doc.rust-lang.org/std/result/enum.Result.html), and Cargo's
[environment handling](https://github.com/rust-lang/cargo/tree/master/crates/cargo-util/src).
