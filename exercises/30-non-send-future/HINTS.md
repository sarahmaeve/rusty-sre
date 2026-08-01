# Hints

## 1 — Direction

An async function stores live locals in its future across suspension points.

## 2 — Localization

Follow the diagnostic to the non-`Send` type, then check whether it remains live at
the indicated `.await`.

## 3 — Mechanism

The executor may move a suspended task between worker threads. A captured local
that lacks `Send` makes the entire generated future lack `Send`.
