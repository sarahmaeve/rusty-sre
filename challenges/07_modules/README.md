# Challenge 07: Modules and Visibility

Learn how to split Rust code across modules and control what each piece exposes.

## Goal

Up to this point every challenge has been a single file. Real Rust projects organize code into modules, and modules have a privacy system more careful than Python's "leading underscore is a hint."

This challenge teaches:

- `mod foo { ... }` for inline modules and `mod foo;` for "look in foo.rs"
- `pub`, `pub(crate)`, `pub(super)` — three levels of visibility
- `use` for bringing names into scope, plus `use ... as alias`
- `super::`, `self::`, `crate::` for relative paths
- Why field-level visibility on structs is its own thing
- The `#[cfg(test)] mod tests` pattern (which you've seen in every challenge)

**File layout note:** This challenge stays in a single file using inline `mod foo { ... }` blocks. The rules for visibility, paths, and `use` are identical to a real multi-file project; the only thing that changes is that `mod foo;` (with a semicolon) tells the compiler to look at `foo.rs` instead of expecting an inline body.

## Python → Rust Quick Reference

| Python | Rust |
|--------|------|
| `foo.py` + `bar.py` | `mod foo;` `mod bar;` |
| `__init__.py` | `mod.rs` (or named after the directory) |
| `from foo import bar` | `use foo::bar;` |
| `from . import foo` | `use self::foo;` (or `use crate::foo;`) |
| `from .. import foo` | `use super::foo;` |
| Leading underscore (convention) | No `pub` (enforced) |

## Files

| File | Purpose |
|------|---------|
| `concept.rs` | 7 sections covering modules, visibility, paths, and tests |
| `skeleton.rs` | Refactor a flat file into three inline modules |
| `debug.rs` | Buggy alert pipeline — find and fix 4 bugs |
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
make CH=07_modules concept
make CH=07_modules skeleton
make CH=07_modules debug
make CH=07_modules solution
```

## Skeleton Challenge: Refactor a Config Validator

The skeleton starts as a single flat namespace with three concerns mixed together: parsing, validation, and config assembly. Your job is to organize it into three inline modules:

- `mod parse { pub fn parse_port, pub fn parse_host }`
- `mod validate { pub fn validate_port, pub fn validate_host }`
- `mod load { pub fn load_config }` — uses items from the other two modules

The tests reach in via qualified paths (`parse::parse_port`, `validate::validate_port`, `load::load_config`), so the file won't compile until your refactor matches.

## Debug Challenge: Alert Pipeline

Four bugs across a `pipeline` module with three submodules (`parse`, `dedup`, `route`):

1. **Private struct field** (compile error) — `Alert.severity` is missing `pub`, so `dedup::is_critical` (a sibling module) can't read it.
2. **Private function** (compile error) — `dedup::count_unique` is missing `pub`, so external callers (the tests) can't reach it.
3. **Private module** (compile error) — `mod route` should be `pub mod route` so the routing functions are reachable from outside `pipeline`.
4. **Wrong path** (runtime) — `dedup::record_alert` calls a local stub `local_counter()` instead of the real `super::shared::next_id()`. Every alert gets id `0` instead of a fresh number.

## Concepts Covered

1. Inline modules (`mod foo { ... }`) and file-based modules (`mod foo;`)
2. The default-private rule and how `pub` exposes items
3. Field-level privacy on structs (the type and each field need their own `pub`)
4. Enum variants inherit the enum's visibility
5. `pub(crate)` and `pub(super)` for narrower exposure
6. The `super::`, `self::`, `crate::` path prefixes
7. `use` and `use ... as alias` for shorter names
8. The `#[cfg(test)] mod tests` pattern you've used all along
9. Why a child module can call its parent's private items via `super::`
