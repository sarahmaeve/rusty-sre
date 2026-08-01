# 29 — Reference to local data

## Symptom

Compilation fails with `E0515` because a function attempts to return a reference to
data created inside that function.

## Contract

Every returned reference points to storage that outlives the caller's use. Locally
created results transfer ownership instead of borrowing vanished stack state.

## Reproduce

Run `make ex N=29`. Identify the referent, its owner, and the scope where that owner
is dropped.

## Task

Decide whether the result should be owned or should borrow from an input. Make that
relationship explicit in the signature.

## What you learn

You will interpret lifetime errors and distinguish output ownership from input
borrowing.

Read [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) and
[`E0515`](https://doc.rust-lang.org/error_codes/E0515.html).
