# 14 — Use path extension semantics

Patch: [14-path-extension.patch](../14-path-extension.patch)

## Contract

`is_json_config` recognizes a final UTF-8 extension equal to `json` without ASCII
case sensitivity. Missing or non-Unicode extensions return false without panic.

## Root cause

The path was converted lossily to text and checked with case-sensitive whole-string
suffix matching.

## Why the symptom follows

`CONFIG.JSON` fails the lowercase suffix check, and lossy conversion needlessly
changes platform path data before a structured component is inspected.

## Repair strategy

Ask `Path` for its final extension, convert only that optional `OsStr`, and compare
using the contract's ASCII case rule.

## Verification

Run `make ex N=14`. Cover uppercase, lowercase, no extension, nested paths, trailing
dots, and non-Unicode extensions where supported.

## Tempting wrong fix

Lowercasing the entire lossy path allocates, still ignores path structure, and can
alter non-Unicode data.

## References

[`Path::extension`](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension),
[`OsStr::to_str`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.to_str),
and Cargo's [cargo-util source](https://github.com/rust-lang/cargo/tree/master/crates/cargo-util/src).
