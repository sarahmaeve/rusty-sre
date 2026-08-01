# Hints

## 1 — Direction

Count admitted work, not merely worker threads.

## 2 — Localization

Inspect the loop that creates futures or tasks and determine whether it ever waits
for capacity before creating the next one.

## 3 — Mechanism

The implementation eagerly spawns once per input. The runtime schedules tasks but
does not provide an application-level admission limit automatically.
