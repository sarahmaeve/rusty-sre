# Hints

## 1 — Direction

Compare the complete behavior set of the default and feature-enabled builds.

## 2 — Localization

Expand the active `#[cfg]` items twice: once with `audit`, once without it.

## 3 — Mechanism

Mutually exclusive branches construct unrelated vectors. The optional branch
replaces the baseline instead of adding to it.
