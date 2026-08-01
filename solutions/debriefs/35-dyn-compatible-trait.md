# 35 — Exclude a generic method from dynamic dispatch

Patch: [35-dyn-compatible-trait.patch](../35-dyn-compatible-trait.patch)

## Contract

`name` is callable through `dyn Reporter`; concrete reporters retain the generic
`encode<T>` convenience method.

## Root cause

Every trait method was considered part of the object interface, but a generic
method has an open-ended set of monomorphizations and cannot occupy one vtable slot.

## Why the symptom follows

Rust cannot construct metadata for `dyn Reporter` while the trait requires dynamic
dispatch of `encode` for any possible `T`.

## Repair strategy

Constrain `encode` with `Self: Sized`. Trait objects are unsized, so the method is
excluded from their dispatch surface while remaining available to concrete types.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/35-dyn-compatible-trait/src/main.rs`. Confirm name dispatch through
`&dyn Reporter` and generic encoding on `TextReporter`.

## Tempting wrong fix

Changing `reporter_name` to accept only `TextReporter` discards the required
implementation erasure instead of making the intended trait boundary sound.

## References

[dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility),
[`E0038`](https://doc.rust-lang.org/error_codes/E0038.html), and
[erased-serde](https://github.com/dtolnay/erased-serde).
