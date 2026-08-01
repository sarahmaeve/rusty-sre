# Panic backtrace: callback re-entry

The failure is deterministic but only occurs when an update callback queries the
registry. Ordinary updates and ordinary reads pass.

## Contract

Callbacks may read the registry. An update either completes atomically or returns a
domain error; a documented callback path must not panic.

## Investigation

Walk from the first application frame at the bottom toward the panic. Record which
runtime borrow guard each frame could create and when it can be dropped. Check
whether the callback's documented re-entry is compatible with the update design.

Continue with `make ex N=15` after completing [WORKSHEET.md](WORKSHEET.md).

Reference: [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) and
[panic backtraces](https://doc.rust-lang.org/std/backtrace/index.html).
