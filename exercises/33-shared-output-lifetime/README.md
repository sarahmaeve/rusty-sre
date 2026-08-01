# 33 — Shared output lifetime

## Symptom

Compilation fails with `E0621`: the declared output lifetime can be satisfied by
one input branch but not the other.

## Contract

`choose_label` may return either borrowed input. Whichever value it selects remains
valid for every use of the returned reference.

## Reproduce

Run `make ex N=33`.

Read the signature, return type, failing branch, and compiler suggestion as a single
lifetime relationship.

## Task

Express which inputs can supply the output reference. Preserve borrowing and avoid
allocating a new label merely to bypass the diagnostic.

## What you learn

You will read explicit lifetime parameters as relationships rather than durations
and audit functions whose output may originate from multiple inputs.

Read [lifetime syntax](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html),
[`E0621`](https://doc.rust-lang.org/error_codes/E0621.html), and the Rust Reference
on [lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html).
