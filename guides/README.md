# Rust reading guides

These small programs teach the language needed to debug unfamiliar Rust. Each
guide runs independently, asserts its important claims, and links to deeper
documentation. Read the source before running it: predicting ownership, control
flow, and output is part of the exercise.

The guides target Rust 1.97.1 and edition 2024.

```console
cargo run --manifest-path guides/Cargo.toml --example 01_bindings
cargo run --manifest-path guides/Cargo.toml --example 11_async
cargo run --manifest-path guides/Cargo.toml --example 18_pin
cargo check --manifest-path guides/Cargo.toml --all-targets --all-features
cargo clippy --manifest-path guides/Cargo.toml --all-targets --all-features
```

## Sequence

| Guide | Read this when code uses… |
| --- | --- |
| `01_bindings` | bindings, expressions, destructuring, `match` |
| `02_state` | structs, enums, methods, state machines |
| `03_ownership` | moves, borrows, `Copy`, `Clone` |
| `04_text` | `String`, `str`, slices, paths, UTF-8 |
| `05_errors` | `Option`, `Result`, `?`, custom errors |
| `06_iterators` | collections, closures, iterator pipelines |
| `07_traits` | generics, associated types, trait objects |
| `08_crates` | modules, visibility, features, Cargo targets |
| `09_pointers` | `Box`, `Rc`, `Arc`, cells, destructors |
| `10_threads` | threads, channels, mutexes, `Send`, `Sync` |
| `11_async` | tasks, futures, cancellation, bounded work |
| `12_unsafe` | unsafe APIs and documented invariants |
| `13_lifetimes` | explicit lifetimes, bounds, elision, higher-ranked bounds |
| `14_testing` | test layers, panic contracts, properties, `cfg(test)` |
| `15_macros` | declarative macros, hygiene, repetition, generated APIs |
| `16_advanced_traits` | associated types, UFCS, supertraits, newtypes, DSTs |
| `17_cargo` | targets, profiles, features, build scripts, workspace inheritance |
| `18_pin` | `Pin`, `Unpin`, future state, `Poll`, and `Waker` |
| `19_patterns` | refutability, guards, binding modes, nested patterns |
| `20_ffi` | C ABI, layouts, raw pointers, strings, safety contracts |

These are primers, not solutions to the debugging exercises. Comments explain
language rules and API contracts; they intentionally do not catalog every smell
or prescribe a single design.
