# Hints

## 1 — Direction

Ask which checks are contractual and which are allowed to disappear in optimized
builds.

## 2 — Localization

Inspect the assertion macro and addition separately under debug and release
overflow settings.

## 3 — Mechanism

`debug_assert!` is removed from optimized builds. Ordinary overflowing addition
then follows the profile's overflow-check setting instead of producing `None`.
