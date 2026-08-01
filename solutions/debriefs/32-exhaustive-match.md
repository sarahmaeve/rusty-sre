# 32 — Handle every state explicitly

Patch: [32-exhaustive-match.patch](../32-exhaustive-match.patch)

## Contract

Only `Healthy` is available. Both `Degraded` and `Draining` are unavailable.

## Root cause

The match handled the first two enum variants but omitted `Draining`.

## Why the symptom follows

Enums are closed sets of possible values, and Rust requires every match to cover
that set. The missing witness produces `E0004` before ambiguous behavior ships.

## Repair strategy

Add a named `Draining` arm with its domain behavior. Preserve exhaustiveness so a
future state addition forces another deliberate decision.

## Verification

Apply the patch, run `cargo run --manifest-path compile-fail/32-non-exhaustive-match/Cargo.toml`,
and assert availability for all three variants.

## Tempting wrong fix

A wildcard false arm compiles but silently assigns every future state the same
behavior, removing the compiler's change-impact signal.

## References

[`E0004`](https://doc.rust-lang.org/error_codes/E0004.html),
[match control flow](https://doc.rust-lang.org/book/ch06-02-match.html), and
rust-analyzer's [source](https://github.com/rust-lang/rust-analyzer/tree/master/crates).
