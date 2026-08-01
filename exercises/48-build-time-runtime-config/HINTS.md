# Hints

## 1 — Direction

Write down when each candidate value is chosen: compilation or process execution.

## 2 — Localization

Follow the runtime parameter and note whether it participates in the returned
expression.

## 3 — Mechanism

`option_env!` expands to a value captured while compiling. The runtime argument is
discarded, so deployment input cannot win the precedence chain.
