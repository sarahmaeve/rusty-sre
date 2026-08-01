# 34 — State the spawned value bound

Patch: [34-spawned-value-bound.patch](../34-spawned-value-bound.patch)

## Contract

`spawn_drop` returns immediately with a handle to an unscoped thread that owns its
captured value and may continue after the caller returns.

## Root cause

The wrapper promised only `T: Send`, while `thread::spawn` also requires captured
state to be `'static`.

## Why the symptom follows

A `Send` type may contain borrowed references. Moving such a type across threads is
permitted, but the referents might disappear before an unscoped thread finishes.

## Repair strategy

Propagate `spawn`'s `T: 'static` requirement into the helper API. Owned values such
as `String` satisfy it without living forever; they contain no short borrow.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/34-spawned-value-bound/src/main.rs`. Confirm owned input works and a
type containing a local borrow is rejected at the call boundary.

## Tempting wrong fix

Adding `Sync` addresses shared-reference access between threads, not how long a
captured value's referents remain valid.

## References

[`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html),
[`E0310`](https://doc.rust-lang.org/error_codes/E0310.html), and Rayon’s
[scoped execution source](https://github.com/rayon-rs/rayon/tree/main/rayon-core/src).
