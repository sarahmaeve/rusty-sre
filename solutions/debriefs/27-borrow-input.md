# 27 — Borrow read-only input

Patch: [27-borrow-input.patch](../27-borrow-input.patch)

## Contract

`summarize` counts matching lines without consuming them, so `main` can report both
the error count and original line count.

## Root cause

The helper accepted `Vec<String>` by value even though it only iterated by reference.

## Why the symptom follows

Passing `lines` transfers ownership because `Vec<String>` is not `Copy`. The later
`lines.len()` therefore uses a moved value and triggers `E0382`.

## Repair strategy

Accept a slice and pass a borrow. This communicates read-only access and supports
vectors, arrays, and other contiguous string storage without cloning.

## Verification

Apply the patch, then run `cargo run --manifest-path compile-fail/27-moved-value/Cargo.toml`.
Confirm the expected counts and no `E0382`.

## Tempting wrong fix

Cloning the vector preserves the caller binding but duplicates every owned string
to satisfy an ownership requirement the helper never needed.

## References

[`E0382`](https://doc.rust-lang.org/error_codes/E0382.html),
[slices](https://doc.rust-lang.org/book/ch04-03-slices.html), and
[ownership and functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions).
