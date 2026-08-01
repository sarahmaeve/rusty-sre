# Hints

## 1 — Direction

Ask which value proves the spawned work completed and who inspects that value.

## 2 — Localization

Follow the return value of `spawn`. Then identify the outer task result and inner
operation result.

## 3 — Mechanism

Dropping a join handle detaches observation of the task. The caller returns before
the required work can contribute its failure.
