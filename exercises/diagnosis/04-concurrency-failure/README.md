# Concurrency failure: quota oversubscription

One seed can pass or fail, so the seed alone does not define the schedule. The
artifact records application-level observations, not every memory operation.

## Contract

Successful reservations never exceed the initial quota. Each accepted reservation
decrements the shared total exactly once, and the decision is linearizable.

## Investigation

Separate suspicious logging from state evidence. Identify the compound decision
that must appear atomic, then design a deterministic schedule or model test that
exposes interleaving without relying on repeated luck.

This is a standalone artifact: no implementation is supplied. Complete
[WORKSHEET.md](WORKSHEET.md), then sketch the smallest deterministic reproducer and
a model-test invariant that would distinguish an atomic reservation from a split
read–decision–write sequence.

Reference: [atomics and locks](https://doc.rust-lang.org/nomicon/atomics.html),
[`compare_exchange`](https://doc.rust-lang.org/std/sync/atomic/struct.Atomic.html#method.compare_exchange),
and [Loom](https://github.com/tokio-rs/loom).
