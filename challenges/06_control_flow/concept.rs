// =============================================================================
// Challenge 06: Control Flow as Expressions — Concept Explainer
// =============================================================================
//
// In Rust, almost every block produces a value. `if`/`else`, `match`, `loop`,
// and even bare braced blocks are EXPRESSIONS — not statements. That's why
// you can write:
//
//     let x = if cond { 1 } else { 2 };
//     let y = match status { Status::Up => "up", Status::Down => "down" };
//     let z = loop { if done() { break 42; } };
//
// Once this clicks, your code gets shorter and the type errors get clearer.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   x = a if cond else b           →  let x = if cond { a } else { b };
//   match dict[key]:               →  let label = match status {
//      case "up": label = "u"          Status::Up => "u",
//      case "down": label = "d"        Status::Down => "d",
//                                  };
//   while True: ... break v        →  let v = loop { ... break val; };
//   for x in xs: if cond: break x  →  xs.iter().find(|&&x| cond(x))
//                                       (or use `for ... { if ... { break } }`)

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. `if` / `else` AS AN EXPRESSION
// =============================================================================
// The whole `if`/`else` produces a value. Both arms must produce the SAME
// type. Drop the trailing semicolon to "return" the value of the arm.

#[test]
fn if_as_expression() {
    let n = 7;
    let label = if n > 0 { "positive" } else { "non-positive" };
    assert_eq!(label, "positive");

    // Multi-line if/else still works:
    let category = if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    };
    assert_eq!(category, "positive");

    // `if` without `else` returns ()  — only useful for side effects.
    let mut counter = 0;
    if n > 0 {
        counter += 1;
    }
    assert_eq!(counter, 1);
}

// =============================================================================
// 2. THE TRAILING SEMICOLON CHANGES TYPES
// =============================================================================
// In Rust, `expr` is an expression. `expr;` is a STATEMENT that produces ().
// Forgetting (or adding) a trailing semicolon is the most common Beginner
// type-mismatch error.

#[test]
fn semicolons_matter() {
    fn double(n: i32) -> i32 {
        n * 2 // no semicolon — this is the return value
    }
    assert_eq!(double(5), 10);

    // With the semicolon, this would NOT compile:
    //   fn double(n: i32) -> i32 { n * 2; }
    //                              ^^^^^^^ statement, type ()
    //                                      but the function's body type is i32

    // Same trap inside if/else:
    let result = {
        let n = 3;
        n + 1 // no semicolon — block evaluates to 4
    };
    assert_eq!(result, 4);

    let unit_result: () = {
        let _ = 99; // semicolon — block evaluates to ()
    };
    assert_eq!(unit_result, ());
}

// =============================================================================
// 3. `match` AS AN EXPRESSION
// =============================================================================
// Same idea as `if`: the whole match produces a value. Every arm must
// produce the same type. Match is also exhaustive — see challenge 05.

#[test]
fn match_as_expression() {
    let code = 200u16;

    let category = match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirection",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown", // wildcard — required because u16 has more than 599
    };
    assert_eq!(category, "success");

    // Match arms can have multi-statement bodies — wrap in braces:
    let parity = match code {
        c if c % 2 == 0 => {
            let s = format!("even ({c})");
            s
        }
        _ => "odd".to_string(),
    };
    assert_eq!(parity, "even (200)");
}

// =============================================================================
// 4. RANGE PATTERNS AND GUARDS
// =============================================================================
// Match patterns can be:
//
//   literal:        42
//   range:          0..10        (exclusive end)
//   inclusive:      0..=9        (inclusive end — most common for "1..=5")
//   alternation:    1 | 2 | 3
//   wildcard:       _
//   binding:        n            (matches anything, binds the value)
//   guard:          n if n > 0   (extra boolean condition on the arm)

