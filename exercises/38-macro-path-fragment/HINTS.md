# Hints

## 1 — Direction

The diagnostic points to token matching, not to the `matches!` expansion.

## 2 — Localization

Compare the grammar accepted by an `ident` fragment with the two identifiers and
separator in a qualified variant.

## 3 — Mechanism

An identifier fragment consumes one identifier token. A qualified enum variant is
a path, so tokens remain when the matcher expects the invocation to end.
