# Prerequisites

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`
- Cargo 1.97.1
- GNU Make
- Git, for applying and reversing solution patches

The pinned toolchain is declared in [`rust-toolchain.toml`](rust-toolchain.toml).
The repository uses the Rust 2024 edition.

Confirm the installation:

```bash
rustc --version
cargo --version
make bootstrap
make test
```

`make bootstrap` requires network access once. The remaining project commands use
Cargo's offline mode so later runs use the locked, cached dependencies.

No editor integration is required. rust-analyzer is strongly recommended
because production Rust work depends heavily on navigation, type information,
macro expansion, and diagnostics.
