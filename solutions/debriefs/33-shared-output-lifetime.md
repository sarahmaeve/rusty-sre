# 33 — Tie both possible outputs to one lifetime

Patch: [33-shared-output-lifetime.patch](../33-shared-output-lifetime.patch)

## Contract

`choose_label` returns either input as a borrow. Both inputs therefore remain valid
for the lifetime promised by the return type.

## Root cause

Only `primary` was tied to `'a`; `fallback` received an independent inferred
lifetime even though one branch returned it as `&'a str`.

## Why the symptom follows

The caller could otherwise supply a short-lived fallback and retain the returned
reference for the longer lifetime associated with primary. The signature did not
prove that branch safe.

## Repair strategy

Give both candidate inputs the same named lifetime as the output. At a call site,
that lifetime is constrained to the overlap during which both borrows are valid.

## Verification

Apply the patch and run `rustc --edition=2024 -D warnings
compile-fail/33-shared-output-lifetime/src/main.rs`. Exercise both selection branches
and place one input in a shorter nested scope to confirm returned-use limits.

## Tempting wrong fix

Requiring `&'static str` compiles for literals but needlessly rejects ordinary
borrowed `String` values whose lifetimes are sufficient for the call.

## References

[`E0621`](https://doc.rust-lang.org/error_codes/E0621.html),
[lifetime relationships](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html),
and the Reference on [lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html).
