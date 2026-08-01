# 09 — Every probe must be ready

Patch: [09-all-probes.patch](../09-all-probes.patch)

## Contract

`fleet_ready` is true only when every supplied required probe is ready. Under the
iterator contract, an empty probe set is vacuously ready.

## Root cause

The implementation used `any`, which answers whether at least one probe is ready.

## Why the symptom follows

Iteration short-circuits on the first `true`, so a later failed required probe does
not affect the aggregate.

## Repair strategy

Use the universal predicate `all` to express the contract directly. If empty fleets
should instead be unready, specify and test that as an additional policy.

## Verification

Run `make ex N=09`. Cover mixed, all-ready, all-failed, one-probe, and empty inputs.

## Tempting wrong fix

Negating `any(|probe| probe.ready)` says no probe is ready; the correct De Morgan
form would need to test for failed probes.

## References

[`Iterator::all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all),
[`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any),
and Linkerd2-proxy's [readiness-oriented source](https://github.com/linkerd/linkerd2-proxy).
