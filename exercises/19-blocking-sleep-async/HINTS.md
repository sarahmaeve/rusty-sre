# Hints

## 1 — Direction

Identify calls that occupy an operating-system thread until they return.

## 2 — Localization

Inspect the delay inside each task and compare its API with the runtime's timer.

## 3 — Mechanism

The synchronous sleep parks a runtime worker rather than returning `Pending`, so
other ready futures assigned to that worker cannot be polled.
