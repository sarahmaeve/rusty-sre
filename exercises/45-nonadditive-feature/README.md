# 45 — Enabling a feature disables baseline behavior

## Symptom

An audit-enabled build performs the audit check but silently drops schema and
bounds checks present in the default build.

## Contract

The `audit` feature adds an optional capability. Enabling it does not remove
baseline validation.

## Reproduce

Run `make ex N=45`. This exercise enables the feature explicitly; compare the
result with `cargo test -p advanced-core` without it.

## Task

Trace both `cfg` branches and treat feature combinations as build inputs. Refactor
the feature to extend common behavior rather than choose a replacement product.

## What you learn

You will read conditional compilation, Cargo features, and the ecosystem rule that
features should normally be additive.

Read Cargo's [feature documentation](https://doc.rust-lang.org/cargo/reference/features.html)
and [feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification).
