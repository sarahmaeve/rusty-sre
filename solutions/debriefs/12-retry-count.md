# 12 — Retries are additional attempts

Patch: [12-retry-count.patch](../12-retry-count.patch)

## Contract

`max_retries` counts calls after the first attempt. Zero means one call; two means
at most three calls. Success returns immediately, and final failure is returned.

## Root cause

The implementation treated `max_retries` as the total attempt count, with a special
case that merely forced at least one.

## Why the symptom follows

For `max_retries = 2`, the loop executes twice and cannot reach a success scheduled
for the required third call.

## Repair strategy

Compute attempts as retries plus the initial call, using saturating arithmetic to
avoid overflow at the type boundary.

## Verification

Run `make ex N=12`. Cover zero retries, eventual success on the last call, immediate
success, and exhausted failure.

## Tempting wrong fix

Renaming the parameter to `max_attempts` without changing callers redefines the API
rather than repairing it.

## References

[`usize::saturating_add`](https://doc.rust-lang.org/std/primitive.usize.html#method.saturating_add),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), and Tower's
[retry middleware](https://github.com/tower-rs/tower/tree/master/tower/src/retry).
