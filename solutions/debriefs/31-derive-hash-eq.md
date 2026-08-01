# 31 — Supply HashSet's key contract

Patch: [31-derive-hash-eq.patch](../31-derive-hash-eq.patch)

## Contract

Two `Host` values with the same name represent the same set member, and `Host` can
therefore be inserted into `HashSet`.

## Root cause

`Host` derived only `Debug`; it supplied neither equality nor hashing.

## Why the symptom follows

`HashSet::from` needs `Eq + Hash` to select a bucket and determine whether a value
already exists. Without those traits, collection construction cannot type-check.

## Repair strategy

Derive `PartialEq`, `Eq`, and `Hash` together. With one identity field, deriving all
three keeps their field selection consistent.

## Verification

Apply the patch, run `cargo run --manifest-path compile-fail/31-missing-hash-eq/Cargo.toml`,
and additionally check that duplicate names collapse in a set.

## Tempting wrong fix

Implementing only `Hash`, or hashing fields that equality ignores, either still
fails to compile or violates the collection's contract.

## References

[`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html),
[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html), and the
[derivable traits appendix](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).
