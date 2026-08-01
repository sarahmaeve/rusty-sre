# 39 — Borrow through Pin

Patch: [39-pin-projection.patch](../39-pin-projection.patch)

## Contract

The `!Unpin` worker remains at its pinned address while callers read its name
through a shared reference.

## Root cause

`Pin::into_inner` attempted to remove the pin wrapper and return an owning pointer
whose pointee could subsequently move.

## Why the symptom follows

Safe extraction is available only when the pointee implements `Unpin`.
`PhantomPinned` deliberately suppresses that auto trait for `Worker`.

## Repair strategy

Reborrow the pin as `Pin<&Worker>`, then obtain `&Worker`. Shared access cannot move
the worker, so reading the unpinned `String` field needs no unsafe projection.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/39-pin-projection/src/main.rs`. Confirm the name prints and the worker
cannot still be safely extracted with `into_inner`.

## Tempting wrong fix

Adding a manual `impl Unpin for Worker` contradicts the type's declared pinning
invariant merely to unlock an extraction API.

## References

[`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html),
[`Unpin`](https://doc.rust-lang.org/std/marker/trait.Unpin.html), and Futures’
[pinning utilities](https://github.com/rust-lang/futures-rs/tree/master/futures-util/src).
