// =============================================================================
// Challenge 07: Numbers and Conversions — Concept Explainer
// =============================================================================
//
// Python integers are arbitrary-precision and just work. Rust integers have
// fixed widths, can overflow, and won't silently widen. This challenge
// covers what to use, when to convert, and where the surprises lurk.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   x = 10_000_000_000_000   →  let x: i64 = 10_000_000_000_000;
//   x = int(s)               →  let x: i32 = s.parse()?;
//   x = int(y)               →  let x = y as i32;        (truncates!)
//                            →  let x: i32 = y.try_into()?;  (safe)
//   if a == b: ...           →  if (a - b).abs() < EPSILON { ... }  (for f64)
//   len(xs)                  →  xs.len()                  (returns usize)

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. INTEGER TYPES — WHEN TO USE WHICH
// =============================================================================
// Rust has many integer types. Pick the right one — guessing is a recipe
// for overflow surprises.
//
//   i8, i16, i32, i64, i128    — signed; range is ±2^(N-1)
//   u8, u16, u32, u64, u128    — unsigned; range is 0..2^N
//   isize / usize              — pointer-sized; isize = -2^(N-1)..=2^(N-1)-1,
//                                usize = 0..=2^N-1, where N = pointer bits
//
// The defaults if you don't pick:
//   integer literals → i32 (unless context forces another type)
//   xs.len()         → usize (always)
//   indexing         → usize (always)

#[test]
fn integer_types() {
    // Type inferred from context. The literal 100 is generic until used.
    let count: u32 = 100;
    assert_eq!(count, 100);

    // Suffix on literal also works:
    let big = 1_000_000_000_u64;
    assert_eq!(big, 1_000_000_000);

    // The default for a bare literal is i32:
    let bare = 42;
    assert_eq!(bare, 42_i32);

    // Underscores are visual separators for big literals:
    let bytes_per_gb: u64 = 1_073_741_824;
    assert_eq!(bytes_per_gb, 1024 * 1024 * 1024);

    // .len() always returns usize, regardless of element type:
    let v = vec![1u8, 2, 3];
    let len: usize = v.len();
    assert_eq!(len, 3);
}

// =============================================================================
// 2. OVERFLOW — DEBUG PANIC, RELEASE WRAP
// =============================================================================
// In debug builds, integer overflow PANICS. In release builds, it WRAPS
// silently. Both are usually wrong! For correctness-critical paths, use
// the explicit methods:
//
//   x.checked_add(y)       → Option<T>           (None on overflow)
//   x.wrapping_add(y)      → T                   (wraps; explicit)
//   x.saturating_add(y)    → T                   (clamps to T::MAX or T::MIN)
//   x.overflowing_add(y)   → (T, bool)           (wrapped result + did_overflow)
//
// All four exist for add / sub / mul / div / shl / shr / etc.

#[test]
fn overflow_handling() {
    let near_max: u8 = 250;

    // checked_add: Option<u8>
    assert_eq!(near_max.checked_add(5), Some(255));
    assert_eq!(near_max.checked_add(10), None); // would overflow → None

    // wrapping_add: never panics, never returns Option
    assert_eq!(near_max.wrapping_add(10), 4); // 260 wraps mod 256 → 4

    // saturating_add: never panics, clamps to type max
    assert_eq!(near_max.saturating_add(10), 255);
    assert_eq!(near_max.saturating_add(100), 255);

    // overflowing_add: returns (result, did_overflow)
    let (result, overflowed) = near_max.overflowing_add(10);
    assert_eq!(result, 4);
    assert!(overflowed);

    // For SRE counters, saturating_add is usually what you want. For
    // arithmetic where overflow is a logic error, prefer checked_add and
    // propagate the None as an error.
}

// =============================================================================
// 3. THE `as` KEYWORD — SHARP, SILENT, SOMETIMES WRONG
// =============================================================================
// `x as T` always succeeds. It does NOT check for truncation, sign loss, or
// precision loss. Use it only when you've reasoned about the conversion.
// For safe conversions, use From / TryFrom (next sections).

