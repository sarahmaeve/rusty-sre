# 15 — Avoid a RefCell reborrow

Patch: [15-refcell-reborrow.patch](../15-refcell-reborrow.patch)

## Contract

`rename` rejects an occupied destination, otherwise moves the existing owner from
the old name to the new name without a runtime borrow panic.

## Root cause

`rename` acquired `borrow_mut` and then called `contains`, which tried to acquire a
shared borrow of the same `RefCell` while the exclusive guard was live.

## Why the symptom follows

`RefCell` enforces aliasing dynamically. The nested shared borrow conflicts with
the existing mutable borrow and panics even on one thread.

## Repair strategy

Perform the destination check before taking the mutable guard, then use one mutable
borrow for removal and insertion.

## Verification

Run `make ex N=15`. Cover successful rename, occupied destination, and missing
source while confirming state is unchanged on rejection.

## Tempting wrong fix

Using `try_borrow` and treating its error as “destination exists” hides an internal
borrow conflict as a domain result.

## References

[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html),
[interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html),
and rustc's [`RefCell` source](https://github.com/rust-lang/rust/blob/master/library/core/src/cell.rs).
