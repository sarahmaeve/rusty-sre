# Hints

## 1 — Direction

Reference counting cannot determine that an isolated cycle is unreachable.

## 2 — Localization

Draw parent-to-child and child-to-parent edges, then compare strong counts before
and after every assignment.

## 3 — Mechanism

Every node in the cycle retains another with a strong reference. No strong count
can reach zero, so recursive cleanup never begins.
