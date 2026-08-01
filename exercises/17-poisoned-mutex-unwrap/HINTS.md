# Hints

## 1 — Direction

A poisoned lock still contains data. The question is whether that data is safe to
use, repair, or reject.

## 2 — Localization

Inspect the acquisition immediately after the deliberate panic and distinguish a
lock error from an unavailable lock.

## 3 — Mechanism

Unwrapping `LockResult` converts the poison marker into another panic, spreading
the first failure rather than applying a state-specific policy.
