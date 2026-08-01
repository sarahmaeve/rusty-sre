# Rusty SRE

Rusty SRE teaches engineers to read, debug, and reason about production Rust.
It targets Rust 1.97.1 and the 2024 edition.

This is not a syntax quiz and it is not an algorithm collection. The core
activity is diagnosis: reproduce an operational symptom, read the evidence,
locate the violated contract, make a bounded repair, and verify the result.

## What is here

- [`guides/`](guides/) contains runnable concept guides. They explain enough
  Rust to make unfamiliar code legible.
- [`crates/`](crates/) is a production-shaped Cargo workspace containing the
  systems used by the exercises.
- [`exercises/`](exercises/) contains 48 debugging exercises, staged hints,
  hard-mode cards, artifact-first diagnoses, and report-first investigations.
- [`compile-fail/`](compile-fail/) contains fourteen small crates whose compiler
  diagnostics are the initial evidence.
- [`solutions/`](solutions/) contains reversible patches and debriefs. It is
  deliberately separate from the exercise instructions.

## Start here

Verify the toolchain and known-good material:

```bash
rustc --version
make bootstrap
make test
make guides
```

Run an exercise by number:

```bash
make ex N=01
```

The command should fail. That failure is the starting evidence, not a broken
installation. Read [`exercises/01-silent-config-fallback/README.md`](exercises/01-silent-config-fallback/README.md),
inspect the relevant test, and work from the contract toward the source.

After attempting the repair, save or revert edits that overlap the reference patch
before applying it:

```bash
make ex N=01
make solution N=01       # apply the reference patch
make unsolution N=01     # restore the exercise
```

Use `make help` for the complete command list.

## Learning paths

The numbered exercises are the main path. The same defects can be revisited
through different evidence:

- **Guided debugging:** an incident description, reproduction, and staged
  hints.
- **Hard mode:** only the observable report and the failing command.
- **Diagnosis:** a compiler diagnostic, backtrace, hang report, or concurrency
  artifact must be interpreted before source inspection.
- **Report-first investigation:** incomplete operational reports require
  clarification, competing hypotheses, and deliberate evidence selection.

See [`STUDY_GUIDE.md`](STUDY_GUIDE.md) for a competency-based sequence.

## Working rules

1. Reproduce before editing.
2. Read the failing test as an executable contract.
3. Separate observation, inference, and assumption.
4. Prefer the smallest repair justified by the evidence.
5. Run neighboring tests after the focused test passes.
6. Treat compiler and Clippy diagnostics as evidence, not instructions to
   apply mechanically.
7. Open hints one stage at a time.
8. Read the solution patch and debrief only after making an attempt.

## Why this repository looks like a real Rust workspace

Large Rust projects commonly use a workspace of focused crates, keep binaries
thin, enforce invariants at boundaries, and make concurrency lifecycle
explicit. The structure here draws from rust-analyzer, ripgrep, Tokio,
Wasmtime, Cargo, and the Rust compiler itself. See
[`RECOMMENDED_READING.md`](RECOMMENDED_READING.md) for specific source links
and what to study in each project.

## Quality checks

```bash
make fmt
make lint
make test
make check
make verify
```

Exercise tests are ignored during the normal suite because the repository
intentionally contains their defects. `make verify` additionally checks that
each exercise fails in its buggy form and that every solution patch applies,
passes, and reverses cleanly.
