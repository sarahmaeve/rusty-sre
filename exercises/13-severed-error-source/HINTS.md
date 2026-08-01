# Hints

## 1 — Direction

An error message can contain the cause's words without retaining the cause itself.

## 2 — Localization

Inspect both the error variant's fields and its `Error` implementation. Then trace
the conversion from the lower-level error.

## 3 — Mechanism

Formatting the lower error into a string erases its type, and the wrapper no longer
has a value it can return from `source`.
