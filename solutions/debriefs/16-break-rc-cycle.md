# 16 — Break the Rc ownership cycle

Patch: [16-break-rc-cycle.patch](../16-break-rc-cycle.patch)

## Contract

Parents own children. A child's parent link is navigational and must not keep the
parent graph alive after external owners are dropped.

## Root cause

Both parent-to-child and child-to-parent edges used strong `Rc` references.

## Why the symptom follows

Dropping external handles leaves a cycle in which every node has a nonzero strong
count. Reference counting cannot discover that the cycle is otherwise unreachable.

## Repair strategy

Represent the non-owning parent edge as `Weak` and downgrade the parent during
attachment. Callers upgrade only when they need a live parent.

## Verification

Run `make ex N=16`. Check strong/weak counts, parent navigation while alive, and
failed upgrade after the root leaves scope.

## Tempting wrong fix

Manually clearing links before every drop relies on all exit paths remembering
cleanup and defeats ownership-driven lifetime management.

## References

[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
[`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html), and
[reference cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).
