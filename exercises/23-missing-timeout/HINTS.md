# Hints

## 1 — Direction

Find the await whose completion depends on a remote system.

## 2 — Localization

Trace the caller's time budget down to that await and inspect what happens to the
in-flight future when the caller stops waiting.

## 3 — Mechanism

The dependency future has no competing deadline future, so remaining pending is a
valid state forever.