#[test]
fn as_casts_can_silently_truncate() {
    let big: i32 = 257;
    let small: u8 = big as u8;  // truncates: 257 mod 256 == 1
    assert_eq!(small, 1);       // probably not what you wanted

    // Negative-to-unsigned: bit-cast, not "0 or error"
    let negative: i32 = -1;
    let positive: u32 = negative as u32;
    assert_eq!(positive, u32::MAX); // 0xFFFFFFFF

    // Float-to-int: truncates toward zero, saturates at type bounds
    let f: f64 = 3.9;
    assert_eq!(f as i32, 3);
    let big_float: f64 = 1e20;
    assert_eq!(big_float as i32, i32::MAX); // saturates

    // The lesson: `as` is a hammer. Reach for it only when you've thought
    // about whether truncation / wrapping is fine.
}

// =============================================================================
// 4. From AND Into — LOSSLESS CONVERSIONS THE COMPILER VERIFIES
// =============================================================================
// `From<X> for Y` says "any X can become a Y, with no loss." The compiler
// only allows From impls where the conversion is provably lossless:
//
//   u8 → u16, u16 → u32, u32 → u64    (widening unsigned)
//   i8 → i16, i16 → i32, i32 → i64    (widening signed)
//   u8 → i16, u16 → i32, u32 → i64    (unsigned to wider signed)
//
// You can write either `Y::from(x)` or `x.into()`; they're the same call.

#[test]
fn lossless_conversions() {
    let small: u8 = 200;

    // u8 fits in u16, u32, u64 with no loss — these all compile:
    let medium: u16 = u16::from(small);
    let large: u32 = u32::from(small);
    let huge: u64 = u64::from(small);
    assert_eq!(medium, 200);
    assert_eq!(large, 200);
    assert_eq!(huge, 200);

    // .into() is the same operation, with the target type inferred:
    let medium2: u16 = small.into();
    assert_eq!(medium2, 200);

    // u8 is NOT From u16 — that would lose data on values >= 256.
    // The line below would fail to compile (commented out):
    //   let bad: u8 = u8::from(300_u16);
    // Use TryFrom instead — the next section.
}

// =============================================================================
// 5. TryFrom AND TryInto — FALLIBLE CONVERSIONS
// =============================================================================
// When loss IS possible, the conversion returns Result<T, _>. You decide
// what to do with the error.

use std::num::TryFromIntError;

#[test]
fn fallible_conversions() {
    // u16 → u8: succeeds when value fits, errors when it doesn't.
    let ok: Result<u8, TryFromIntError> = u8::try_from(200_u16);
    assert!(ok.is_ok());
    assert_eq!(ok.unwrap(), 200);

    let too_big: Result<u8, TryFromIntError> = u8::try_from(300_u16);
    assert!(too_big.is_err());

    // i32 → u32: errors on negative values.
    let positive: Result<u32, _> = u32::try_from(42_i32);
    assert_eq!(positive.unwrap(), 42);

    let negative: Result<u32, _> = u32::try_from(-1_i32);
    assert!(negative.is_err());

    // .try_into() is the same call, with the target inferred:
    let n: i64 = 1_000_000_000_000;
    let as_i32: Result<i32, _> = n.try_into();
    assert!(as_i32.is_err()); // doesn't fit in i32

    // Real SRE pattern: convert .len() (usize) to a u32 for a metric
    // payload, propagating overflow as an error.
    fn count_to_u32(v: &[u8]) -> Result<u32, TryFromIntError> {
        u32::try_from(v.len())
    }
    let small = vec![0_u8; 100];
    assert_eq!(count_to_u32(&small).unwrap(), 100);
}

// =============================================================================
// 6. usize VS u64 — WHY THE DIFFERENCE MATTERS
// =============================================================================
// usize is "the size of a pointer." On a 64-bit machine that's 64 bits;
// on a 32-bit machine it's 32 bits. Use usize for:
//
//   - collection lengths (.len() returns usize)
//   - indices into collections
//   - byte offsets into memory
//
// Use u64 (or i64) for:
//
//   - numeric quantities that aren't memory-related (timestamps, byte
//     counts that exceed memory size, durations)
//   - anything that needs to be the same on every platform

#[test]
fn usize_vs_u64() {
    let v: Vec<u8> = vec![0; 100];

    // .len() is usize — use try_from (or as) to get u64 for a metric:
    let len_usize: usize = v.len();
    let len_u64: u64 = u64::try_from(len_usize).unwrap();
    assert_eq!(len_u64, 100);

    // Going the other way — u64 to usize — also needs try_from in general,
    // because a u64 might not fit in usize on a 32-bit platform.
    let big_count: u64 = 50_000;
    let _idx: usize = usize::try_from(big_count).unwrap();
}

