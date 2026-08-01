# 11 — Inverted retention predicate

## Symptom

Host cleanup removes active hosts and preserves the decommissioned hosts.

## Contract

After `remove_decommissioned`, the list contains only original hosts absent from
the decommissioned list, in original order.

## Reproduce

Run `make ex N=11`. Inspect before/after keys rather than only the number removed.

## Task

Name the predicate in domain terms and determine whether the collection API asks
which values to keep or which to delete. Cover no matches and all matches.

## What you learn

You will read closure contracts and reduce errors caused by negatively named logic.

Read [`Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain)
and [`HashMap::retain`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain).
