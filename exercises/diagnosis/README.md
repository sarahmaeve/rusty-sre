# Artifact-first diagnosis

These scenarios begin with evidence rather than a named Rust concept. They train a
common debugging task: deciding what to inspect next when the first thing you have
is compiler output, a backtrace, a task trace, or one intermittent failure.

## Method

1. Read only `ARTIFACT.txt`.
2. Copy [WORKSHEET_TEMPLATE.md](WORKSHEET_TEMPLATE.md) into your notes.
3. Extract observations without interpreting them.
4. Form at least two hypotheses and name evidence that separates them.
5. Open the scenario README only after choosing the next command or inspection.
6. Finish the scenario worksheet before looking at an exercise hint or solution.

The artifacts are compact reproductions, not complete observability systems. Treat
timestamps as monotonic within one artifact and do not infer precision they do not
provide.

Useful references: [rustc diagnostic structure](https://rustc-dev-guide.rust-lang.org/diagnostics.html),
[`RUST_BACKTRACE`](https://doc.rust-lang.org/std/backtrace/index.html), Tokio's
[`tracing` guide](https://tokio.rs/tokio/topics/tracing), and Loom's
[source and examples](https://github.com/tokio-rs/loom).
