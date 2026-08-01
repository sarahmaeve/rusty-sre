# Hints

## 1 — Direction

Compare the fields used for identity by each trait implementation.

## 2 — Localization

Inspect `PartialEq`, `Eq`, and `Hash` together. Derived and manual implementations
can disagree even when each looks reasonable alone.

## 3 — Mechanism

The hash table chooses a bucket before testing equality. Equal values sent to
different buckets violate the collection's required invariant.
