# 29 — Return locally created data by value

Patch: [29-return-owned.patch](../29-return-owned.patch)

## Contract

`service_label` creates a label that remains valid after the function returns.

## Root cause

The function returned `&str` borrowed from a local `String`.

## Why the symptom follows

The local owner is dropped at function return, so the proposed reference would
dangle. Rust rejects it with `E0515`; annotations cannot extend storage lifetime.

## Repair strategy

Return the owned `String`, transferring its allocation to the caller. If the value
were a fixed literal instead, returning `&'static str` would be another design.

## Verification

Apply the patch, then run `cargo run --manifest-path compile-fail/29-reference-to-local/Cargo.toml`.
Confirm the label prints and no `E0515` remains.

## Tempting wrong fix

Adding a generic lifetime to the return type promises a relationship to caller
input that does not exist and cannot keep the local alive.

## References

[`E0515`](https://doc.rust-lang.org/error_codes/E0515.html),
[lifetime syntax](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), and
[`String`](https://doc.rust-lang.org/std/string/struct.String.html).
