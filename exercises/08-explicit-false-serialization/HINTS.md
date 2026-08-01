# Hints

## 1 — Direction

List the wire output required for each boolean value. Then separately list what an
older input may omit.

## 2 — Localization

Inspect the field's skip condition and deserialization default. They affect
opposite directions of the wire boundary.

## 3 — Mechanism

The omission predicate selects the false value, so the serializer deliberately
removes a field the output contract requires.
