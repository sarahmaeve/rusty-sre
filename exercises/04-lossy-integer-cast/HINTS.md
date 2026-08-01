# Hints

## 1 — Direction

List the source and destination ranges before reading the implementation.

## 2 — Localization

Find where the wide counter crosses an API or storage boundary into a narrower or
signed representation.

## 3 — Mechanism

An `as` numeric cast is defined even when the value is not representable. It does
not communicate failure to the caller.
