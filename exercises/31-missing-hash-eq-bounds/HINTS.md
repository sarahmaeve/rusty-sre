# Hints

## 1 — Direction

Read the `HashSet` construction error and every suggested derive.

## 2 — Localization

The diagnostic lists traits required on the set element. Map each one back to
`Host` and its fields.

## 3 — Mechanism

Hash lookup first chooses a bucket and then tests equality. `Host` provides neither
operation, even though its `String` field provides both.
