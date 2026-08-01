# Hints

## 1 — Direction

Draw one operation timeline rather than treating attempts independently.

## 2 — Localization

Find where the timeout duration is converted to a timer. Is that conversion inside
or outside the retry loop?

## 3 — Mechanism

Each iteration creates a new full-duration relative timeout, so elapsed time from
earlier attempts is forgotten.
