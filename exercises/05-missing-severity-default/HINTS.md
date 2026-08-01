# Hints

## 1 — Direction

Treat omission as a policy decision, not automatically as numeric zero.

## 2 — Localization

Look for the point where `Option<u8>` is collapsed to `u8`, then compare the chosen
number with the stated contract.

## 3 — Mechanism

The conversion embeds a domain default. The type checker can verify the number's
type, but it cannot verify that the number is the safe policy.
