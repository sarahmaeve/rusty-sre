# Hints

## 1 — Direction

Draw the owned values before and after the clone. Which one is reachable later?

## 2 — Localization

Inspect the function parameter and the value whose status field is assigned.
Determine whether they refer to the same `Service`.

## 3 — Mechanism

Mutation is applied to an independent owned copy. Dropping that copy cannot alter
the original value held by the collection.
