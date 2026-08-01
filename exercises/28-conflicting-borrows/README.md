# 28 — Conflicting borrows

## Symptom

Compilation fails with `E0502`: a value is mutably borrowed while a shared borrow
is still used later.

## Contract

Mutation cannot invalidate an outstanding reference. The implementation scopes
reads and writes so their actual uses do not overlap.

## Reproduce

Run `make ex N=28`. Use the diagnostic labels to locate borrow creation, mutation,
and the final use that extends the borrow.

## Task

Reorder data extraction and mutation or change the API boundary so the necessary
access periods are disjoint without concealing the conflict behind cloning.

## What you learn

You will read non-lexical lifetime diagnostics and reason from last use.

Read [references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
and [`E0502`](https://doc.rust-lang.org/error_codes/E0502.html).
