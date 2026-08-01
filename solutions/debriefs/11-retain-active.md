# 11 — Retain active hosts

Patch: [11-retain-active.patch](../11-retain-active.patch)

## Contract

After `remove_decommissioned`, `hosts` contains the original hosts not present in
the decommissioned list, in their original order.

## Root cause

The retain predicate returned true for decommissioned hosts.

## Why the symptom follows

`Vec::retain` keeps values whose predicate is true; it does not remove them. The
domain predicate and collection predicate had opposite meanings.

## Repair strategy

Negate membership in the decommissioned set so only active hosts are retained.

## Verification

Run `make ex N=11`. Cover no matches, all matches, interleaved matches, duplicates,
and empty inputs.

## Tempting wrong fix

Renaming the function to `keep_decommissioned` makes the code internally coherent
but violates every existing caller's cleanup contract.

## References

[`Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain),
[`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter),
and TiKV's [source](https://github.com/tikv/tikv).
