# 23 — Enforce the scrape deadline

Patch: [23-scrape-timeout.patch](../23-scrape-timeout.patch)

## Contract

A scrape completes within the supplied relative deadline or returns
`DeadlineExceeded`; a completed response is returned unchanged.

## Root cause

The deadline parameter was unused and the client future was awaited directly.

## Why the symptom follows

A pending fetch has no competing event to end the wait, so response latency alone
controls operation lifetime.

## Repair strategy

Wrap the fetch future in Tokio's timeout and map elapsed time to the domain error.
Dropping the timed-out future cancels this simulated fetch.

## Verification

Run `make ex N=23`. Test a response before the deadline, one after it, and a boundary
with enough scheduling tolerance.

## Tempting wrong fix

Applying a timeout only in the test prevents a hang but leaves production callers
unbounded and cannot return the API's typed timeout error.

## References

Tokio's [`timeout`](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html),
[cancellation safety](https://tokio.rs/tokio/tutorial/select), and Hyper's
[client source](https://github.com/hyperium/hyper/tree/master/src/client).
