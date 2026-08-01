# Hints

## 1 — Direction

Ask whether the caller is required to remain on the stack until the new thread
finishes.

## 2 — Localization

Inspect the bounds on `thread::spawn` and compare them one by one with the bounds
promised by `spawn_drop`.

## 3 — Mechanism

`T: Send` permits ownership transfer but still allows `T` to contain non-static
references. An unscoped thread may outlive those referents.
