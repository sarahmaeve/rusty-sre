# Rusty SRE

Learn Rust through SRE-themed challenges. No Cargo required.

Built for SRE and DevOps engineers who know Python and want to pick up Rust. Each challenge uses real-world scenarios — alert pipelines, metrics aggregation, fleet management, incident response — with Python-to-Rust comparison tables to bridge what you already know.

## Quick Start

```bash
git clone <this-repo> && cd rusty-sre
cd challenges/01_vectors
rustc concept.rs --edition 2024 --test && ./concept
```

## Challenges

| # | Challenge | Topic | Prereqs |
|---|-----------|-------|---------|
| 01 | [Vectors](challenges/01_vectors) | `Vec<T>` — Rust's growable array | None |
| 01a | [Ownership & Borrowing](challenges/01a_vectors_advanced) | Borrow checker, moves, lifetimes | 01 |
| 02 | [HashMaps](challenges/02_hashmaps) | `HashMap<K,V>`, Entry API, counting/grouping | 01 |
| 02a | [HashSets & Performance](challenges/02a_hashmaps_advanced) | `HashSet`, set algebra, capacity, `retain()` | 02 |
| 03 | [derive](challenges/03_derive) | `#[derive]`, standard traits, manual impls | 02 |
| 03a | [Derive Ecosystem](challenges/03a_derive_ecosystem) | Patterns behind serde, thiserror, clap | 03 |

## How Each Challenge Works

Every challenge has three standalone `.rs` files:

- **`concept.rs`** — Heavily commented explainer with tests. Read this first to learn the concept.
- **`skeleton.rs`** — Fill in the `todo!()` stubs to make the tests pass. This is the challenge.
- **`debug.rs`** — A program with bugs. Find and fix them. Some won't compile; some produce wrong results.

Each file compiles directly with `rustc`:

```bash
# Learn the concept (all tests pass)
rustc concept.rs --edition 2024 --test && ./concept

# Do the challenge (tests fail until you complete the TODOs)
rustc skeleton.rs --edition 2024 --test && ./skeleton

# Hunt the bugs (may not compile until you fix them)
rustc debug.rs --edition 2024 --test && ./debug
```

An HTML overview is available at [`challenges/index.html`](challenges/index.html).

## Prerequisites

- **Rust** — Install via [rustup.rs](https://rustup.rs). Any recent stable version (1.85+) works.
- **A text editor** — That's it. No Cargo, no IDE plugins, no dependencies.
