# 16 — Rc reference cycle

## Symptom

Dropping all top-level handles does not run destructors for nodes in a relationship
graph, and memory remains retained.

## Contract

Ownership edges determine lifetime. Non-owning back-references must not keep the
graph alive after its owning root is dropped.

## Reproduce

Run `make ex N=16`. Observe strong and weak counts before and after linking nodes,
then check destructor evidence.

## Task

Draw the ownership graph and label which relationships own their targets. Break
cycles by making the domain's non-owning direction explicit.

## What you learn

You will reason about `Rc`, `Weak`, destructor reachability, and graph ownership.

Read [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
[`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html), and the
[reference-cycle chapter](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).
