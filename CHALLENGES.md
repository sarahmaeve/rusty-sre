# List of concepts or challenges

Challenges are numbered to follow a Beginner → Intermediate → Advanced ladder. Numbers 04–07 are reserved for upcoming Beginner challenges (strings, structs/enums, control flow, modules).

Beginner shelf:
01. Use of Rust vectors and vectors versus other data structures
02. Text processing in Rust using HashMaps
03. Option and nullability — Some/None, combinators, and choosing safe defaults

Intermediate shelf:
08. Ownership and borrowing with vectors
09. HashSet and performance comparisons
10. Introduction to derive in Rust
11. Result and the `?` operator — propagating errors with custom error enums

Advanced shelf:
12. The derive ecosystem — manual impls behind serde, thiserror, clap

## output directory structure

The output directories should be separate for each challenge or concept. It is fine to put the subchallenges in further subdirectories if that increases clarity or prevents filename collisions.

The output directory needs a README.md that includes instructions on which files point to which type of challenge and how to run its tests.

The output directory should have a subdirectory called SOLUTION with the corrected code for the debug challenge. The SOLUTION for each directory must pass its unit tests as written.

Any data required for each challenge should be included.

Prefer Rust 2024 over any previous versions.
