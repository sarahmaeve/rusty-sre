# Study Guide: Making the Most of Your Prep Time

Rusty SRE has 8 challenges totalling roughly a workday's worth of focused practice. This guide helps you choose what to do first.

The numbering follows a Beginner → Intermediate → Advanced ladder. Numbers 04–07 are reserved for upcoming Beginner challenges (strings, structs/enums, control flow, modules); for now there is a gap between 03 and 08 in the directory list.

## If You Have 2 Hours

Skip the concept files; learn from the skeletons and debugs.

1. **Challenge 01 — Vectors** (30 min) — `Vec<T>`, slicing, retain. The Rust foundations every other challenge builds on.
2. **Challenge 02 — HashMaps** (30 min) — Entry API, counting/grouping. The Python `dict` analogue, with surprises.
3. **Challenge 03 — Option** (30 min) — `Some`/`None`, combinators, `let else`. The first place Rust feels different from Python.
4. **Challenge 11 — Result & `?`** (30 min) — Error propagation, custom error enums. The other half of "Rust's exception story."

After each, look at `solution/debug_solution.rs` to compare with your fix.

## If You Have 4 Hours

Start with concepts, then tackle the debugs.

**Hour 1: Read concepts**
- `challenges/01_vectors/concept.rs` — for slice/vec ergonomics
- `challenges/03_option/concept.rs` — to internalize `Option<T>` early
- `challenges/11_result_and_question_mark/concept.rs` — for `?` and error enums

**Hours 2–4: Skeletons and debugs**
1. Challenge 01 — skeleton (Vectors)
2. Challenge 03 — skeleton (Option)
3. Challenge 02 — debug (HashMaps)
4. Challenge 11 — debug (Result & `?`)
5. Challenge 10 — skeleton (derive)

## If You Have a Full Day (6–8 Hours)

**Morning: Concepts + foundations** (2–3 hours)
Read concepts in order: 01 → 02 → 03 → 10 → 11. Each is heavily commented and ends with passing tests, so running them is part of the read.

**Afternoon: Skeletons and debugs** (3–4 hours)
1. Challenge 01 — skeleton + debug
2. Challenge 02 — skeleton + debug
3. Challenge 11 — skeleton + debug (this teaches you to design error enums)
4. Challenge 03 — debug (the runtime bugs are subtle SRE traps)

**Evening: Advanced ladder** (1–2 hours)
- Challenge 08 — Ownership & Borrowing
- Challenge 09 — HashSets & Performance
- Challenge 12 — Derive Ecosystem (manual impls behind serde/thiserror/clap)

## If You Have Multiple Days

Work through everything in the table below, in this order.

| # | Challenge | Difficulty | Why this order |
|---|-----------|-----------|----------------|
| 01 | Vectors | Beginner | Foundation — slices, indexing, iteration |
| 02 | HashMaps | Beginner | Counting/grouping, the Python `dict` analogue |
| 03 | Option | Beginner | Pull this in early — touches every later challenge |
| 08 | Ownership & Borrowing | Intermediate | Builds on 01 with the borrow checker |
| 09 | HashSets & Perf | Intermediate | Builds on 02 with set algebra and capacity |
| 10 | derive | Intermediate | Sets up the trait vocabulary for production code |
| 11 | Result & `?` | Intermediate | Error propagation; companion to Option |
| 12 | Derive Ecosystem | Advanced | Manual impls behind serde / thiserror / clap |

## Key Skills by SRE Area

| Interview / on-call focus | Most relevant challenges |
|---------------------------|--------------------------|
| Reading unfamiliar Rust | All `concept.rs` files |
| Compile-error literacy | 01, 03, 08, 11 (debug) |
| Designing error types | 11, 12 |
| Choosing the right collection | 01, 02, 09 |
| Avoiding panics in production code | 03, 11 |
| Trait-based abstractions | 10, 12 |
| Python-to-Rust intuition | All READMEs (each has a comparison table) |

## Concept-to-Challenge Prerequisite Map

| Challenge | Read first |
|-----------|-----------|
| 01 | — (start here) |
| 02 | 01 |
| 03 | 01 (uses Vec and HashMap throughout examples) |
| 08 | 01 |
| 09 | 02 |
| 10 | 02 |
| 11 | 03, 10 (uses derive in the error-enum concept) |
| 12 | 10, 11 |

## Running Everything

From the repo root:

```bash
make test            # run every concept + solution test
make test-debug      # see which debug bugs are still unfixed
make test-skeletons  # see which skeleton TODOs are still empty
make CH=11_result_and_question_mark concept   # run one file
```

See [`README.md`](README.md) for a per-challenge file-layout reference.
