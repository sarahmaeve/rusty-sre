# 31 — HashSet key lacks Hash and Eq

## Symptom

Constructing a `HashSet<Host>` fails because `Host` does not implement the equality
and hashing contracts required of set elements.

## Contract

`Host` values have value equality based on their fields and hash consistently with
that equality so they can be set members.

## Reproduce

Run `make ex N=31`. Trace each diagnostic note from `HashSet::from` back to the
missing implementations on `Host`.

## Task

Decide whether field-based derived identity matches the domain, then provide the
complete set-key contract at the type definition.

## What you learn

You will read trait-bound diagnostics and connect `PartialEq`, `Eq`, and `Hash`.

Read [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html),
[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html), and
[`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html).
