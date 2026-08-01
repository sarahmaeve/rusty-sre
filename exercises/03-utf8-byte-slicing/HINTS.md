# Hints

## 1 — Direction

Ask what an index into `str` measures and which indices are legal boundaries.

## 2 — Localization

Inspect the operation that creates the prefix. Compare it with iteration over the
input's textual elements.

## 3 — Mechanism

A Rust string is UTF-8. A byte offset can land inside a multi-byte encoded scalar,
and slicing at that offset checks and panics.
