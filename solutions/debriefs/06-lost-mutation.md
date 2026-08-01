# 06 — Mutation lost in a clone

Patch: [06-lost-mutation.patch](../06-lost-mutation.patch)

## Contract

`mark_degraded(&mut service, reason)` changes the caller's service to
`Status::Degraded` with an owned copy of the reason.

## Root cause

The function cloned `service`, mutated the clone, and dropped it. The mutable input
was never written.

## Why the symptom follows

`Clone` creates an independent `Service`; it is not an alias. Local assertions about
that copy cannot make the caller's value change.

## Repair strategy

Assign the new status directly through the existing mutable reference. Only the
borrowed reason needs conversion to owned storage.

## Verification

Run `make ex N=06`. Assert both the variant and stored reason after the call.

## Tempting wrong fix

Returning the clone would compile, but changes a clear in-place API and makes it
easy for callers to ignore the returned update.

## References

[mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html),
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), and rust-analyzer's
[source](https://github.com/rust-lang/rust-analyzer).
