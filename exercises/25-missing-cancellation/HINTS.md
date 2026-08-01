# Hints

## 1 — Direction

For each spawned task, identify its owner, stop signal, and completion observation.

## 2 — Localization

Inspect the worker's receive loop. Determine which future is pending when the
channel remains open but has no jobs.

## 3 — Mechanism

The cancellation token is passed into the worker but never polled. A pending
`recv` cannot wake because unrelated state changed.
