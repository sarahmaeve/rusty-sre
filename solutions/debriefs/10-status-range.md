# 10 — HTTP success status range

Patch: [10-status-range.patch](../10-status-range.patch)

## Contract

Every HTTP status from 200 through 299 is successful; 199 and 300 are not.

## Root cause

The half-open range ended at `299`, so it contained 200 through 298.

## Why the symptom follows

Rust's `start..end` excludes `end`. The code visually resembled the protocol's last
success status while omitting it.

## Repair strategy

Use a half-open range ending at 300, preserving the simple `contains` check and
making the upper boundary correct.

## Verification

Run `make ex N=10`. Test 199, 200, 298, 299, and 300.

## Tempting wrong fix

Checking `status >= 200` accepts redirects, client errors, server errors, and
nonstandard values beyond the success class.

## References

[RFC 9110 status codes](https://www.rfc-editor.org/rfc/rfc9110.html#name-status-codes),
[`Range::contains`](https://doc.rust-lang.org/std/ops/struct.Range.html#method.contains),
and Hyper's [status type](https://docs.rs/http/latest/http/status/struct.StatusCode.html).
