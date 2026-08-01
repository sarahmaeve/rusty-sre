# Hints

## 1 — Direction

Do not assume every filesystem component is present or representable as `str`.

## 2 — Localization

Inspect the conversion of the entire `Path` to text. Compare it with the structured
extension API and list the absence represented by each optional step.

## 3 — Mechanism

Whole-path lossy suffix matching is case-sensitive and ignores the structure that
`Path` already exposes. Inspect only the final extension and compare by the stated
ASCII rule.
