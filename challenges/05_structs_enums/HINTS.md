# Challenge 05: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Incident State Machine

**Task 4 — acknowledge.** You take `&mut self`, check that `self.state` is
`State::Open`, and replace `self.state` if the transition is valid;
otherwise return `Err`.

**Task 5 — resolve.** Same shape as Task 4, but the only valid starting
state is `Acknowledged { .. }` — `matches!(self.state, State::Acknowledged { .. })`
is a tidy way to test a variant you don't need the payload of.

**Task 6 — state_label.** A three-arm `match` on `self.state`; use `{ .. }`
to ignore variant payloads.

## Debug: Service Registry

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "cannot assign to `self.status`, which is
  behind a `&` reference" (E0594).
- **Where:** `Service::mark_degraded`.
- **Fix:** The receiver is `&self`, an immutable borrow, so the body can't
  assign to any field. A method that mutates needs `&mut self` — and its
  callers need a `mut` binding.

### Bug 2

- **Symptom:** Doesn't compile — "non-exhaustive patterns:
  `&Status::Degraded { .. }` not covered" (E0004).
- **Where:** `status_label`.
- **Fix:** `match` on an enum must cover every variant — that's the
  point: adding `Degraded` to `Status` makes the compiler flag every match
  that hasn't caught up. Add a `Status::Degraded { .. } => "degraded"` arm.

### Bug 3

- **Symptom:** `services_get_unique_ids` fails — every service has id 1.
- **Where:** `Service::new`.
- **Fix:** `next_id` is declared *inside* the function, so it's recreated at
  0 and bumped to 1 on every call — nothing persists between calls. The
  counter has to live outside the function: a
  `static NEXT_ID: AtomicU32 = AtomicU32::new(1);` at module level, bumped
  with `NEXT_ID.fetch_add(1, Ordering::Relaxed)`, gives each call a fresh id.

### Bug 4

- **Symptom:** `apply_status_updates_in_place` fails — the service is still
  `Healthy` after applying `Down`.
- **Where:** `Service::apply_status`.
- **Fix:** The body clones `self` into a local, mutates the *clone*, and
  drops it — the caller's instance never changes. Delete the clone and
  assign directly: `self.status = status;`. (In Python every name is a
  reference so this pattern would work; in Rust `clone()` is a real copy.)
