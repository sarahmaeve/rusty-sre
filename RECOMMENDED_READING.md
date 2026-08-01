# Recommended Reading and Source Inspirations

Rusty SRE uses primary documentation and mature Rust projects as its reference
points. Do not try to read every project linearly. Follow a question into the
smallest relevant module, test, or design document.

## Language and tooling

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — the
  primary learning text for ownership, enums, errors, smart pointers,
  concurrency, and async Rust.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) — compact,
  runnable examples for syntax and standard-library patterns.
- [The Rust Reference](https://doc.rust-lang.org/reference/) — exact language
  behavior when the Book deliberately simplifies.
- [Standard library documentation](https://doc.rust-lang.org/std/) — trait
  contracts and API details. Read the trait and method documentation before
  inferring behavior from a name.
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/) — workspaces,
  features, targets, dependency resolution, profiles, and build scripts.
- [rustc error index](https://doc.rust-lang.org/error_codes/error-index.html) —
  extended explanations for compiler error codes.
- [Clippy documentation](https://doc.rust-lang.org/clippy/) — correctness,
  suspicious, complexity, performance, and opt-in lint categories.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — naming,
  interoperability, documentation, predictability, and dependable APIs.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe boundaries,
  soundness, ownership internals, `Send`, and `Sync`. Use after the core guides.

## Large Rust projects

### rust-analyzer

- [Repository](https://github.com/rust-lang/rust-analyzer)
- [Architecture](https://rust-analyzer.github.io/book/contributing/architecture.html)

Study its many focused crates, explicit dependency direction, durable data
model, cancellation-aware analysis, and separation between compiler-like
internals and IDE-facing APIs.

### ripgrep

- [Repository](https://github.com/BurntSushi/ripgrep)
- [User guide](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md)

Study the thin CLI over reusable crates, streaming instead of collecting,
Unicode and binary-data boundaries, configuration precedence, parallel
directory traversal, and performance claims backed by benchmarks.

### Tokio and mini-redis

- [Tokio repository](https://github.com/tokio-rs/tokio)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Graceful shutdown](https://tokio.rs/tokio/topics/shutdown)
- [mini-redis](https://github.com/tokio-rs/mini-redis)

Study task ownership, bounded channels, cancellation, timeouts, graceful
shutdown, framing, and the separation between connection tasks and shared
state.

### Wasmtime

- [Repository](https://github.com/bytecodealliance/wasmtime)
- [Wasmtime guide](https://docs.wasmtime.dev/)

Study its workspace boundaries, feature surface, configuration validation,
resource limits, fuzzing, unsafe-code review burden, and defense-in-depth
approach to a security-sensitive runtime.

### Rust compiler and Cargo

- [Rust compiler repository](https://github.com/rust-lang/rust)
- [rustc development guide](https://rustc-dev-guide.rust-lang.org/)
- [Cargo repository](https://github.com/rust-lang/cargo)

Study diagnostic construction, staged computation, query boundaries, tests
for failure output, compatibility policy, and the way Cargo models packages,
targets, features, and resolution separately.

## Common ecosystem contracts

- [Serde](https://serde.rs/) — serialization derives, data models, defaults,
  field attributes, and format independence.
- [thiserror](https://docs.rs/thiserror/) — typed library errors and preserved
  sources.
- [clap](https://docs.rs/clap/) — declarative CLI contracts and validation.
- [tracing](https://docs.rs/tracing/) — structured, contextual diagnostics for
  asynchronous systems.
- [Tokio](https://docs.rs/tokio/) and
  [tokio-util](https://docs.rs/tokio-util/) — runtime and cancellation tools.

These crates are common, not mandatory. Exercises ask what contract the code
needs before asking which crate supplies it.
