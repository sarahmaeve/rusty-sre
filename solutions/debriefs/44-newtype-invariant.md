# 44 — Make invalid newtype conversion fallible

Patch: [44-validated-newtype.patch](../44-validated-newtype.patch)

## Contract

All `Port` values are nonzero, and converting arbitrary integers reports the
reserved zero value as `PortError`.

## Root cause

The public `From<u16>` implementation constructed the private tuple field without
validation.

## Why the symptom follows

`From` has no error channel. Once implemented for the entire source type, it
promises every `u16`, including zero, has a valid `Port` representation.

## Repair strategy

Replace the infallible conversion with `TryFrom<u16>` and route configuration
through it.

## Verification

Run `make ex N=44`. Test zero, one, and `u16::MAX`.

## Tempting wrong fix

Checking only in `configured_port` leaves the public `From` path available to other
callers and fails to make the type invariant global.

## References

[`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) and the Rust
API Guidelines on [conversion traits](https://rust-lang.github.io/api-guidelines/interoperability.html#conversions-use-the-standard-traits-from-asref-asmut-c-conv-traits).
