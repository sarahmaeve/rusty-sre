# List of concepts or challenges

1. Use of Rust vectors and vectors versus other data structures
1a. Ownership and borrowing with vectors
2. Text processing in Rust using HashMaps
02a. HashSet and performance comparisons
3. Introduction to derive in Rust
03a. The derive ecosystem — manual impls behind serde, thiserror, clap
4. Result and the `?` operator — propagating errors with custom error enums
5. Option and nullability — Some/None, combinators, and choosing safe defaults

## output directory structure

The output directories should be separate for each challenge or concept. It is fine to put the subchallenges in further subdirectories if that increases clarity or prevents filename collisions.

The output directory needs a README.md that includes instructions on which files point to which type of challenge and how to run its tests.

The output directory should have a subdirectory called SOLUTION with the corrected code for the debug challenge. The SOLUTION for each directory must pass its unit tests as written.

Any data required for each challenge should be included.

Prefer Rust 2024 over any previous versions.
