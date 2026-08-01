# 30 — End the MutexGuard before await

Patch: [30-drop-guard-before-await.patch](../30-drop-guard-before-await.patch)

## Contract

The spawned multi-thread task records the value and may then await unrelated delay;
no standard mutex guard crosses a suspension point.

## Root cause

A `std::sync::MutexGuard` remained live across `tokio::time::sleep(...).await`.

## Why the symptom follows

Tokio may move a suspended spawned future between worker threads. The non-`Send`
guard becomes part of the future's saved state, so the future fails `spawn`'s bound.

## Repair strategy

Complete the synchronous state mutation in an explicit inner scope, drop the guard,
and only then await the delay.

## Verification

Apply the patch, then run `cargo run --manifest-path compile-fail/30-non-send-future/Cargo.toml`.
Confirm it compiles without the “future cannot be sent” diagnostic.

## Tempting wrong fix

Switching blindly to an async mutex makes compilation easier but still creates an
unnecessarily long critical section.

## References

Tokio's [`spawn`](https://docs.rs/tokio/latest/tokio/fn.spawn.html),
[`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html), and Tokio's
[shared-state tutorial](https://tokio.rs/tokio/tutorial/shared-state).
