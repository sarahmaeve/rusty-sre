# Study Guide: Making the Most of Your Prep Time

Rusty SRE has 13 challenges totalling roughly two workdays of focused practice. This guide helps you choose what to do first.

The numbering follows a Beginner → Intermediate → Advanced ladder — walking 01 → 13 in order matches a sensible learning path.

## If You Have 2 Hours

Skip the concept files; learn from the skeletons and debugs.

1. **Challenge 01 — Vectors** (30 min) — `Vec<T>`, slicing, retain. The Rust foundations every other challenge builds on.
2. **Challenge 02 — HashMaps** (30 min) — Entry API, counting/grouping. The Python `dict` analogue, with surprises.
3. **Challenge 03 — Option** (30 min) — `Some`/`None`, combinators, `let else`. The first place Rust feels different from Python.
4. **Challenge 04 — Strings** (30 min) — `String` vs `&str`, byte-vs-character, case-insensitive compare. Everyone trips here on day one.

After each, look at `solution/debug_solution.rs` to compare with your fix.

## If You Have 4 Hours

Start with concepts, then tackle the debugs.

**Hour 1: Read concepts**
- `challenges/01_vectors/concept.rs` — for slice/vec ergonomics
- `challenges/03_option/concept.rs` — to internalize `Option<T>` early
- `challenges/04_strings/concept.rs` — for `String` vs `&str`
- `challenges/07_numbers/concept.rs` — for integer overflow and casts

**Hours 2–4: Skeletons and debugs**
1. Challenge 01 — skeleton (Vectors)
2. Challenge 04 — skeleton (Strings)
3. Challenge 05 — skeleton (Structs, Enums)
4. Challenge 07 — debug (Numbers — the SRE-flavoured overflow trap)
5. Challenge 06 — debug (Control Flow)

## If You Have a Full Day (6–8 Hours)

**Morning: Beginner shelf** (3–4 hours)
Read concepts and do the skeletons + debugs for 01 → 08 in order. They're sized so each challenge is roughly 30–45 minutes end to end.

**Afternoon: Intermediate ladder** (3–4 hours)
1. Challenge 09 — Ownership & Borrowing
2. Challenge 10 — HashSets & Performance
3. Challenge 11 — derive
4. Challenge 12 — Result & `?`

**Evening: Advanced** (1 hour)
- Challenge 13 — Derive Ecosystem (manual impls behind serde/thiserror/clap)

## If You Have Multiple Days

Work through everything in order.

| # | Challenge | Difficulty | Why this order |
|---|-----------|-----------|----------------|
| 01 | Vectors | Beginner | Foundation — slices, indexing, iteration |
| 02 | HashMaps | Beginner | Counting/grouping, the Python `dict` analogue |
| 03 | Option | Beginner | Pull this in early — touches every later challenge |
| 04 | Strings | Beginner | The "two strings" surprise, byte/char trap |
| 05 | Structs, Enums & impl | Beginner | Type design — used by every later challenge |
| 06 | Control Flow as Expressions | Beginner | Reframes how you write Rust |
| 07 | Numbers & Conversions | Beginner | Integer overflow, casts, float compare — Python-vs-Rust traps |
| 08 | Modules & Visibility | Beginner | How real projects organize code |
| 09 | Ownership & Borrowing | Intermediate | Builds on 01 with the borrow checker |
| 10 | HashSets & Perf | Intermediate | Builds on 02 with set algebra and capacity |
| 11 | derive | Intermediate | Sets up the trait vocabulary for production code |
| 12 | Result & `?` | Intermediate | Error propagation; companion to Option |
| 13 | Derive Ecosystem | Advanced | Manual impls behind serde / thiserror / clap |

## Key Skills by SRE Area

| Interview / on-call focus | Most relevant challenges |
|---------------------------|--------------------------|
| Reading unfamiliar Rust | All `concept.rs` files |
| Compile-error literacy | 04, 05, 06, 08, 09, 12 (debug) |
| Designing types | 05, 12, 13 |
| Choosing the right collection | 01, 02, 10 |
| Avoiding panics in production code | 03, 07, 12 |
| String handling | 04 |
| Numeric correctness (overflow, casts) | 07 |
| Trait-based abstractions | 11, 13 |
| Code organization | 08 |
| Python-to-Rust intuition | All READMEs (each has a comparison table) |

## Concept-to-Challenge Prerequisite Map

| Challenge | Read first |
|-----------|-----------|
| 01 | — (start here) |
| 02 | 01 |
| 03 | 01 (uses Vec and HashMap throughout examples) |
| 04 | 01 |
| 05 | 01 |
| 06 | 01 |
| 07 | 01 |
| 08 | 05 (uses structs and enums in examples) |
| 09 | 01 |
| 10 | 02 |
| 11 | 05 (extends the type-design vocabulary) |
| 12 | 03, 11 (uses derive in the error-enum concept) |
| 13 | 11, 12 |

## Running Everything

From the repo root:

```bash
make test            # run every concept + solution test
make test-debug      # see which debug bugs are still unfixed
make test-skeletons  # see which skeleton TODOs are still empty
make CH=07_numbers concept   # run one file
```

See [`README.md`](README.md) for a per-challenge file-layout reference.
