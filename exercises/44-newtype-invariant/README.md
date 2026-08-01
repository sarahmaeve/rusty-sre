# 44 — An infallible conversion bypasses a newtype invariant

## Symptom

Configuration accepts port zero even though `Port` exists to exclude the reserved
value.

## Contract

Every constructible `Port` is nonzero. Conversion from an arbitrary `u16` is
fallible and returns `PortError` for zero.

## Reproduce

Run `make ex N=44`. Compare the private field with the public conversion paths.

## Task

Audit every constructor and trait implementation that can create the newtype.
Choose a conversion trait whose signature exposes the domain's failure.

## What you learn

You will distinguish type aliases from newtypes and `From` from `TryFrom` when
encoding invariants.

Read [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) and the
Rust API Guidelines' [conversion guidance](https://rust-lang.github.io/api-guidelines/interoperability.html#conversions-use-the-standard-traits-from-asref-asmut-c-conv-traits).