#[test]
fn match_patterns() {
    fn describe(n: i32) -> &'static str {
        match n {
            0 => "zero",
            1..=9 => "single-digit positive",
            10..=99 => "double-digit",
            n if n < 0 => "negative",
            _ => "big",
        }
    }
    assert_eq!(describe(0), "zero");
    assert_eq!(describe(5), "single-digit positive");
    assert_eq!(describe(42), "double-digit");
    assert_eq!(describe(-3), "negative");
    assert_eq!(describe(1000), "big");

    // Alternation and binding:
    fn weekday(s: &str) -> bool {
        matches!(s, "Mon" | "Tue" | "Wed" | "Thu" | "Fri")
    }
    assert!(weekday("Wed"));
    assert!(!weekday("Sun"));
}

// =============================================================================
// 5. `loop`, `while`, `for` — AND `loop` AS AN EXPRESSION
// =============================================================================
// Plain `while` and `for` evaluate to (). But `loop` can produce a value
// via `break value;` — useful for "retry until success."

#[test]
fn loops() {
    // Classic while
    let mut n = 0;
    while n < 5 {
        n += 1;
    }
    assert_eq!(n, 5);

    // `for x in iter` — the standard form. Iterates by value or reference
    // depending on the iterator.
    let v = vec![10, 20, 30];
    let mut sum = 0;
    for &x in v.iter() {
        sum += x;
    }
    assert_eq!(sum, 60);

    // `loop` with break value — a value-returning loop.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 7 {
            break counter * 10; // value flows out of the loop
        }
    };
    assert_eq!(result, 70);
}

// =============================================================================
// 6. `if let` AND `while let`
// =============================================================================
// Shorthand for "match a single pattern, otherwise do nothing." Useful when
// you only care about one variant.

#[test]
fn if_let_while_let() {
    let some_value: Option<i32> = Some(42);

    // if let — like `match { Some(n) => ..., None => () }` but tighter
    let mut found = 0;
    if let Some(n) = some_value {
        found = n;
    }
    assert_eq!(found, 42);

    // while let — keep popping until the iterator returns None
    let mut stack = vec![1, 2, 3];
    let mut total = 0;
    while let Some(top) = stack.pop() {
        total += top;
    }
    assert_eq!(total, 6);
    assert!(stack.is_empty());
}

// =============================================================================
// 7. `let else` — EARLY RETURN ON THE NEGATIVE PATTERN
// =============================================================================
// Cleanest way to say "bind this Some value, otherwise diverge." Covered in
// challenge 03 as well.

fn parse_positive(s: &str) -> Option<i32> {
    let Ok(n) = s.parse::<i32>() else {
        return None;
    };
    if n > 0 { Some(n) } else { None }
}

#[test]
fn let_else_pattern() {
    assert_eq!(parse_positive("42"), Some(42));
    assert_eq!(parse_positive("-1"), None);
    assert_eq!(parse_positive("abc"), None);
}

// =============================================================================
// 8. BLOCKS AS EXPRESSIONS
// =============================================================================
// A bare braced block `{ ... }` is itself an expression — useful for
// scoping and for splitting an inline computation across multiple lines.

#[test]
fn block_as_expression() {
    let total = {
        let a = 5;
        let b = 10;
        a + b // no semicolon — block evaluates to 15
    };
    assert_eq!(total, 15);

    // Common idiom: scope a temporary so it drops at the end of the block.
    let result = {
        let mut buffer = String::new();
        buffer.push_str("hello ");
        buffer.push_str("world");
        buffer
    };
    assert_eq!(result, "hello world");
}

// =============================================================================
// 9. EVERY ARM MUST AGREE — TYPE-INFERENCE TRAPS
// =============================================================================
// The compiler unifies the types of all arms. If they don't agree, you get
// a "match arms have incompatible types" or "if and else have incompatible
// types" error.

#[test]
fn type_unification() {
    let n = 5;

    // ALL arms produce String here
    let label = if n > 0 {
        format!("pos {n}")
    } else {
        format!("non-pos {n}")
    };
    assert_eq!(label, "pos 5");

    // This would NOT compile (left for reference):
    //   let bad = if n > 0 { 1 } else { "not a number" };
    //                                ^^^^^^^^^^^^^^^^^ expected i32, found &str
}
