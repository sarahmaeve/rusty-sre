# 05 — Missing severity fails open

## Symptom

An incident without a classified severity is treated as severity zero and falls
below alerting thresholds.

## Contract

An explicit severity is preserved. Missing severity uses the documented fail-safe
value `5`, not the least severe value.

## Reproduce

Run `make ex N=05`. Compare `None` with explicit severities at zero and five.

## Task

Trace where `Option<u8>` becomes a concrete severity. Align that boundary with the
domain policy without changing explicit values.

## What you learn

You will examine policy hidden in `unwrap_or` and review defaults as part of an
API's safety boundary.

Read [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
and Kubernetes' [API conventions](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md).
