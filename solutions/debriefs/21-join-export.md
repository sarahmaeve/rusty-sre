# 21 — Join required export work

Patch: [21-join-export.patch](../21-join-export.patch)

## Contract

`dispatch_export` returns only after the required sink write completes, and a sink
rejection reaches its caller as `ExportError::Rejected`.

## Root cause

The task handle was discarded and the sink's `Result` was ignored.

## Why the symptom follows

`yield_now` offers scheduling but does not establish completion or transfer a task's
result. The dispatcher therefore returns `Ok(())` regardless of sink failure.

## Repair strategy

Retain and await the `JoinHandle`, represent join failure in the public error type,
and propagate both the outer task result and the inner sink result.

## Verification

Run `make ex N=21`. Cover accepting and rejecting sinks, assert one write attempt,
and verify that the error type retains a distinct join-failure variant.

## Tempting wrong fix

Adding more `yield_now` calls remains a timing guess; scheduling is not joining.

## References

Tokio's [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html),
[`spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html), and Tokio's
[task source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/task).
