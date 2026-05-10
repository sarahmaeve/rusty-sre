// =============================================================================
// Challenge 05: Option<T> and Nullability — Concept Explainer
// =============================================================================
//
// Option<T> is Rust's answer to "this might or might not be there." Unlike
// Python's None — which can sneak into any variable and surprise you at
// runtime — Rust forces nullability to be explicit at the type level:
//
//     enum Option<T> {
//         Some(T),
//         None,
//     }
//
// You cannot accidentally pass None where a T is expected. The compiler
// requires every Option to be unwrapped (safely or unsafely) before its
// inner value can be used.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   x = None                     →  let x: Option<T> = None;
//   if x is None:                →  if x.is_none() { ... }
//   if x is not None: use(x)     →  if let Some(v) = x { use(v) }
//   x or default                 →  x.unwrap_or(default)
//   x or compute()               →  x.unwrap_or_else(compute)
//   x.field if x else default    →  x.map(|v| v.field).unwrap_or(default)
//   raise KeyError if x is None  →  x.ok_or(MyError::Missing)?

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. CREATING AND CHECKING OPTIONS
// =============================================================================

#[test]
fn option_basics() {
    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;

    assert!(some.is_some());
    assert!(none.is_none());

    // Pattern matching is the explicit way to consume an Option:
    match some {
        Some(n) => assert_eq!(n, 42),
        None => panic!("should be Some"),
    }

    // The same with `if let` — handy when you only care about Some:
    if let Some(n) = some {
        assert_eq!(n, 42);
    }
}

// =============================================================================
// 2. `if let Some` AND `let else`
// =============================================================================
// `let else` (stable since Rust 1.65) is the cleanest way to early-return on
// None. You bind the inner value as a normal local; the else branch must
// diverge (return / break / continue / panic).

fn first_word_len(s: &str) -> Option<usize> {
    let Some(word) = s.split_whitespace().next() else {
        return None;
    };
    Some(word.len())
}

#[test]
fn let_else_pattern() {
    assert_eq!(first_word_len("hello world"), Some(5));
    assert_eq!(first_word_len(""), None);
}

// =============================================================================
// 3. UNWRAPPING — unwrap, expect, unwrap_or, unwrap_or_else, unwrap_or_default
// =============================================================================

#[test]
fn unwrapping_options() {
    // unwrap() — panics on None
    assert_eq!(Some(7).unwrap(), 7);

    // expect("msg") — panics on None with a custom message
    assert_eq!(Some(7).expect("hard-coded; cannot be None"), 7);

    // unwrap_or(default) — eager: `default` is built even on Some
    assert_eq!(Some(7).unwrap_or(0), 7);
    assert_eq!(None::<i32>.unwrap_or(0), 0);

    // unwrap_or_else(f) — lazy: f only runs on None
    let mut times_called = 0;
    let _ = Some(7).unwrap_or_else(|| {
        times_called += 1;
        0
    });
    assert_eq!(times_called, 0);
    let _ = None::<i32>.unwrap_or_else(|| {
        times_called += 1;
        0
    });
    assert_eq!(times_called, 1);

    // unwrap_or_default() — uses Default::default() for T
    assert_eq!(None::<String>.unwrap_or_default(), "");
    assert_eq!(None::<Vec<i32>>.unwrap_or_default(), Vec::<i32>::new());
}

// =============================================================================
// 4. COMBINATORS — map, and_then, or, or_else, filter
// =============================================================================
// These let you chain transformations without pattern matching.

#[test]
fn option_combinators() {
    // map: transform inside Some, None passes through
    assert_eq!(Some(5).map(|n| n * 2), Some(10));
    assert_eq!(None::<i32>.map(|n| n * 2), None);

    // and_then: chain a function that itself returns Option (a.k.a. flat-map)
    fn parse_pos(s: &str) -> Option<i32> {
        let n: i32 = s.parse().ok()?;
        if n > 0 { Some(n) } else { None }
    }
    assert_eq!(Some("10").and_then(parse_pos), Some(10));
    assert_eq!(Some("nope").and_then(parse_pos), None);

    // or / or_else: fallback to another Option
    assert_eq!(None::<i32>.or(Some(0)), Some(0));
    assert_eq!(Some(5).or(Some(0)), Some(5)); // first Some wins
    assert_eq!(None::<i32>.or_else(|| Some(99)), Some(99));

    // filter: turn Some into None if the predicate is false
    assert_eq!(Some(5).filter(|&n| n > 0), Some(5));
    assert_eq!(Some(-3).filter(|&n| n > 0), None);
}

// =============================================================================
// 5. THE `?` OPERATOR ON OPTION
// =============================================================================
// `?` works on Option in functions whose return type is also Option. It
// returns None early if the operand is None. Same shape as `?` on Result.

fn get_first_two_chars(s: &str) -> Option<(char, char)> {
    let mut chars = s.chars();
    let a = chars.next()?; // None if string is empty
    let b = chars.next()?; // None if only one char
    Some((a, b))
}

