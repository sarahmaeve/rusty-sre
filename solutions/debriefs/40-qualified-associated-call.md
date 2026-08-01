# 40 — Select the associated implementation explicitly

Patch: [40-qualified-associated-call.patch](../40-qualified-associated-call.patch)

## Contract

The program calls the `label` associated function from `Compact`'s `Format`
implementation.

## Root cause

`Format::label()` named the trait but supplied neither a receiver nor a type context
from which Rust could choose an implementation.

## Why the symptom follows

Associated functions can differ by implementor. Even with one current impl, the
trait API permits more, so Rust does not infer `Compact` from global program state.

## Repair strategy

Use fully qualified syntax to name both the implementor and trait at the call site.
This also remains unambiguous if another trait defines a same-named item.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/40-qualified-associated-call/src/main.rs`. Add a second implementor and
confirm the selected output remains `Compact`'s.

## Tempting wrong fix

Adding a dummy `&self` receiver only to enable inference forces construction of a
value for an operation that has no instance state.

## References

[`E0790`](https://doc.rust-lang.org/error_codes/E0790.html),
[fully qualified syntax](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name),
and Tower’s [`Layer`](https://github.com/tower-rs/tower/blob/master/tower-layer/src/lib.rs).
