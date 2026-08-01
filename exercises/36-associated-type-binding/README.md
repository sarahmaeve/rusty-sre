# 36 — Bind a trait object's associated type

## Symptom

Compilation fails with `E0191`: the value of `Source::Item` is unspecified in the
trait object type.

## Contract

`drain` accepts dynamically dispatched sources whose `next` operation yields owned
service names and returns the number consumed.

## Reproduce

Run `make ex N=36`.

Follow the associated type from the trait definition through the return type of
`next` to the erased source boundary.

## Task

Make the erased source's item type part of the function's API. Keep dynamic dispatch
and avoid replacing the associated type with an unrelated generic method.

## What you learn

You will read associated-type equality constraints and understand which type
information trait objects erase and retain.

Read [associated types](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types),
[`E0191`](https://doc.rust-lang.org/error_codes/E0191.html), and Futures’
[`Stream` source](https://github.com/rust-lang/futures-rs/blob/master/futures-core/src/stream.rs).
