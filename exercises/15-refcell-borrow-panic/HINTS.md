# Hints

## 1 — Direction

Inventory active `Ref` and `RefMut` guards at the moment of panic.

## 2 — Localization

Inspect the update expression surrounding the callback. A guard can live until the
end of a statement or scope even after its last apparent use.

## 3 — Mechanism

The callback re-enters the same cell while an exclusive runtime borrow guard is
still alive, so `RefCell` enforces the conflict by panicking.