#[test]
fn question_mark_on_option() {
    assert_eq!(get_first_two_chars("hello"), Some(('h', 'e')));
    assert_eq!(get_first_two_chars(""), None);
    assert_eq!(get_first_two_chars("x"), None);
}

// =============================================================================
// 6. CONVERTING BETWEEN Option AND Result
// =============================================================================
// Use ok_or / ok_or_else when you want to feed an Option into a `?`-style
// pipeline that returns Result. Use Result::ok() / err() for the reverse.

#[derive(Debug, PartialEq)]
enum LookupError {
    Missing(&'static str),
}

fn lookup_required(
    map: &std::collections::HashMap<&str, i32>,
    key: &'static str,
) -> Result<i32, LookupError> {
    let v = map.get(key).copied().ok_or(LookupError::Missing(key))?;
    Ok(v * 2)
}

#[test]
fn option_to_result() {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert("count", 21);

    assert_eq!(lookup_required(&m, "count"), Ok(42));
    assert_eq!(lookup_required(&m, "missing"), Err(LookupError::Missing("missing")));
}

// =============================================================================
// 7. take / replace / get_or_insert_with
// =============================================================================
// Sometimes you need to remove or swap the inner value of an Option in place.

#[test]
fn taking_and_replacing() {
    let mut slot: Option<String> = Some("hello".to_string());

    // take() — leaves None in place, returns the previous value
    let prev = slot.take();
    assert_eq!(prev, Some("hello".to_string()));
    assert_eq!(slot, None);

    // replace(v) — installs v, returns the previous value
    let prev = slot.replace("world".to_string());
    assert_eq!(prev, None);
    assert_eq!(slot, Some("world".to_string()));

    // get_or_insert_with(f) — if None, install f(); in either case return &mut T
    let mut empty: Option<Vec<i32>> = None;
    empty.get_or_insert_with(Vec::new).push(1);
    empty.get_or_insert_with(Vec::new).push(2);
    assert_eq!(empty, Some(vec![1, 2]));
}

// =============================================================================
// 8. ITERATOR METHODS THAT RETURN Option
// =============================================================================
// Many Iterator methods return Option because the result might not exist:
// next, find, position, max, max_by_key, min_by_key, last, nth, etc.

#[test]
fn iterators_and_option() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    assert_eq!(v.iter().max(), Some(&9));
    assert_eq!(v.iter().min(), Some(&1));
    assert_eq!(v.iter().find(|&&n| n > 4), Some(&5));
    assert_eq!(v.iter().position(|&n| n == 4), Some(2));

    let empty: Vec<i32> = vec![];
    assert_eq!(empty.iter().max(), None);
    assert_eq!(empty.first(), None);
    assert_eq!(empty.last(), None);
}

// =============================================================================
// 9. Option<&T> vs &Option<T>
// =============================================================================
// These are different types, and the compiler will not coerce between them.
//
//   Option<&T>   — "maybe a reference to a T"            (most common shape)
//   &Option<T>   — "a reference to an Option that might hold a T"
//
// .as_ref() turns &Option<T> into Option<&T>, which is usually what you
// want when looking at the inner value without moving it.

#[test]
fn as_ref_and_as_deref() {
    let owned: Option<String> = Some("hello".to_string());

    // .as_ref() turns &Option<String> into Option<&String>
    let borrowed: Option<&String> = owned.as_ref();
    assert_eq!(borrowed, Some(&"hello".to_string()));

    // .as_deref() goes one layer further: Option<String> → Option<&str>
    let s: Option<&str> = owned.as_deref();
    assert_eq!(s, Some("hello"));

    // owned is still usable because we only borrowed:
    assert_eq!(owned, Some("hello".to_string()));
}

// =============================================================================
// 10. WHY RUST HAS NO `null`
// =============================================================================
// Tony Hoare called null pointers his "billion-dollar mistake." Rust avoids
// the issue by:
//
//   - Not allowing null references at all. A &T always points to a real T.
//   - Forcing optionality to be expressed via Option<T> at the type level.
//   - Refusing to compile code that uses an Option as if it were T.
//
// In Python you can do `user.email.upper()` and get AttributeError if email
// is None. In Rust, the type system stops you at compile time:
//
//     let email: Option<String> = some_user.email.clone();
//     // email.to_uppercase();  // compile error — Option doesn't have that
//     let upper = email.map(|s| s.to_uppercase());  // OK — Option<String>
//
// This is the entire reason Option exists.

#[test]
fn null_safety_in_practice() {
    #[derive(Debug, Clone)]
    struct User {
        email: Option<String>,
    }

    let u = User { email: None };

    // Option<String> → Option<String> (uppercased) without panicking on None.
    let upper = u.email.clone().map(|s| s.to_uppercase());
    assert_eq!(upper, None);

    let u2 = User { email: Some("alice@example.com".into()) };
    let upper2 = u2.email.clone().map(|s| s.to_uppercase());
    assert_eq!(upper2, Some("ALICE@EXAMPLE.COM".to_string()));
}
