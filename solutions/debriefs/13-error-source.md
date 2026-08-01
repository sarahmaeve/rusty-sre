# 13 — Preserve the error source chain

Patch: [13-error-source.patch](../13-error-source.patch)

## Contract

`PipelineError` displays its operational message and exposes its stored I/O error
through `Error::source`.

## Root cause

The wrapper owned `_source` but its `Error` implementation always returned `None`.

## Why the symptom follows

Display and causal traversal are separate protocols. Formatting still works, while
reporters walking `source()` stop at the wrapper and lose the I/O kind and details.

## Repair strategy

Borrow the optional stored error and coerce it to the trait-object reference
required by `source`.

## Verification

Run `make ex N=13`. Walk the chain, inspect the I/O kind, and confirm the top-level
message remains concise.

## Tempting wrong fix

Appending the source text to `Display` improves one string but does not restore
typed causal inspection.

## References

[`Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source),
[error handling project group](https://rust-lang.github.io/project-error-handling/),
and Cargo's [diagnostic source](https://github.com/rust-lang/cargo/tree/master/src/cargo/util).
