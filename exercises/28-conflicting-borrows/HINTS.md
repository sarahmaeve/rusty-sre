# Hints

## 1 — Direction

The borrow ends at its last use, not necessarily at the closing brace.

## 2 — Localization

Label creation of the shared reference, the mutation, and the later shared access.

## 3 — Mechanism

The later read keeps the shared borrow live across the mutation, so exclusive
access cannot be proven safe.
