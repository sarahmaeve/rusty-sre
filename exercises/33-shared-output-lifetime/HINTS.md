# Hints

## 1 — Direction

For each return branch, name the input storage that owns the selected text.

## 2 — Localization

Compare the lifetime written on the return type with the lifetime promised by each
input parameter.

## 3 — Mechanism

The fallback receives an independent inferred lifetime, so the signature does not
prove that it remains valid for the output's declared lifetime.
