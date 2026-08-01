# 10 — HTTP success boundary

## Symptom

HTTP status 299 is classified as a failure while the rest of the 2xx class is
classified as success.

## Contract

Every status from 200 through 299 is successful. Status 199 and 300 are not.

## Reproduce

Run `make ex N=10`. Check 199, 200, 298, 299, and 300.

## Task

Map the protocol's inclusive upper boundary to Rust's half-open range semantics.

## What you learn

You will debug inclusive/exclusive boundary translation and test both neighbors.

Read [RFC 9110 status codes](https://www.rfc-editor.org/rfc/rfc9110.html#name-status-codes)
and [`Range::contains`](https://doc.rust-lang.org/std/ops/struct.Range.html#method.contains).
