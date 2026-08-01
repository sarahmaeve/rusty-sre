# 46 — Express overflow in the return type

Patch: [46-profile-independent-overflow.patch](../46-profile-independent-overflow.patch)

## Contract

Generation exhaustion returns `None` in debug and release builds.

## Root cause

A debug-only assertion guarded ordinary addition rather than implementing the
fallible `Option` contract.

## Why the symptom follows

Debug builds execute the assertion and panic. Optimized builds normally omit it;
overflow behavior then depends on profile settings.

## Repair strategy

Use checked arithmetic, whose value-level outcome is independent of assertion and
overflow-check configuration.

## Verification

Run `make ex N=46` in debug and release profiles, plus boundary-neighbor cases.

## Tempting wrong fix

Changing `debug_assert!` to `assert!` makes profiles agree only by panicking and
still ignores the function's `Option` return type.

## References

[`checked_add`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add),
the Reference on [overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow),
and Cargo [profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
