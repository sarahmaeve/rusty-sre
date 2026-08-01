# 28 — Shorten the shared borrow

Patch: [28-shorten-borrow.patch](../28-shorten-borrow.patch)

## Contract

The program adds `api-2` and then reports the first API host from the final vector.

## Root cause

It saved a reference into `hosts`, mutated the vector with `push`, and used the
saved reference afterward.

## Why the symptom follows

`push` requires exclusive access and may reallocate the vector, invalidating element
references. The later print keeps the shared borrow live across that mutation.

## Repair strategy

Perform the mutation first and create the reference afterward, making the access
periods disjoint and matching the final-state query.

## Verification

Apply the patch, then run `cargo run --manifest-path compile-fail/28-conflicting-borrows/Cargo.toml`.
Confirm there is no `E0502`.

## Tempting wrong fix

Cloning the selected host before mutation compiles but changes the query to a stale
snapshot and allocates unnecessarily.

## References

[`E0502`](https://doc.rust-lang.org/error_codes/E0502.html),
[`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push), and
[references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).
