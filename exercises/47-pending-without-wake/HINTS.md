# Hints

## 1 — Direction

An executor is not required to poll every pending task repeatedly.

## 2 — Localization

Inspect the branch returning `Pending`. Identify what external event could cause
the task to re-enter `poll`.

## 3 — Mechanism

The future mutates its own state but never notifies the executor. Internal readiness
does not schedule another poll.
