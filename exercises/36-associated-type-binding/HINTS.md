# Hints

## 1 — Direction

Ask what concrete type must emerge from `source.next()` inside `drain`.

## 2 — Localization

The missing information belongs in the trait object type at the function boundary,
not in the concrete source's already-complete implementation.

## 3 — Mechanism

Different `Source` implementations may choose different `Item` types. A single
trait object type must fix that associated type so calls have one known signature.
