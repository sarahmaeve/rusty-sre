# 02 — Repeated errors are overwritten

## Symptom

The error summary reports one failure for a service even when that service appears
several times in the event stream.

## Contract

`count_errors` returns the occurrence count for each service name.

## Reproduce

Run `make ex N=02`. Compare counts for distinct names with counts for repeated and
interleaved names.

## Task

Trace one repeated key through the update loop. Repair the accumulation without
adding a second pass over the input.

## What you learn

You will read `HashMap` mutation code and use the entry API for accumulation.

Read [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
and ripgrep's [ignore-file collection code](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore).
