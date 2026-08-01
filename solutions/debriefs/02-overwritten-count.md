# 02 — Repeated errors are overwritten

Patch: [02-overwritten-count.patch](../02-overwritten-count.patch)

## Contract

`count_errors` returns the occurrence count for each service name.

## Root cause

Every iteration used `insert(service, 1)`. Repeated keys replaced their earlier
value with another `1`.

## Why the symptom follows

`HashMap::insert` stores one value per equal key. Ignoring its previous value turns
the function into a presence map, so three `api` events still produce `api: 1`.

## Repair strategy

Use the entry API to create a zero count only for a new key and increment the value
for every occurrence.

## Verification

Run `make ex N=02`. Cover empty input, distinct keys, repeated keys, and interleaved
repetitions.

## Tempting wrong fix

Counting the input length reports total events but cannot provide per-service
counts.

## References

[`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry),
[`Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html), and
ripgrep's [ignore crate](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore).
