# Hints

## 1 — Direction

Write the required behavior for both enum variants before selecting syntax.

## 2 — Localization

The binding needs a diverging path for the variant that does not supply `value`.

## 3 — Mechanism

A plain `let` must bind for every possible input. This enum pattern is refutable
because `Shutdown` contains no sample payload.
