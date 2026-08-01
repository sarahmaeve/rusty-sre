# Hints

## 1 — Direction

Follow the stored value for a key each time that key reappears.

## 2 — Localization

Inspect the insertion loop and the final value after two inputs share a key.

## 3 — Mechanism

`insert(service, 1)` replaces the previous value. An entry operation can expose
the existing count and initialize only when the key is absent.
