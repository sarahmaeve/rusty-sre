# 01 — Silent configuration fallback

Patch: [01-silent-config.patch](../01-silent-config.patch)

## Contract

Missing `threshold` and invalid `threshold` are separate configuration errors. A
present valid decimal value becomes the configured `u16`.

## Root cause

`parse().unwrap_or(80)` replaced every parse error with a plausible value. The
function already had `InvalidThreshold`, but never constructed it.

## Why the symptom follows

`unwrap_or` operates on `Result` here. Any malformed or out-of-range input becomes
`Ok(80)`, so startup cannot distinguish bad operator input from success.

## Repair strategy

Keep the missing check, propagate successful parsing, and map the parse failure to
the domain error while retaining the original input for diagnosis.

## Verification

Run `make ex N=01`. Test missing, malformed, out-of-range, and valid boundary values.

## Tempting wrong fix

Changing `80` to a “safer” default still hides invalid input; the bug is silent
recovery, not the chosen number.

## References

[`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err),
[`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse), and
Cargo's [configuration source](https://github.com/rust-lang/cargo/tree/master/crates/cargo-util/src).
