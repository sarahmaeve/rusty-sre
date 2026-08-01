# 08 — Serialization loses explicit false

## Symptom

Serialized `DeliveryPolicy` output omits `cancel_on_error` whenever it is false,
although downstream consumers require the producer's complete policy.

## Contract

Serialized output always includes `cancel_on_error`, including `false`.
Deserialization still accepts older payloads that omit the field.

## Reproduce

Run `make ex N=08`. Compare encoded output for true and false, then consider a
legacy input with the field missing.

## Task

Inspect serialization omission and deserialization defaults separately. Make the
writer explicit without breaking compatibility in the reader.

## What you learn

You will reason about Serde field attributes and asymmetric wire compatibility.

Read Serde's [field attributes](https://serde.rs/field-attrs.html) and the
[Serde source](https://github.com/serde-rs/serde).
