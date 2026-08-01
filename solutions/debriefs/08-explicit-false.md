# 08 — Preserve explicit false

Patch: [08-explicit-false.patch](../08-explicit-false.patch)

## Contract

Serialized `DeliveryPolicy` includes `cancel_on_error`, including when its value is
explicitly `false`. Deserialization may still default a missing legacy field.

## Root cause

`skip_serializing_if = "is_false"` omitted false values from output.

## Why the symptom follows

Serde applies the predicate before emitting the field. The receiver therefore sees
absence rather than evidence that the producer explicitly selected false.

## Repair strategy

Remove the serialization skip while retaining the deserialization default for
compatibility with older payloads.

## Verification

Run `make ex N=08`. Check encoded true and false fields, plus decoding of input that
omits the field.

## Tempting wrong fix

Removing `#[serde(default)]` makes explicit false visible but breaks decoding of
older payloads; serialization and deserialization compatibility are separate.

## References

Serde [field attributes](https://serde.rs/field-attrs.html),
Serde's [default attribute](https://serde.rs/attr-default.html), and
[Serde source](https://github.com/serde-rs/serde).
