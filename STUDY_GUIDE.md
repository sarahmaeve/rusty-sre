# Study Guide: Making the Most of Your Prep Time

Rusty SRE has 8 challenges totalling roughly a workday's worth of focused practice. This guide helps you choose what to do first.

## If You Have 2 Hours

Skip the concept files; learn from the skeletons and debugs.

1. **Challenge 01 — Vectors** (30 min) — `Vec<T>`, slicing, retain. The Rust foundations every other challenge builds on.
2. **Challenge 02 — HashMaps** (30 min) — Entry API, counting/grouping. The Python `dict` analogue, with surprises.
3. **Challenge 05 — Option** (30 min) — `Some`/`None`, combinators, `let else`. The first place Rust feels different from Python.
4. **Challenge 04 — Result & `?`** (30 min) — Error propagation, custom error enums. The other half of "Rust's exception story."

After each, look at `solution/debug_solution.rs` to compare with your fix.

## If You Have 4 Hours

Start with concepts, then tackle the debugs.

**Hour 1: Read concepts**
- `challenges/01_vectors/concept.rs` — for slice/vec ergonomics
- `challenges/05_option/concept.rs` — to internalize `Option<T>` early
- `challenges/04_result_and_question_mark/concept.rs` — for `?` and error enums

**Hours 2–4: Skeletons and debugs**
1. Challenge 01 — skeleton (Vectors)
2. Challenge 05 — skeleton (Option)
3. Challenge 02 — debug (HashMaps)
4. Challenge 04 — debug (Result & `?`)
5. Challenge 03 — skeleton (derive)

## If You Have a Full Day (6–8 Hours)

**Morning: Concepts + foundations** (2–3 hours)
Read concepts in order: 01 → 02 → 03 → 04 → 05. Each is heavily commented and ends with passing tests, so running them is part of the read.

**Afternoon: Skeletons and debugs** (3–4 hours)
1. Challenge 01 — skeleton + debug
2. Challenge 02 — skeleton + debug
3. Challenge 04 — skeleton + debug (this teaches you to design error enums)
4. Challenge 05 — debug (the runtime bugs are subtle SRE traps)

**Evening: Advanced ladder** (1–2 hours)
- Challenge 01a — Ownership & Borrowing
- Challenge 02a — HashSets & Performance
- Challenge 03a — Derive Ecosystem (manual impls behind serde/thiserror/clap)

## If You Have Multiple Days

Work through everything in the table below, in order. The numbering reflects difficulty within each topic family.

| # | Challenge | Difficulty | Why this order |
|---|-----------|-----------|----------------|
| 01 | Vectors | Beginner | Foundation — slices, indexing, iteration |
| 02 | HashMaps | Beginner | Counting/grouping, the Python `dict` analogue |
| 05 | Option | Beginner | Pull this in early — touches every later challenge |
| 04 | Result & `?` | Intermediate | Error propagation; companion to Option |
| 01a | Ownership & Borrowing | Intermediate | Builds on 01 with the borrow checker |
| 02a | HashSets & Perf | Intermediate | Builds on 02 with set algebra and capacity |
| 03 | derive | Intermediate | Sets up the trait vocabulary for production code |
| 03a | Derive Ecosystem | Advanced | Manual impls behind serde / thiserror / clap |

## Key Skills by SRE Area

| Interview / on-call focus | Most relevant challenges |
|---------------------------|--------------------------|
| Reading unfamiliar Rust | All `concept.rs` files |
| Compile-error literacy | 01, 01a, 04, 05 (debug) |
| Designing error types | 04, 03a |
| Choosing the right collection | 01, 02, 02a |
| Avoiding panics in production code | 04, 05 |
| Trait-based abstractions | 03, 03a |
| Python-to-Rust intuition | All READMEs (each has a comparison table) |

## Concept-to-Challenge Prerequisite Map

| Challenge | Read first |
|-----------|-----------|
| 01 | — (start here) |
| 01a | 01 |
| 02 | 01 |
| 02a | 02 |
| 03 | 02 |
| 03a | 03 |
| 04 | 03 (uses derive for error enums in the concept) |
| 05 | 01 (uses Vec and HashMap throughout examples) |

## Running Everything

From the repo root:

```bash
make test            # run every concept + solution test
make test-debug      # see which debug bugs are still unfixed
make test-skeletons  # see which skeleton TODOs are still empty
make CH=04_result_and_question_mark concept   # run one file
```

See [`README.md`](README.md) for a per-challenge file-layout reference.
