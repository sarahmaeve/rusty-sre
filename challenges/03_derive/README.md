# Challenge 03: `derive` — Traits, Automatic Implementations, and SRE Patterns

## Goal

Learn Rust's `#[derive]` mechanism for automatic trait implementations. Previous challenges used `#[derive(Debug, Clone, Hash, Eq, PartialEq)]` without explaining what derive *does*. This challenge teaches:

- How derive generates trait implementations at compile time (zero runtime cost)
- Every standard derivable trait and when to use each one
- When and how to write manual trait implementations
- Practical SRE patterns combining multiple traits

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `__repr__` | `Debug` (`{:?}`) |
| `__str__` | `Display` (`{}`) — manual impl only |
| `__eq__` | `PartialEq` + `Eq` |
| `__hash__` | `Hash` |
| `__lt__` / `total_ordering` | `PartialOrd` + `Ord` |
| `__init__` defaults | `Default` |
| `copy.deepcopy()` | `Clone` (`.clone()`) |
| *(no equivalent)* | `Copy` (implicit stack copy) |

## Files

### `concept.rs` — Commented Explainer
12 sections with ~28 tests covering every derivable trait, the mechanism itself, manual implementations, and practical combinations. Each section includes Python comparisons.

```bash
rustc concept.rs --edition 2021 --test && ./concept
```

### `skeleton.rs` — Alert Pipeline (YOUR CHALLENGE)
6 progressively harder tasks building an SRE alert dedup and routing pipeline. Complete the `todo!()` stubs.

```bash
# Tests will fail until you complete the TODOs:
rustc skeleton.rs --edition 2021 --test && ./skeleton
```

**Tasks:**
1. Implement `Display` for `Severity` — format as uppercase string
2. Implement `Display` for `Alert` — format as `[SEVERITY] service: message`
3. Implement custom `PartialEq`/`Eq` for `Alert` — ignore id and timestamp for dedup
4. Implement `Default` for `AlertConfig` — sensible SRE defaults
5. Implement `Ord`/`PartialOrd` for `Alert` — sort by severity desc, then service asc
6. Build `dedup_alerts` function — deduplicate alerts using a HashSet

### `debug.rs` — On-Call Dashboard (BUG HUNT)
An on-call dashboard with 4 bugs related to derive and trait implementations. Find and fix all 4.

```bash
# Won't compile until bugs are fixed:
rustc debug.rs --edition 2021 --test && ./debug
```

**Bug types:**
- 2 compile-time errors (derive violations)
- 2 runtime errors (trait contract violations)

### `solution/debug_solution.rs` — Fixed Version
Reference solution for `debug.rs` with all 4 bugs fixed.

```bash
cd solution && rustc debug_solution.rs --edition 2021 --test && ./debug_solution
```

## Concepts Covered

1. The derive mechanism (compile-time code generation)
2. `Debug` — developer-facing formatting
3. `Clone` and `Copy` — value duplication semantics
4. `PartialEq` and `Eq` — equality comparison
5. `PartialOrd` and `Ord` — ordering and sorting
6. `Default` — sensible default values
7. `Hash` — hashing for HashMap/HashSet keys
8. Trait dependency graph (Copy→Clone, Eq→PartialEq, Ord→PartialOrd+Eq)
9. Manual trait implementations (Display, custom equality, custom ordering)
10. The newtype pattern for type safety
