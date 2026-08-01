# 39 — Read through Pin without moving

## Symptom

Compilation fails with `E0277`: `Pin::into_inner` requires `Worker: Unpin`, but
`Worker` contains `PhantomPinned`.

## Contract

Once boxed and pinned, the worker is not moved. Its name remains readable through a
shared reference.

## Reproduce

Run `make ex N=39`.

Follow the diagnostic from `into_inner` to its `Unpin` bound and the field that
suppresses the auto trait.

## Task

Access the unpinned field without extracting or moving the pinned `Worker`. Do not
use unsafe code; shared projection is sufficient.

## What you learn

You will separate moving a pinned value from borrowing it and recognize `Unpin` as
the capability required by safe extraction APIs.

Read [`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html), the
[`pin` module](https://doc.rust-lang.org/std/pin/), and Futures’
[pinning source](https://github.com/rust-lang/futures-rs/tree/master/futures-util/src).
