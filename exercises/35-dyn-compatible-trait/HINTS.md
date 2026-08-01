# Hints

## 1 — Direction

List which methods the function actually needs to call through the trait object.

## 2 — Localization

The compiler identifies one method whose possible monomorphizations cannot occupy
one ordinary vtable slot.

## 3 — Mechanism

A method unavailable on unsized implementors does not need dynamic dispatch. Its
constraint can be stated independently of methods that remain callable through
`dyn Reporter`.
