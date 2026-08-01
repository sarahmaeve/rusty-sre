# Async hang: shutdown waits forever

This test has several plausible suspects: channel closure, task joining, dependency
cancellation, and lock scope. Do not choose by keyword alone.

## Contract

Shutdown stops admission, signals owned workers, and completes within its deadline.
A blocked dependency call cannot create an unbreakable wait cycle.

## Investigation

Build a wait-for graph from the task snapshot. Mark owned resources, guards, and
events required to make each await ready. Then choose the single trace or source
scope most likely to confirm a cycle.

Exercises 20 and 25 are relevant only after completing
[WORKSHEET.md](WORKSHEET.md).

Reference: Tokio's [graceful shutdown guide](https://tokio.rs/tokio/topics/shutdown)
and [async tracing](https://tokio.rs/tokio/topics/tracing).
