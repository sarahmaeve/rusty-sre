# Challenge 08: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Refactor a Config Validator

- Wrap the moved functions in `mod parse { ... }` / `mod validate { ... }` /
  `mod load { ... }` blocks and mark each function `pub` — items are private
  by default, even to the tests.
- Inside `load`, the body of `load_config` calls `parse_*` and `validate_*`
  unqualified. `pub use super::parse::*;` and `pub use super::validate::*;`
  bring those names in (and re-export them).
- `Config` lives at the top level, so inside `load` you'll need
  `use super::Config;` (or write `super::Config` at the use sites).

## Debug: Alert Pipeline

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "field `severity` of struct `Alert` is
  private" (E0616).
- **Where:** `pipeline::parse::Alert`.
- **Fix:** Struct fields have their own visibility, separate from the
  struct's. Every other field is `pub`; `severity` is missing it, so the
  sibling module `dedup` can't read it. Add `pub` to the field.

### Bug 2

- **Symptom:** Doesn't compile — "function `count_unique` is private"
  (E0603).
- **Where:** `pipeline::dedup::count_unique`.
- **Fix:** Items are private to their module by default. The tests (and
  `main`) call it from outside `dedup`, so it needs `pub fn`.

### Bug 3

- **Symptom:** Doesn't compile — "module `route` is private" (E0603).
- **Where:** the `mod route` declaration inside `pipeline`.
- **Fix:** A private module is only visible to its parent. To make
  `pipeline::route::destination` reachable from outside `pipeline`, declare
  it `pub mod route`. (Note `destination` is already `pub` — both the
  module and the item need visibility.)

### Bug 4

- **Symptom:** `record_alert_uses_shared_counter` fails — every recorded
  alert has id 0.
- **Where:** `pipeline::dedup::record_alert`.
- **Fix:** The function calls the local stub `local_counter()` (always 0)
  instead of the real shared counter that lives in the sibling `shared`
  module. Path to it from inside `dedup`: `super::shared::next_id()`.
