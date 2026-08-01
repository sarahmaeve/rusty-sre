# Hints

## 1 — Direction

Determine what condition proves that the complete stream has been received.

## 2 — Localization

Inspect how many times the receiver polls `recv`, when it is dropped, and how the
producer task is joined.

## 3 — Mechanism

After one receive, the receiver is dropped. Remaining sends fail, their errors are
ignored, and the producer's join result is also discarded.
