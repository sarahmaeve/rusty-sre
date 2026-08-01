# 05 — Fail-safe severity default

Patch: [05-safe-severity-default.patch](../05-safe-severity-default.patch)

## Contract

An explicit severity is preserved. Missing severity receives the domain's safe
default, `5`, rather than the least severe value.

## Root cause

The `Option<u8>` was collapsed with `unwrap_or(0)`, embedding the wrong policy at
the absence boundary.

## Why the symptom follows

`None` contains no numeric evidence. Selecting zero makes absent input appear
explicitly harmless and can suppress attention to an incompletely classified event.

## Repair strategy

Change only the documented default while leaving every `Some(value)` unchanged.
Keep the policy visible at the conversion point.

## Verification

Run `make ex N=05`. Test `None`, zero, five, and the allowed upper boundary.

## Tempting wrong fix

Calling `unwrap` rejects omission with a panic and replaces a policy bug with an
availability bug.

## References

[`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or),
[API Guidelines: documentation](https://rust-lang.github.io/api-guidelines/documentation.html),
and Kubernetes' [API conventions](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md).
