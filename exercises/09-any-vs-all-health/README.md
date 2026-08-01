# 09 — Any versus all health logic

## Symptom

A pool is reported healthy when one required member responds and another is down.

## Contract

The aggregate is healthy only when every required member is healthy. Empty-pool
behavior must be explicit rather than inherited accidentally from iteration.

## Reproduce

Run `make ex N=09`. Compare mixed, all-healthy, all-unhealthy, and empty inputs.

## Task

Translate the prose contract into a truth table, then make the implementation and
tests match it without special-casing only the observed mixed input.

## What you learn

You will review iterator predicates, short-circuiting, and vacuous truth.

Read [`Iterator::all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)
and [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any).
