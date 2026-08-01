# Hints

## 1 — Direction

Do not start with three. Simulate budgets of zero and one.

## 2 — Localization

Compare the parameter name with the loop bound and the unconditional first
attempt implied by the contract.

## 3 — Mechanism

The implementation treats the retry count as the total attempt count. It forces a
minimum of one but never adds the initial call to a nonzero retry budget.
