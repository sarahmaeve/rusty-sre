# 06 — Clone, then mutate

## Symptom

`mark_degraded` returns, but the caller's `Service` remains healthy.

## Contract

`mark_degraded(&mut service, reason)` changes the supplied service to
`Status::Degraded` and preserves the reason.

## Reproduce

Run `make ex N=06`. Inspect the service before and after the call, including the
stored reason.

## Task

Track the original value and its clone. Repair the state transition through the
mutable reference already supplied by the API.

## What you learn

You will reason about value identity, `Clone`, mutable references, and entry APIs.

Read [references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
and [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).
