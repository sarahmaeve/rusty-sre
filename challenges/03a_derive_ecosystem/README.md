# Challenge 03a: Derive Ecosystem — Serialization, Error Handling, and CLI Parsing

## Goal

Learn the manual implementations behind Rust's three most-used derive crates: **serde** (serialization), **thiserror** (error types), and **clap** (CLI parsing). These crates are external and require cargo, so this challenge teaches the underlying traits they automate using only `std`.

When you later use `#[derive(Serialize)]`, `#[derive(Error)]`, or `#[derive(Parser)]` in production cargo projects, you'll understand exactly what they generate.

## Python → Rust Quick Reference

| Python | Rust (manual) | Rust (with crate) |
|--------|---------------|-------------------|
| `json.dumps(obj.__dict__)` | `impl Display` | `#[derive(Serialize)]` |
| `MyClass(**json.loads(s))` | `impl FromStr` | `#[derive(Deserialize)]` |
| `class MyError(Exception)` | `impl Error + Display` | `#[derive(thiserror::Error)]` |
| `raise X from Y` | `impl Error::source()` | `#[source]` attribute |
| `except X as e: raise Y(e)` | `impl From<X> for Y` | `#[from]` attribute |
| `argparse.ArgumentParser` | manual arg parsing | `#[derive(clap::Parser)]` |

## Files

### `concept.rs` — Commented Explainer
12 sections with ~28 tests covering serialization (Display/FromStr), error handling (Error/From/?), and argument parsing. Each section includes Python comparisons.

```bash
rustc concept.rs --edition 2021 --test && ./concept
```

### `skeleton.rs` — Incident Report Pipeline (YOUR CHALLENGE)
6 progressively harder tasks building an incident report system. Complete the `todo!()` stubs.

```bash
# Tests will fail until you complete the TODOs:
rustc skeleton.rs --edition 2021 --test && ./skeleton
```

**Tasks:**
1. Implement `Display` and `FromStr` for `Severity` (round-trip)
2. Implement `Display` for `IncidentReport` (logfmt serialization)
3. Implement `FromStr` for `IncidentReport` (logfmt deserialization)
4. Create a `ReportError` enum with `Display` and `Error`
5. Implement `From<String>` for `ReportError` (enabling `?` operator)
6. Build `parse_report_args` (CLI argument parser)

### `debug.rs` — SRE Log Exporter (BUG HUNT)
A deployment event export tool with 4 bugs. Find and fix all 4.

```bash
# Won't compile until bugs are fixed:
rustc debug.rs --edition 2021 --test && ./debug
```

**Bug types:**
- 2 compile-time errors (missing `From` impl, wrong `Err` type)
- 2 runtime errors (broken error chain, serialization mismatch)

### `solution/debug_solution.rs` — Fixed Version
Reference solution for `debug.rs` with all 4 bugs fixed.

```bash
cd solution && rustc debug_solution.rs --edition 2021 --test && ./debug_solution
```

## Concepts Covered

1. **Serialization** — `Display` as manual `Serialize` (logfmt format)
2. **Deserialization** — `FromStr` as manual `Deserialize`
3. **Round-trip correctness** — serialize then deserialize == original
4. **`std::error::Error` trait** — what `thiserror::Error` generates
5. **Error composition** — enum variants wrapping subsystem errors
6. **`From` conversions** — enabling the `?` operator across error types
7. **Error chaining** — `source()` for root cause analysis
8. **CLI argument parsing** — what `clap::Parser` generates
9. **Key=value argument styles** — `--flag value` vs `--flag=value`
10. **Batch processing** — combining parse, error handling, and reporting
