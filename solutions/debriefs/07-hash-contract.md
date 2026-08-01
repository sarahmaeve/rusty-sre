# 07 — Equality and hash contract

Patch: [07-hash-contract.patch](../07-hash-contract.patch)

## Contract

Incidents are equal when `service` and `summary` match. Equal incidents must always
produce equal hashes.

## Root cause

`PartialEq` ignored `id`, while `Hash` included it.

## Why the symptom follows

A hash collection chooses a candidate bucket before checking equality. Two equal
incidents with different IDs can enter different buckets and coexist.

## Repair strategy

Hash exactly the identity fields used by equality. The patch retains the declared
domain identity rather than changing equality to fit hashing.

## Verification

Run `make ex N=07`. Check equal values with different IDs, unequal summaries, and
both set insertion and lookup.

## Tempting wrong fix

Removing the manual equality implementation and comparing every field changes the
type's incident-deduplication semantics.

## References

[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html),
[`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html), and rust-analyzer's
[interning crate](https://github.com/rust-lang/rust-analyzer/tree/master/crates/intern).
