# 15 — RefCell runtime borrow panic

## Symptom

A single-threaded request panics with `already borrowed` when one registry method
re-enters the same cell during an update.

## Contract

Dynamic borrow checks must never fail on a valid call path. Nested method calls and
borrow scope are part of the design even without threads.

## Reproduce

Run `make ex N=15` with a backtrace. Note which borrow is still alive when the
callback attempts access.

## Task

Map runtime borrow lifetimes, including temporaries. Shorten the critical scope or
restructure the operation so a nested access occurs outside it.

## What you learn

You will distinguish compile-time borrowing from `RefCell`'s runtime rules and
recognize nested-access hazards.

Read [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) and the
[interior mutability chapter](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).
