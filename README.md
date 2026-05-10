# Rusty SRE

Learn Rust through SRE-themed challenges. No Cargo required.

Built for SRE and DevOps engineers who know Python and want to pick up Rust. Each challenge uses real-world scenarios — alert pipelines, metrics aggregation, fleet management, incident response — with Python-to-Rust comparison tables to bridge what you already know.

## Quick Start

```bash
git clone <this-repo> && cd rusty-sre

# Run every concept + solution test in the repo:
make test

# Or run a single file:
cd challenges/01_vectors
rustc concept.rs --edition 2024 --test && ./concept
```

See [`STUDY_GUIDE.md`](STUDY_GUIDE.md) for prioritized 2-hour, 4-hour, and full-day prep tracks.

## Challenges

Challenges are numbered to follow a Beginner → Intermediate → Advanced ladder. The Beginner shelf (01–07) is being filled out — slots 04–07 are reserved for upcoming basics (strings, structs/enums, control flow, modules).

| # | Challenge | Topic | Difficulty | Builds on |
|---|-----------|-------|------------|-----------|
| 01 | [Vectors](challenges/01_vectors) | `Vec<T>` — Rust's growable array | Beginner | — |
| 02 | [HashMaps](challenges/02_hashmaps) | `HashMap<K,V>`, Entry API, counting/grouping | Beginner | 01 |
| 03 | [Option & Nullability](challenges/03_option) | `Option<T>`, combinators, `let else`, why Rust has no null | Beginner | 01 |
| 08 | [Ownership & Borrowing](challenges/08_borrowing) | Borrow checker, moves, lifetimes | Intermediate | 01 |
| 09 | [HashSets & Performance](challenges/09_hashsets_and_performance) | `HashSet`, set algebra, capacity, `retain()` | Intermediate | 02 |
| 10 | [derive](challenges/10_derive) | `#[derive]`, standard traits, manual impls | Intermediate | 02 |
| 11 | [Result & `?`](challenges/11_result_and_question_mark) | `Result<T,E>`, `?`, error enums, `From` for error conversion | Intermediate | 03, 10 |
| 12 | [Derive Ecosystem](challenges/12_derive_ecosystem) | Patterns behind serde, thiserror, clap | Advanced | 10, 11 |

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

### Running everything at once

The top-level [`Makefile`](Makefile) discovers every challenge and runs all its tests:

```bash
make test            # concept + solution tests across every challenge (everything passes)
make test-skeletons  # skeleton tests (expected to fail until you complete the TODOs)
make test-debug      # debug tests (expected to fail until you fix the bugs)

# Run one file in one challenge:
make CH=11_result_and_question_mark concept
make CH=11_result_and_question_mark debug
```

## Prerequisites

- **Rust** — Install via [rustup.rs](https://rustup.rs). Any recent stable version (1.85+) works.
- **A text editor** — That's it. No Cargo, no IDE plugins, no dependencies.
- **GNU make** (optional) — only needed for the bulk targets above.
