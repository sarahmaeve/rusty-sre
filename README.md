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

Challenges are numbered to follow a Beginner → Intermediate → Advanced ladder. Walk them in number order to see Beginner topics before Intermediate ones.

| # | Challenge | Topic | Difficulty | Builds on |
|---|-----------|-------|------------|-----------|
| 01 | [Vectors](challenges/01_vectors) | `Vec<T>` — Rust's growable array | Beginner | — |
| 02 | [HashMaps](challenges/02_hashmaps) | `HashMap<K,V>`, Entry API, counting/grouping | Beginner | 01 |
| 03 | [Option & Nullability](challenges/03_option) | `Option<T>`, combinators, `let else`, why Rust has no null | Beginner | 01 |
| 04 | [Strings](challenges/04_strings) | `String` vs `&str`, parsing, UTF-8, case-insensitive compare | Beginner | 01 |
| 05 | [Structs, Enums & impl](challenges/05_structs_enums) | Struct/enum design, methods, associated functions, match | Beginner | 01 |
| 06 | [Control Flow as Expressions](challenges/06_control_flow) | `if`/`match` as expressions, range patterns, `let else` | Beginner | 01 |
| 07 | [Numbers & Conversions](challenges/07_numbers) | Integer overflow, `as`, `From`/`Into`, `TryFrom`/`TryInto`, float compare | Beginner | 01 |
| 08 | [Modules & Visibility](challenges/08_modules) | `mod`, `pub`, `use`, `super::`/`self::`/`crate::` | Beginner | 05 |
| 09 | [Ownership & Borrowing](challenges/09_borrowing) | Borrow checker, moves, lifetimes | Intermediate | 01 |
| 10 | [HashSets & Performance](challenges/10_hashsets_and_performance) | `HashSet`, set algebra, capacity, `retain()` | Intermediate | 02 |
| 11 | [derive](challenges/11_derive) | `#[derive]`, standard traits, manual impls | Intermediate | 05 |
| 12 | [Result & `?`](challenges/12_result_and_question_mark) | `Result<T,E>`, `?`, error enums, `From` for error conversion | Intermediate | 03, 11 |
| 13 | [Derive Ecosystem](challenges/13_derive_ecosystem) | Patterns behind serde, thiserror, clap | Advanced | 11, 12 |

## How Each Challenge Works

Every challenge has three standalone `.rs` files:

- **`concept.rs`** — Heavily commented explainer with tests. Read this first to learn the concept.
- **`skeleton.rs`** — Fill in the `todo!()` stubs to make the tests pass. This is the challenge.
- **`debug.rs`** — A program with bugs. Find and fix them. Some won't compile; some produce wrong results.

Plus two supports for when you need them:

- **`HINTS.md`** — Progressive hints, staged so you only reveal as much as you need: symptom, then location, then the fix.
- **`solution/`** — Reference solutions (`skeleton_solution.rs`, `debug_solution.rs`). Compare after your own attempt.

Each file compiles directly with `rustc`:

```bash
# Learn the concept (all tests pass)
rustc concept.rs --edition 2024 --test && ./concept

# Do the challenge (tests fail until you complete the TODOs)
rustc skeleton.rs --edition 2024 --test && ./skeleton

# Hunt the bugs (may not compile until you fix them)
rustc debug.rs --edition 2024 --test && ./debug
```

An HTML overview is available at [`challenges/index.html`](challenges/index.html), with a page per challenge. The pages are generated from the README files — after editing any README, regenerate them with `make html` (a standalone Rust tool, like everything else here).

### Running everything at once

The top-level [`Makefile`](Makefile) discovers every challenge and runs all its tests:

```bash
make test            # concept + solution tests across every challenge (everything passes)
make test-skeletons  # skeleton tests (expected to fail until you complete the TODOs)
make test-debug      # debug tests (expected to fail until you fix the bugs)

# Run one file in one challenge:
make CH=12_result_and_question_mark concept
make CH=12_result_and_question_mark debug
```

## Prerequisites

- **Rust** — Install via [rustup.rs](https://rustup.rs). Any recent stable version (1.85+) works.
- **A text editor** — That's it. No Cargo, no IDE plugins, no dependencies.
- **GNU make** (optional) — only needed for the bulk targets above.
