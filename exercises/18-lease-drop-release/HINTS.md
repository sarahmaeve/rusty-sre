# Hints

## 1 — Direction

Write the counter transition expected at acquisition and at destruction.

## 2 — Localization

Trace the `Lease` from construction to scope exit, then inspect what its destructor
does to the same atomic counter.

## 3 — Mechanism

`acquire` increments the counter, but `Drop` only reads it. Reading state is not the
inverse transition needed to release a lease.
