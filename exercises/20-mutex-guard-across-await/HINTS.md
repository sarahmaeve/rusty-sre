# Hints

## 1 — Direction

List every `.await` and every live guard at that point.

## 2 — Localization

Focus on the interval between lock acquisition and guard drop, including branches
and calls that may re-enter the same state.

## 3 — Mechanism

Suspending does not drop local variables. The guard remains live while another
future waits for the lock it protects.
