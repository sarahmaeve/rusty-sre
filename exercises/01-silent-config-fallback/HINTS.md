# Hints

## 1 — Direction

Separate “not supplied” from “supplied but unusable.” They are different states.

## 2 — Localization

Follow the value from lookup through parsing to the point where a concrete number
is selected. Note every conversion between `Option` and `Result`.

## 3 — Mechanism

One convenience conversion removes the parse error before default selection. Look
for an API whose return type cannot retain why parsing failed.
