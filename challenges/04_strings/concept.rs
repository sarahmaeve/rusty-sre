// =============================================================================
// Challenge 04: Strings — String vs &str — Concept Explainer
// =============================================================================
//
// Rust has two main string types and a third you'll see often:
//
//   String   — owned, growable, heap-allocated. Like Python's `str`, but
//              you own it and can mutate it.
//   &str     — a borrowed slice into some string data. The most common
//              parameter type. String literals like "hello" are &'static str.
//   &String  — a reference to an owned String. You almost never want this
//              as a parameter — take &str instead so callers can pass either.
//
// This challenge teaches when to use which, how to convert between them,
// and the most common Beginner traps (especially byte-indexing pitfalls).
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   s = "hello"                  →  let s: &str = "hello";
//   s = "hello".lower()          →  let s = "hello".to_lowercase();
//   s += " world"                →  s.push_str(" world");  // for String
//   s = s + " world"             →  let s = format!("{s} world");
//   s.startswith("h")            →  s.starts_with("h")
//   int(s)                       →  s.parse::<i32>()  // returns Result
//   s[0]                         →  s.chars().next()  // not s[0]! see §5

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. STRING LITERALS ARE &str
// =============================================================================
// A literal in source code has type &'static str — a borrowed slice into
// the read-only program text. To get an owned String, you have to ask.

#[test]
fn literals_are_str() {
    let literal: &str = "hello";
    assert_eq!(literal.len(), 5);
    assert_eq!(literal, "hello");

    // Three equivalent ways to make an owned String from a literal:
    let owned1: String = String::from("hello");
    let owned2: String = "hello".to_string();
    let owned3: String = "hello".to_owned();

    assert_eq!(owned1, owned2);
    assert_eq!(owned2, owned3);
    assert_eq!(owned1, "hello"); // String == &str works as you'd expect
}

// =============================================================================
// 2. PARAMETERS: PREFER &str OVER String OR &String
// =============================================================================
// A function that takes &str accepts both String and &str (and &String, via
// auto-deref). A function that takes String forces every caller to give up
// ownership — usually overkill.

fn shout(text: &str) -> String {
    text.to_uppercase()
}

#[test]
fn flexible_parameters() {
    let owned: String = String::from("hello");
    let borrowed: &str = "world";

    // Both work, no copying:
    assert_eq!(shout(&owned), "HELLO"); // &String coerces to &str
    assert_eq!(shout(borrowed), "WORLD");

    // owned is still usable here:
    assert_eq!(owned, "hello");
}

// =============================================================================
// 3. GROWING A STRING: push_str, push, +, format!
// =============================================================================

#[test]
fn growing_strings() {
    // push_str(&str) — appends a slice
    let mut s = String::from("hello");
    s.push_str(", world");
    assert_eq!(s, "hello, world");

    // push(char) — appends a single character
    s.push('!');
    assert_eq!(s, "hello, world!");

    // The `+` operator: takes a String on the LEFT and a &str on the RIGHT.
    // It also CONSUMES the left operand — `s` is moved.
    let combined = String::from("foo") + "bar";
    assert_eq!(combined, "foobar");

    // format! — the safest, most flexible way to build strings. Like Python's
    // f-strings. Doesn't consume any of its inputs.
    let name = "alice";
    let count = 3;
    let line = format!("user={name} attempts={count}");
    assert_eq!(line, "user=alice attempts=3");
}

// =============================================================================
// 4. PARSING STRINGS INTO TYPED VALUES
// =============================================================================
// `.parse::<T>()` returns Result<T, _>. Combine with `?` or `.ok()` as needed.

#[test]
fn parsing() {
    let port_text = "8080";
    let port: u16 = port_text.parse().unwrap();
    assert_eq!(port, 8080);

    let bad = "eighty";
    let result: Result<u16, _> = bad.parse();
    assert!(result.is_err());

    // .parse() works for any T that implements FromStr:
    let pi: f64 = "3.14159".parse().unwrap();
    assert!((pi - std::f64::consts::PI).abs() < 0.001);
}

// =============================================================================
// 5. BYTE-INDEXING IS A TRAP — USE chars()
// =============================================================================
// Rust strings are UTF-8 encoded. A "character" can take 1–4 bytes. Indexing
// `s[i]` would have to either return a byte (confusing) or scan from the
// start (slow), so Rust simply forbids `s[i]` and forces you to choose:
//
//   s.chars()                  — yields Unicode characters (slow random
//                                access, but correct)
//   s.bytes()                  — yields u8 bytes (fast, but only sensible
//                                for ASCII)
//   s.as_bytes()[i]            — direct byte access (use only with ASCII)
//   s.get(start..end)          — Option<&str> for a slice; None if the
//                                range falls between chars