// =============================================================================
// 7. f64 EQUALITY IS A TRAP
// =============================================================================
// Floating-point arithmetic accumulates rounding error. Comparing two f64s
// with `==` is rarely what you want. Use a small epsilon:
//
//   (a - b).abs() < EPSILON
//
// Or for a relative tolerance, scale the epsilon by the magnitude.

#[test]
fn float_equality() {
    let a = 0.1_f64 + 0.2;
    let b = 0.3_f64;

    // Surprising but true:
    assert_ne!(a, b);  // 0.30000000000000004 != 0.3

    // The right way:
    const EPSILON: f64 = 1e-9;
    assert!((a - b).abs() < EPSILON);

    // f64 also has special values: NAN, INFINITY, NEG_INFINITY.
    // NAN is never equal to anything, including itself:
    let nan = f64::NAN;
    assert!(!(nan == nan));
    assert!(nan.is_nan()); // use .is_nan() to detect

    // For sorting, f64 doesn't implement Ord (because of NAN). Use
    // partial_cmp or total_cmp:
    let mut vals = vec![3.1, 1.4, 2.7];
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vals, vec![1.4, 2.7, 3.1]);
}

// =============================================================================
// 8. ARITHMETIC ON DIFFERENT INT TYPES — NO IMPLICIT WIDENING
// =============================================================================
// In Python, `a + b` quietly works regardless of types. In Rust, `a` and `b`
// must be the SAME type. There's no implicit widening — you have to convert
// explicitly. This catches a lot of bugs at compile time.

#[test]
fn no_implicit_widening() {
    let a: u32 = 100;
    let b: u64 = 200;

    // The line below would NOT compile — types don't match:
    //   let sum = a + b;
    //                 ^ expected `u32`, found `u64`

    // Convert one side. Widening is lossless here so .into() works:
    let sum: u64 = u64::from(a) + b;
    assert_eq!(sum, 300);

    // Or: convert b down to u32, but that's fallible:
    let sum: u32 = a + u32::try_from(b).unwrap();
    assert_eq!(sum, 300);
}

// =============================================================================
// 9. PARSING NUMBERS FROM STRINGS
// =============================================================================
// .parse::<T>() returns Result<T, _>. This already showed up in challenge 04;
// here it interacts with the integer types above.

#[test]
fn parsing_numbers() {
    // Parse a string into a specific integer type:
    let port: u16 = "8080".parse().unwrap();
    assert_eq!(port, 8080);

    // Parse fails on non-numeric, on overflow, AND on negative-into-unsigned:
    assert!("nope".parse::<u16>().is_err());
    assert!("99999".parse::<u16>().is_err());  // overflows u16
    assert!("-1".parse::<u16>().is_err());     // negative into unsigned

    // f64 parsing:
    let pi: f64 = "3.14159".parse().unwrap();
    assert!((pi - 3.14159).abs() < 1e-9);
}

// =============================================================================
// 10. COMMON SRE PATTERNS
// =============================================================================
// A few patterns that come up in real metrics / counter / sizing code:

#[test]
fn sre_patterns() {
    // Saturating counter — never wraps, always reports the truth or "lots":
    fn increment_counter(current: u32, by: u32) -> u32 {
        current.saturating_add(by)
    }
    assert_eq!(increment_counter(u32::MAX - 5, 100), u32::MAX);

    // Byte-rate calculation with no precision loss:
    fn bytes_to_kb(bytes: u64) -> f64 {
        // u64 → f64 widens (might lose precision for huge values, but
        // safe for "kilobytes" arithmetic). Use as_f64 helpers for
        // explicit precision policy in real code.
        bytes as f64 / 1024.0
    }
    assert!((bytes_to_kb(2048) - 2.0).abs() < 1e-9);

    // Percent calculation that doesn't divide by zero:
    fn percent(numerator: u64, denominator: u64) -> Option<f64> {
        if denominator == 0 {
            None
        } else {
            Some(numerator as f64 / denominator as f64 * 100.0)
        }
    }
    assert_eq!(percent(50, 200), Some(25.0));
    assert_eq!(percent(0, 0), None);
}
