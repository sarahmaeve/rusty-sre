# Hints

## 1 — Direction

For every reference in the return type, name the value that owns its referent.

## 2 — Localization

Find where the returned data is allocated and when that local owner is dropped.

## 3 — Mechanism

The referent belongs to a local value destroyed at function return. No lifetime
annotation can make that storage live longer.
