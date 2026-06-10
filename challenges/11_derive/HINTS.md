# Challenge 11: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Alert Pipeline

**Task 1 — Display for Severity.** A `match self { ... }` where every arm is
a `write!(f, "...")` with the uppercase string.

**Task 2 — Display for Alert.** One line:
`write!(f, "[{}] {}: {}", self.severity, self.service, self.message)` —
`{}` on `self.severity` uses Task 1's Display.

**Task 3 — PartialEq/Eq for Alert.** Compare `service`, `severity`, and
`message`; ignore the rest. `Eq` is a marker trait — `impl Eq for Alert {}`
with an empty body.

**Task 4 — Default for AlertConfig.** `impl Default` whose `default()`
returns `Self { ... }` with the five listed values.

**Task 5 — Ord/PartialOrd for Alert.** Reverse the severity comparison by
swapping sides — `other.severity.cmp(&self.severity)` — then chain
`.then_with(|| self.service.cmp(&other.service))`. `PartialOrd` just
delegates: `Some(self.cmp(other))`.

**Task 6 — dedup_alerts.** Track seen combinations in a
`HashSet<(String, Severity, String)>`; `insert` returns `true` only for new
tuples, so push the alert exactly when it does.

## Debug: On-Call Dashboard

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "the trait bound `IncidentKey: Ord` is not
  satisfied" (E0277) wherever it's used as a `BTreeMap` key.
- **Where:** the `IncidentKey` derive list.
- **Fix:** `BTreeMap` keys need a *total* order — `Ord`, not just
  `PartialOrd`. Add `Ord` to the derive (its prerequisites `Eq` and
  `PartialOrd` are already there).

### Bug 2

- **Symptom:** `test_incident_dedup` fails — 3 "unique" incidents where 1
  was expected.
- **Where:** `impl Hash for Incident`.
- **Fix:** The hash includes `id` and `timestamp`, but `PartialEq` ignores
  them. That breaks the contract `a == b ⇒ hash(a) == hash(b)`, so equal
  incidents land in different hash buckets and `HashSet` dedup silently
  fails. Hash exactly the fields `eq` compares: service, priority,
  description.

### Bug 3

- **Symptom:** Doesn't compile — "the trait `Copy` cannot be implemented
  for this type" (E0204), pointing at the `String` fields.
- **Where:** the `OnCallEngineer` derive list.
- **Fix:** `Copy` means "duplicate by memcpy", which heap-owning types like
  `String` can never do. Remove `Copy` (keep `Clone` for explicit copies).

### Bug 4

- **Symptom:** `test_display_format` fails — got `[Critical]`, expected
  `[CRITICAL]`.
- **Where:** `impl fmt::Display for Incident`.
- **Fix:** The format string uses `{:?}` (Debug) for the priority, which
  prints the variant name. Use `{}` to route through `Priority`'s `Display`
  impl, which produces the uppercase form.
