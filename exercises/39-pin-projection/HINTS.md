# Hints

## 1 — Direction

The program needs a shared read, not ownership of the inner worker.

## 2 — Localization

Inspect methods that turn a pinned owning pointer into a pinned shared reference,
then methods that expose a shared referent.

## 3 — Mechanism

`into_inner` removes the pinning wrapper and permits movement of the returned box's
value, so it is safe only when the pointee implements `Unpin`.
