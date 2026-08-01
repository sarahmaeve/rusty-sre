# 32 — Non-exhaustive match

## Symptom

Compilation fails with `E0004` after the state model gains another enum variant.

## Contract

Every state is handled intentionally. A wildcard is used only when all unmatched
present and future states truly share behavior.

## Reproduce

Run `make ex N=32`. Read the compiler's uncovered-pattern witness and locate the
decision whose behavior is now unspecified.

## Task

Define the new state's semantics at each relevant match. Preserve exhaustive
checking when future variants should force another decision.

## What you learn

You will use enums as closed state models and treat compile failures as change
impact reports.

Read [patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html),
[`E0004`](https://doc.rust-lang.org/error_codes/E0004.html), and rust-analyzer's
[enum-heavy protocol types](https://github.com/rust-lang/rust-analyzer/tree/master/crates).
