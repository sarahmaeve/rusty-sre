# Hints

## 1 — Direction

Do not add a wildcard until you can state why all unmatched states are equivalent.

## 2 — Localization

Use the compiler's witness pattern to find the missing semantic decision, then
search for other matches over the same enum.

## 3 — Mechanism

The enum's value set expanded, but the match still covers only the former set. Rust
requires proof that exactly one arm can handle every possible value.
