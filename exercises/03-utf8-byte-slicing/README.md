# 03 — UTF-8 byte slicing

## Symptom

Generating a short label succeeds for ASCII input but panics for some valid
Unicode names.

## Contract

Every valid UTF-8 string is accepted. Truncation must define whether its limit is
bytes, Unicode scalar values, or user-perceived graphemes, and must never panic.

## Reproduce

Run `make ex N=03`. Observe which inputs fail and compare their byte length with
their visible character count.

## Task

Identify the intended unit, preserve valid UTF-8, and cover input both shorter and
longer than the limit.

## What you learn

You will distinguish byte indices from character iteration and recognize when
grapheme-aware behavior needs a dedicated crate.

Read [`str`](https://doc.rust-lang.org/std/primitive.str.html) and Servo's
[string-handling source](https://github.com/servo/servo/tree/main/components).
