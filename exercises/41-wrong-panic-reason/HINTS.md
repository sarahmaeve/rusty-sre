# Hints

## 1 — Direction

A panic is an outcome category, not proof that the intended condition was checked.

## 2 — Localization

Trace zero through `require_positive` in execution order. Record the first
operation that cannot accept it.

## 3 — Mechanism

Arithmetic panics before the explicit validation runs. A test that accepts any
panic cannot distinguish the accidental failure from the required one.
