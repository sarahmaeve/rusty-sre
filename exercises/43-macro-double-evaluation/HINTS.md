# Hints

## 1 — Direction

Do not reason from the macro call. Write out the tokens produced for that call.

## 2 — Localization

Count occurrences of the sampler expression in the expanded conditional.

## 3 — Mechanism

The condition evaluates the expression once and the selected branch evaluates the
same tokens again. An expression fragment is substitution, not a stored value.
