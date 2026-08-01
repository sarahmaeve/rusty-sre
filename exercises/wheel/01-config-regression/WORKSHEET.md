# Investigation worksheet

## Impact and contract

State the user-visible impact and how present-invalid input must differ from absent
input.

## Initial hypotheses

Rank at least three explanations before reading evidence.

## Evidence ledger

| Packet | New fact | Hypothesis strengthened | Hypothesis weakened |
| --- | --- | --- | --- |
| | | | |

## Causal chain

Connect deployment input to parser state, worker-pool size, resource use, and
readiness behavior. Mark anything inferred rather than observed.

## Repair boundary

Name the smallest behavior change. List unrelated cleanup separately.

## Verification

Include malformed, absent, valid, and boundary values. Specify what startup logging
must report on rejection and whether snapshot tests should compare wire state or
effective state.

## Handoff

Write a six-sentence incident summary: impact, trigger, mechanism, evidence,
repair, and remaining risk.
