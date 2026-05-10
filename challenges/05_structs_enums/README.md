# Challenge 05: Structs, Enums, and `impl`

Learn how to define types in Rust — the building blocks every later challenge uses.

## Goal

Earlier challenges defined structs and enums but never explained the *type design* itself. This challenge covers:

- Struct definition and field access
- Methods with `&self`, `&mut self`, `self` — and why the receiver matters
- Associated functions (`Self::new`)
- Enums with payload data — Rust's algebraic data types
- `match` on enums (exhaustive by default)
- Unit structs and tuple structs

No traits, no generics, no derive yet — those come in Challenge 10.

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `class Foo:` ... `def __init__(self, x):` | `struct Foo { x: i32 }` + `impl Foo { fn new(x: i32) -> Self { ... } }` |
| `def method(self):` (read only) | `fn method(&self) { ... }` |
| `def method(self):` (mutating) | `fn method(&mut self) { ... }` |
| `def method(self):` (consuming) | `fn method(self) { ... }` |
| `class Color(Enum):` ... | `enum Color { Red, Green, Blue }` |
| Dataclass with one of N shapes | `enum Shape { Circle { radius: f64 }, Rect { w: f64, h: f64 } }` |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 9 sections covering structs, enums, methods, and match |
| `skeleton.rs` | Incident state machine — fill in six TODOs |
| `debug.rs` | Buggy service registry — find and fix 4 bugs |
| `solution/debug_solution.rs` | Fixed version of `debug.rs` |

## How to Run

```bash
rustc concept.rs --edition 2024 --test && ./concept
rustc skeleton.rs --edition 2024 --test && ./skeleton
rustc debug.rs --edition 2024 --test && ./debug
cd solution && rustc debug_solution.rs --edition 2024 --test && ./debug_solution
```

Or, from the repo root:

```bash
make CH=05_structs_enums concept
make CH=05_structs_enums skeleton
make CH=05_structs_enums debug
make CH=05_structs_enums solution
```

## Skeleton Challenge: Incident State Machine

Six tasks model an incident lifecycle:

1. **Define the `State` enum** — `Open`, `Acknowledged { by }`, `Resolved { summary }`
2. **Define the `Incident` struct** — id, service, state
3. **`Incident::new`** — start in `State::Open`
4. **`acknowledge`** — Open → Acknowledged, error otherwise
5. **`resolve`** — Acknowledged → Resolved, error otherwise
6. **`state_label`** — short string for the current state

The state machine enforces the transition order: you can't resolve an Open incident, or acknowledge twice.

## Debug Challenge: Service Registry

Four bugs:

1. **Mutating via `&self`** (compile error) — `mark_degraded` is declared `&self` but assigns to `self.status`. Fix: `&mut self`.
2. **Non-exhaustive match** (compile error) — a `Degraded` variant was added to `Status` but `status_label`'s match wasn't updated.
3. **Shared counter that isn't shared** (runtime) — `Service::new` declares `next_id` *inside* the function, so it resets to 1 on every call. Every service ends up with id 1.
4. **Mutating a clone** (runtime) — `apply_status` takes `&mut self` but clones `self` into a local before mutating. The mutation is lost.

## Concepts Covered

1. Struct definition with named fields and struct-update syntax
2. The three method receivers and what each one allows
3. Associated functions and the `new` constructor convention
4. Enums with no data, with one field, and with multiple fields
5. `match` is exhaustive — adding a variant forces every match site to be updated
6. Pattern binding (`Variant { field }`) inside match arms
7. Methods on enums via `impl`
8. Unit structs and tuple structs
9. When to reach for an enum: "this can be one of N shapes" with payload data
