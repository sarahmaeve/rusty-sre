# 14 — Path and extension assumption

## Symptom

An uppercase `.JSON` configuration is rejected, and whole-path suffix matching
obscures the intended path-component rule.

## Contract

`is_json_config` recognizes a final UTF-8 extension equal to `json` without ASCII
case sensitivity. Missing and non-Unicode extensions return false.

## Reproduce

Run `make ex N=14`. Include an extensionless path, a trailing dot, mixed case, and
platform-native non-Unicode data where supported.

## Task

Replace whole-path lossy string matching with structured extension inspection.
Keep unsupported extensions as a normal false result.

## What you learn

You will work with `Path`, `OsStr`, optional components, and platform text.

Read [`Path::extension`](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension)
and Cargo's [path-sensitive source](https://github.com/rust-lang/cargo/tree/master/crates/cargo-util/src).
