# 36 — Bind the erased source item

Patch: [36-associated-type-binding.patch](../36-associated-type-binding.patch)

## Contract

`drain` dynamically dispatches any `Source` whose associated `Item` is `String` and
counts every yielded service name.

## Root cause

`Box<dyn Source>` erased the implementor without specifying the associated item
type required to type-check `next`.

## Why the symptom follows

Different implementations can select unrelated `Item` types. Without an equality
binding, the trait object's `next` method has no single known return type.

## Repair strategy

Bind `Item = String` on the trait object at `drain`'s API boundary. The implementor
remains erased; the operation's data contract does not.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/36-associated-type-binding/src/main.rs`. Add a second string source and
confirm a source with another item type is rejected.

## Tempting wrong fix

Making `drain` generic over the source compiles but removes the explicitly required
runtime polymorphism and changes storage and code-generation behavior.

## References

[`E0191`](https://doc.rust-lang.org/error_codes/E0191.html),
[associated types](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types),
and Futures’ [`Stream`](https://github.com/rust-lang/futures-rs/blob/master/futures-core/src/stream.rs).
