# Hints

## 1 — Direction

An error message can contain the cause's words without retaining the cause itself.

## 2 — Localization

Inspect both the error variant's fields and its `Error` implementation. Then trace
the conversion from the lower-level error.

## 3 — Mechanism

The wrapper still owns the typed lower error, but `source` does not return a reference
to it. Display formatting and causal traversal are separate protocols.
