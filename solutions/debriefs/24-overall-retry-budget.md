# 24 — One overall retry budget

Patch: [24-overall-retry-budget.patch](../24-overall-retry-budget.patch)

## Contract

All delivery attempts share one wall-clock budget. Success reports attempts used;
failed attempts within budget exhaust normally; expiry returns `DeadlineExceeded`.

## Root cause

Each attempt received a newly created timeout for the full overall budget.

## Why the symptom follows

Elapsed time from prior attempts is forgotten. Several attempts can each consume
nearly the full duration, multiplying the documented limit.

## Repair strategy

Place the complete retry loop inside one timeout so every await consumes the same
budget. Preserve `AttemptsExhausted` when the loop finishes in time.

## Verification

Run `make ex N=24`. Cover early success, in-budget exhaustion, and cumulative delay
that exceeds the budget although no individual attempt does.

## Tempting wrong fix

Dividing the budget equally among attempts changes the policy, wastes unused time,
and still requires care around backoff and attempt-count changes.

## References

Tokio's [`timeout`](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html),
[`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html), and Tower's
[timeout middleware](https://github.com/tower-rs/tower/tree/master/tower/src/timeout).
