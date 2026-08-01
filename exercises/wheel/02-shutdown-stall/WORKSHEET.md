# Investigation worksheet

## Impact and contract

Define when shutdown is complete and what happens to buffered events at its
deadline.

## Initial hypotheses

Include at least one each for task ownership, external I/O, retry budgeting, and
locking.

## Evidence ledger

| Packet | Observation | Interpretation | Alternative interpretation |
| --- | --- | --- | --- |
| | | | |

## Wait-for graph

Draw the supervisor, worker, collector response, cancellation signal, and shared
state. Label which event could make each wait ready.

## Cause ranking

Classify candidate issues as proven cause, necessary contributor, independent risk,
or red herring. Cite evidence for every classification.

## Repair and verification

Define one bounded repair. Test cancellation while response headers remain pending,
resource cleanup, error propagation, and the shutdown deadline. State how you will
avoid a real-time 30-second test.

## Handoff

Write a concise operator-facing explanation that distinguishes connect timeout from
request deadline and explains whether buffered data can still be lost.
