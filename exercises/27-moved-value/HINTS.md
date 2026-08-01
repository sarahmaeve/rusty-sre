# Hints

## 1 — Direction

Mark the last use of each field and ask which operation truly needs to own data.

## 2 — Localization

Follow the value at the first “moved here” label into the callee's parameter type.

## 3 — Mechanism

The type is not `Copy`; passing it by value transfers ownership. The original
binding cannot be used afterward even if the callee only reads it.
