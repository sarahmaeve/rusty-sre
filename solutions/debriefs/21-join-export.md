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

Retain and await the `JoinHandle`, distinguish join failure from the inner operation
result, and propagate the sink error. The small exercise treats task panic as an
invariant violation with `expect`.

## Verification

Run `make ex N=21`. Cover accepting and rejecting sinks and assert one write attempt.

## Tempting wrong fix

Adding more `yield_now` calls remains a timing guess; scheduling is not joining.

## References

Tokio's [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html),
[`spawn`](https://docs.rs/tokio/latest/tokio/fn.spawn.html), and Tokio's
[task source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/task).
