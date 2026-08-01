# Debugging Rust Systems

## Begin with the contract

A compiler error, panic, failed assertion, stalled task, or resource graph is
evidence about a violated contract. State that contract before proposing a
repair. “The test should pass” is not a contract; “invalid configuration must
remain distinguishable from an omitted field” is.

## Read diagnostics structurally

For compiler diagnostics, identify:

1. the primary error code and message;
2. the value, reference, trait, or future involved;
3. where the relevant state was created;
4. where it was consumed or required later;
5. the compiler's suggested repair, if any;
6. whether that suggestion preserves the program's intended ownership or API.

Compiler suggestions optimize for compilation, not for the system's design.

## Use focused commands

```bash
cargo test -p fleet-core test_name -- --nocapture
cargo test -p async-ops test_name -- --ignored --nocapture
cargo check -p compile-27 --manifest-path compile-fail/Cargo.toml
cargo clippy -p fleet-core --all-targets
RUST_BACKTRACE=1 cargo test -p ops-core test_name -- --ignored
```

Prefer the smallest command that reproduces the symptom, then widen after the
repair.

## Common high-value questions

- Who owns this value now?
- Which borrow remains live, and why?
- Does this default represent absence, failure, or a real domain value?
- Can this conversion lose information?
- Do equality, hashing, and ordering describe the same identity?
- Is work bounded by requests, time, memory, or task count?
- Who cancels this task, and who waits for it?
- Can a lock guard survive an `.await`?
- What happens when a receiver closes or a spawned task fails?
- Which invariant makes this unsafe operation sound?

## Verification

Run the focused reproduction, relevant neighboring tests, formatting, and
Clippy. For concurrency defects, repeat the focused test and prefer a
deterministic synchronization point over timing-based confidence.
