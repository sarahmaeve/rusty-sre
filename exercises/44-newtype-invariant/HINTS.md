# Hints

## 1 — Direction

A private tuple field helps only if every public constructor enforces the invariant.

## 2 — Localization

List the functions and trait implementations that can produce `Port` from an
unvalidated integer.

## 3 — Mechanism

`From` promises conversion cannot fail. That promise conflicts with a domain that
rejects one of the source type's values.
