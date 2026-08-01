# Hints

## 1 — Direction

Ask what information a method receiver normally contributes to trait resolution.

## 2 — Localization

The ambiguity is entirely at the associated function call; the implementation
itself already defines the desired label.

## 3 — Mechanism

With no `self` parameter and no generic context selecting a type, `Format::label`
does not identify which implementation should run.
