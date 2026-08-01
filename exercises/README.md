# Debugging exercises

These exercises teach Rust by asking you to diagnose a broken system. The code is
small enough to inspect, but the failure modes come from production Rust: hidden
fallbacks, ownership mistakes, invalid equality contracts, blocking async work,
and incomplete shutdown.

Run an exercise with:

```console
make ex N=01
```

Use the two-digit number shown in the directory name. Start from the failure,
write down a hypothesis, and gather evidence before editing code. `README.md`
states the observable symptom and contract. `HINTS.md` has three progressively
more specific clues. Solutions live outside this directory so opening an exercise
does not reveal the repair.

## Working method

1. Reproduce the symptom and save the exact output.
2. State what the program should guarantee.
3. Identify the smallest relevant boundary: parsing, state update, task lifetime,
   or type contract.
4. Change one cause, not every suspicious line.
5. Re-run the focused exercise and the broader suite.
6. Explain why the fix restores the contract and which regression test protects it.

Compiler diagnostics, warnings, logs, backtraces, and timing are evidence. They
are not instructions to apply mechanically. A passing test is also not proof that
shutdown, cancellation, resource use, or public contracts are correct.

## Progression

| Exercises | Primary skill |
| --- | --- |
| 01–06 | Data flow, defaults, ownership, and mutation |
| 07–14 | Type contracts, representation, boundaries, and errors |
| 15–18 | Runtime borrowing, shared ownership, locking, and cleanup |
| 19–26 | Async execution, bounded work, timeout, and cancellation |
| 27–32 | Reading and resolving compiler diagnostics |

If the named topic itself is too much of a clue, use [HARD_MODE.md](HARD_MODE.md).
For practice starting from raw evidence, use [diagnosis/](diagnosis/). For a
longer investigation with incomplete reports, use [wheel/](wheel/).

## Reference shelf

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust standard library](https://doc.rust-lang.org/std/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy lint documentation](https://rust-lang.github.io/rust-clippy/master/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Tokio source](https://github.com/tokio-rs/tokio)
- [rust-analyzer source](https://github.com/rust-lang/rust-analyzer)
- [Cargo source](https://github.com/rust-lang/cargo)
- [ripgrep source](https://github.com/BurntSushi/ripgrep)

Project links are inspiration, not required reading. Use them to see how mature
Rust code organizes errors, cancellation, task ownership, configuration, and
performance-sensitive boundaries.
