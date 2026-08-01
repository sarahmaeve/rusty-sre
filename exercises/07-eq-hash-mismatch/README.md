# 07 — Equality and hashing disagree

## Symptom

A `HashSet` can retain two values that direct comparison reports as equal, and a
lookup may miss an apparently equal key.

## Contract

Values that compare equal must hash identically. Equality must also remain an
equivalence relation for the fields the type declares as identity.

## Reproduce

Run `make ex N=07`. Compare direct equality, hashes, insertion count, and lookup.

## Task

Write down the type's identity rule, then audit every manual or derived equality
and hashing implementation against it. Add a contract-focused regression test.

## What you learn

You will review trait laws rather than method bodies in isolation.

Read [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and rust-analyzer's
[interning implementation](https://github.com/rust-lang/rust-analyzer/tree/master/crates/intern).
