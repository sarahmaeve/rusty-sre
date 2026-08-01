# 03 — UTF-8 prefix

Patch: [03-utf8-prefix.patch](../03-utf8-prefix.patch)

## Contract

`prefix(input, n)` returns the first `n` Unicode scalar values, or `TooShort` when
the input has fewer. The returned `&str` always ends at a UTF-8 boundary.

## Root cause

The implementation treated `str::len` and a slice endpoint as character counts.
Both use bytes.

## Why the symptom follows

A non-ASCII scalar occupies multiple bytes. The numeric endpoint can land within
its encoding, and checked string indexing panics at a non-character boundary.

## Repair strategy

Find the byte offset of the `n`th character with `char_indices`; use the full byte
length when exactly `n` characters exist, and report short input otherwise.

## Verification

Run `make ex N=03`. Cover zero, ASCII, multi-byte scalars, exact length, and too-short
input.

## Tempting wrong fix

Using `input.chars().take(n).collect::<String>()` is safe but changes the API from a
borrowed slice to an allocation without need.

## References

[`str::char_indices`](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices),
[string slices](https://doc.rust-lang.org/book/ch04-03-slices.html), and Servo's
[source](https://github.com/servo/servo).
