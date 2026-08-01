# Hints

## 1 — Direction

List every way control can leave the callback, including unwinding.

## 2 — Localization

Compare the lifetime of the saved previous value with the line that performs
restoration.

## 3 — Mechanism

Ordinary teardown is skipped during unwinding. A value whose destructor owns the
restoration remains active until the enclosing scope exits.
