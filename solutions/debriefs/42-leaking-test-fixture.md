# 42 — Give fixture cleanup an RAII owner

Patch: [42-fixture-guard.patch](../42-fixture-guard.patch)

## Contract

Temporary mode replacement is restored after normal completion and unwinding.
Nested overrides work, and overrides from different threads do not overlap.

## Root cause

Restoration was an ordinary statement after the callback. A panic bypassed it.

## Why the symptom follows

`catch_unwind` catches control flow after stack unwinding has already skipped the
linear teardown, leaving the static mode at its temporary value.

## Repair strategy

Store the prior value in a guard whose `Drop` implementation restores it. Track the
owning thread and nesting depth so the same thread can nest overrides while other
threads wait. Stack unwinding executes cleanup and releases the outer scope.

## Verification

Run `make ex N=42`. Verify normal return, callback panic, nested overrides, and two
threads attempting overlapping overrides.

## Tempting wrong fix

Catching the panic inside `with_mode` complicates payload propagation and still
duplicates cleanup paths; the resource lifetime already defines the right scope.

## References

[`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html),
[`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html), and
[`UnwindSafe`](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html).
