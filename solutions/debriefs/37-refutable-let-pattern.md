# 37 — Give the refutable pattern an else path

Patch: [37-refutable-let-pattern.patch](../37-refutable-let-pattern.patch)

## Contract

Sample events expose their payload to the main path. Shutdown ends the command
without producing a fabricated payload.

## Root cause

A plain `let` used `Event::Sample(value)`, which does not match every possible
`Event`.

## Why the symptom follows

Bindings must initialize all names whenever execution continues. `Shutdown` has no
`value`, so Rust requires control flow that handles or leaves on that variant.

## Repair strategy

Use `let else` and make the nonmatching branch diverge with an early return. The
sample binding stays in the surrounding scope.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/37-refutable-let-pattern/src/main.rs`. Exercise both event variants and
confirm only samples reach the print.

## Tempting wrong fix

Using `unreachable!()` for `Shutdown` silences the compiler by replacing a valid
domain state with a runtime panic.

## References

[refutability](https://doc.rust-lang.org/book/ch19-02-refutability.html),
[`let` statements](https://doc.rust-lang.org/reference/statements.html#let-statements),
and [`E0005`](https://doc.rust-lang.org/error_codes/E0005.html).
