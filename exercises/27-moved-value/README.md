# 27 — Moved value

## Symptom

Compilation fails with `E0382`: the log vector is used after being passed by value
to `summarize`.

## Contract

`summarize` reads log lines without consuming them, and the caller may still read
the vector afterward without cloning its strings.

## Reproduce

Run `make ex N=27`. Read the primary diagnostic, labeled move, later use, and any
suggestion separately.

## Task

Compare what `summarize` does with what its parameter type permits. Adjust the
boundary to express read-only temporary access.

## What you learn

You will read move diagnostics and choose APIs from lifetime requirements.

Read [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) and
[`E0382`](https://doc.rust-lang.org/error_codes/E0382.html).
