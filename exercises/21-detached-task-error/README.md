# 21 — Detached task loses its error

## Symptom

The public operation reports success even when background work fails immediately;
the failure is visible only as incidental runtime output, if at all.

## Contract

Work required for operation success has an owner that observes both task join
failure and the task's own result. Truly detached work has an explicit policy.

## Reproduce

Run `make ex N=21`. Use the rejecting sink and trace its result through the spawned
task to the public return value.

## Task

Trace the task handle and its nested result. Align task lifetime with the caller's
success boundary and preserve both failure layers.

## What you learn

You will inspect `JoinHandle`, nested `Result`, and structured task ownership.

Read Tokio's [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)
and Tokio's [task module source](https://github.com/tokio-rs/tokio/tree/master/tokio/src/task).