#[test]
fn iterating_characters() {
    let s = "café";
    assert_eq!(s.len(), 5); // 5 BYTES — `é` is two bytes
    assert_eq!(s.chars().count(), 4); // 4 characters

    let chars: Vec<char> = s.chars().collect();
    assert_eq!(chars, vec!['c', 'a', 'f', 'é']);

    // The byte-indexing trap: this would PANIC at runtime if uncommented:
    //   let _ = &s[0..3]; // OK: slices on char boundaries
    //   let _ = &s[0..4]; // PANICS: cuts `é` in half

    // The safe form returns Option:
    assert_eq!(s.get(0..3), Some("caf"));
    assert_eq!(s.get(0..4), None); // mid-character slice → None

    // chars().nth(i) is the right way to get the "i-th character":
    assert_eq!(s.chars().nth(3), Some('é'));
}

// =============================================================================
// 6. CASE-INSENSITIVE COMPARISON
// =============================================================================
// `s1 == s2` is byte-exact. To compare without case, lowercase both sides
// (or uppercase both — pick one). Don't lowercase only one side.

#[test]
fn case_insensitive() {
    let a = "ServiceA";
    let b = "servicea";

    assert_ne!(a, b); // byte-exact comparison fails

    assert_eq!(a.to_lowercase(), b.to_lowercase());
    assert!(a.eq_ignore_ascii_case(b)); // ASCII-only, but allocation-free
}

// =============================================================================
// 7. SPLITTING, JOINING, AND COMMON PARSE HELPERS
// =============================================================================

#[test]
fn splitting_and_joining() {
    let line = "auth,payments,search";

    let services: Vec<&str> = line.split(',').collect();
    assert_eq!(services, vec!["auth", "payments", "search"]);

    let rejoined = services.join(":");
    assert_eq!(rejoined, "auth:payments:search");

    // split_whitespace: handles any run of whitespace, no empty entries
    let words: Vec<&str> = "  hello   world  ".split_whitespace().collect();
    assert_eq!(words, vec!["hello", "world"]);

    // splitn — limit the number of splits (handy for "key=value" with =-in-value)
    let kv: Vec<&str> = "name=alice=test".splitn(2, '=').collect();
    assert_eq!(kv, vec!["name", "alice=test"]);

    // split_once — get exactly the first split as a tuple
    let pair = "key=value".split_once('=');
    assert_eq!(pair, Some(("key", "value")));
}

// =============================================================================
// 8. TRIMMING AND CHECKING PREFIXES/SUFFIXES
// =============================================================================

#[test]
fn trimming_and_prefixes() {
    assert_eq!("  hi  ".trim(), "hi");
    assert_eq!("  hi  ".trim_start(), "hi  ");
    assert_eq!("  hi  ".trim_end(), "  hi");

    // Trim specific characters:
    assert_eq!("...hello...".trim_matches('.'), "hello");

    // Prefix/suffix checks return bool:
    assert!("https://example.com".starts_with("https://"));
    assert!("file.log".ends_with(".log"));

    // strip_prefix / strip_suffix return Option<&str> — useful for parsing:
    assert_eq!("v1.2.3".strip_prefix("v"), Some("1.2.3"));
    assert_eq!("v1.2.3".strip_prefix("x"), None);
}

// =============================================================================
// 9. RETURNING A &str FROM A FUNCTION
// =============================================================================
// A function can return &str if the slice is borrowed from a longer-lived
// source (an input parameter, a `&'static str`, etc.) — but it cannot return
// a slice into a String it just allocated. That would dangle, so the
// compiler refuses.
//
// In practice: take &str, return &str — or return String when you need to
// own the result.

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn first_word_owned(s: &str) -> String {
    first_word(s).to_string()
}

#[test]
fn returning_slices() {
    let line = String::from("hello world");
    assert_eq!(first_word(&line), "hello");
    assert_eq!(first_word_owned(&line), String::from("hello"));
}

// =============================================================================
// 10. WHEN TO CLONE
// =============================================================================
// Cloning a String allocates and copies the whole buffer. Don't clone when
// a borrow would do — borrowing is free. Reach for .clone() only when the
// caller needs an independent owned copy.

#[test]
fn clone_when_needed() {
    let original = String::from("hello");

    // Wasteful: storing in two variables, but neither is mutated.
    // The right move here is to just use a reference to `original`.
    let copy = original.clone();
    assert_eq!(original, copy);
    // Clone allocates — fine here, but in a hot loop it would matter.
}
