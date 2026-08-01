# Study Guide

The goal is not memorizing Rust syntax. The goal is reliable diagnosis of
unfamiliar Rust systems. Progress by demonstrated capability rather than time
spent.

## 1. Read values and control flow

Run guides 01–02, then exercises 09–11 and 32.

You are ready to continue when you can trace ownership-neutral code, read
patterns in `match`, and explain why exhaustive matching protects a state
transition.

## 2. Track ownership

Run guides 03–04, then exercises 03, 06, 14, and 27–29.

You should be able to identify who owns a value, why a move occurred, how long
a borrow remains live, and whether cloning is part of the domain operation or
merely avoiding an ownership decision.

## 3. Follow absence and failure

Run guide 05, then exercises 01, 05, 08, 12, and 13.

You should be able to follow `Option` and `Result` through `?`, combinators,
and error conversion without mentally replacing them with implicit control
flow. Pay particular attention to code that turns failure into a default.

## 4. Read collections and trait contracts

Run guides 06–07, then exercises 02, 07, and 31.

You should recognize iterator ownership, collection entry APIs, and the
contracts connecting `Eq`, `Hash`, `Ord`, and collection behavior.

## 5. Navigate real crates

Run guide 08. Trace one request or value across all workspace crates. Inspect
the root and member manifests, enabled features, public modules, and tests.

Then revisit exercises 01–14 in hard mode. The goal is localization without a
topic label.

## 6. Understand shared ownership and cleanup

Run guides 09–10, then exercises 15–18 and 30.

You should be able to explain runtime borrow checking, reference cycles,
poisoned locks, RAII cleanup, and why `Send` and `Sync` appear in compiler
diagnostics.

## 7. Diagnose asynchronous systems

Run guide 11, then exercises 19–26.

Look for blocking work, locks across suspension points, detached failures,
unbounded concurrency, timeout scope, cancellation propagation, and shutdown
completion.

## 8. Review safety boundaries

Run guide 12. Read the unsafe-code diagnosis exercise and its linked source.
The required skill is not writing unsafe code; it is finding the invariant,
the caller obligations, and the tests or tools that support the safety claim.

## 9. Transfer the skill

Complete the diagnosis track and both report-first investigations. A strong
final handoff states:

- observed behavior;
- localized mechanism;
- evidence supporting it;
- bounded repair;
- verification performed;
- remaining uncertainty.

## Useful commands

```bash
make guides
make ex N=19
make hard N=19
make status
make solution N=19
make unsolution N=19
make verify-solution N=19
```
