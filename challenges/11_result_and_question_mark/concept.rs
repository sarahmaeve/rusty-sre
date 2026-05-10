// =============================================================================
// Challenge 04: Result and the `?` Operator — Concept Explainer
// =============================================================================
//
// Result<T, E> is Rust's primary mechanism for fallible operations. It is an
// enum with two variants:
//
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
//
// In Python, you would raise an exception. In Rust, you return a Result and
// let the caller decide what to do. The `?` operator makes this ergonomic by
// automatically propagating errors up the call stack.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   raise ValueError("bad")               →  return Err(MyErr::Bad)
//   try: ... except X as e: raise Y(e)    →  impl From<X> for Y, then `?`
//   except X: pass                        →  .ok()  (drops the error)
//   except X: return default              →  .unwrap_or(default)
//   for x in xs: validate(x)              →  xs.iter().map(validate)
//                                              .collect::<Result<Vec<_>, _>>()
//   sys.exit(1) from main                 →  return Err(...) from main

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. CREATING AND PATTERN-MATCHING ON RESULT
// =============================================================================
// Result has two variants: Ok(value) for success, Err(error) for failure.
// You consume a Result by pattern matching, by combinator, or with `?`.

#[test]
fn result_basics() {
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    let good = divide(10, 2);
    assert_eq!(good, Ok(5));

    let bad = divide(10, 0);
    assert_eq!(bad, Err("division by zero".to_string()));

    // Pattern matching is the explicit way to consume a Result:
    match divide(8, 4) {
        Ok(n) => assert_eq!(n, 2),
        Err(_) => panic!("should have succeeded"),
    }

    // .is_ok() / .is_err() are the cheap predicates.
    assert!(good.is_ok());
    assert!(bad.is_err());
}

// =============================================================================
// 2. THE `?` OPERATOR — EARLY RETURN ON ERROR
// =============================================================================
// `expr?` is shorthand for:
//
//     match expr {
//         Ok(v)  => v,
//         Err(e) => return Err(From::from(e)),
//     }
//
// It only works in functions whose return type carries an "error" branch
// (Result or Option). The `From::from` part lets you mix error types as
// long as From conversions are implemented (see Section 6).
//
// Python equivalent: `try/except` with a re-raise — but at the type level.

#[test]
fn question_mark_operator() {
    fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
        let n: u16 = s.parse()?; // returns Err early if parse fails
        Ok(n)
    }

    assert_eq!(parse_port("8080"), Ok(8080));
    assert!(parse_port("abc").is_err());

    // Without `?`, the same function is verbose but identical in behavior:
    fn parse_port_long(s: &str) -> Result<u16, std::num::ParseIntError> {
        let n: u16 = match s.parse() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(n)
    }
    assert_eq!(parse_port_long("8080"), Ok(8080));
}

// =============================================================================
// 3. COMBINATORS: map, map_err, and_then, or_else, ok, err
// =============================================================================
// Result has a rich set of combinators that let you transform values without
// pattern matching. Use them when the chain stays mostly happy-path.

#[test]
fn result_combinators() {
    let parsed: Result<i32, _> = "42".parse();

    // map: transform the Ok value, leave Err alone
    let doubled = parsed.clone().map(|n| n * 2);
    assert_eq!(doubled, Ok(84));

    // map_err: transform the Err value, leave Ok alone
    let renamed = "abc".parse::<i32>().map_err(|_| "bad number");
    assert_eq!(renamed, Err("bad number"));

    // and_then: chain another fallible op (a.k.a. flatMap; Python: nested try)
    let result: Result<i32, String> = "10"
        .parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| if n > 0 { Ok(n) } else { Err("non-positive".into()) });
    assert_eq!(result, Ok(10));

    // or_else: provide a fallback Result if Err
    let recovered: Result<i32, String> =
        "bad".parse::<i32>().or_else(|_| Ok::<i32, String>(0));
    assert_eq!(recovered, Ok(0));

    // unwrap_or / unwrap_or_else / unwrap_or_default: collapse to a value
    assert_eq!("100".parse::<i32>().unwrap_or(0), 100);
    assert_eq!("nope".parse::<i32>().unwrap_or(0), 0);
    assert_eq!("nope".parse::<i32>().unwrap_or_default(), 0); // i32::default() == 0

    // .ok() / .err() turn Result into Option
    assert_eq!(Ok::<i32, &str>(5).ok(), Some(5));
    assert_eq!(Err::<i32, &str>("x").ok(), None);
    assert_eq!(Err::<i32, &str>("x").err(), Some("x"));
}

// =============================================================================
// 4. unwrap / expect — WHEN TO USE THEM (HINT: SPARINGLY)
// =============================================================================
// .unwrap() returns the Ok value or PANICS on Err. .expect("msg") is the same
// but with a custom panic message. Rules of thumb:
//
//   - In tests:                           fine.
//   - In examples and one-off scripts:    fine.
//   - In library code:                    almost never. Propagate with `?`.
//   - In binary code:                     only when the variant is provably Ok.
//
// `.expect("...")` is strictly better than `.unwrap()` when you do choose to
// panic — the message becomes an "I believed this was safe because…" note.

#[test]
fn unwrap_versus_propagation() {
    let n: i32 = "42".parse().unwrap(); // OK in tests
    assert_eq!(n, 42);

    let m: i32 = "100".parse().expect("hard-coded literal must parse");
    assert_eq!(m, 100);
}

// =============================================================================
// 5. CUSTOM ERROR ENUMS — `impl Error for MyError`
// =============================================================================
// Real-world Rust errors are usually enums where each variant represents one
// failure mode. Implementing Display + std::error::Error is the standard
// shape; the `thiserror` crate just generates this for you.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum AppError {
    BadInput(String),
    NotFound(String),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadInput(msg) => write!(f, "bad input: {msg}"),
            AppError::NotFound(name) => write!(f, "not found: {name}"),
            AppError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            _ => None,
        }
    }
}

// From<X> for AppError — this is what makes `?` work across error types.
// When `?` sees an Err(io_error) but the function returns Result<_, AppError>,
// it asks for `<AppError as From<io::Error>>::from(io_error)` and uses that.
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

#[test]
fn custom_error_enum() {
    let err = AppError::BadInput("port out of range".into());
    assert_eq!(err.to_string(), "bad input: port out of range");

    let nf = AppError::NotFound("config.toml".into());
    assert_eq!(nf.to_string(), "not found: config.toml");

    // The Error trait gives us source() for chaining root causes.
    let io = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no perms");
    let app: AppError = io.into(); // From<io::Error> kicks in
    assert!(app.source().is_some());
}

// =============================================================================
// 6. PROPAGATING DIFFERENT ERROR TYPES WITH `?`
// =============================================================================
// `?` will perform a `From::from` conversion automatically. So a function that
// returns `Result<T, AppError>` can use `?` on any operation whose error type
// has `impl From<E> for AppError`.

fn parse_then_check(s: &str) -> Result<u32, AppError> {
    // ParseIntError → BadInput via map_err (no From impl needed)
    let n: u32 = s
        .parse()
        .map_err(|e: std::num::ParseIntError| AppError::BadInput(e.to_string()))?;

    // Synthesize a NotFound for paths we know don't exist:
    if !std::path::Path::new("/probably/does/not/exist/abcdef").exists() {
        return Err(AppError::NotFound("/probably/does/not/exist/abcdef".into()));
    }

    Ok(n)
}

#[test]
fn cross_error_propagation() {
    // Bad parse: BadInput
    let r = parse_then_check("abc");
    assert!(matches!(r, Err(AppError::BadInput(_))));

    // Good parse, but the synthetic path is missing: NotFound
    let r = parse_then_check("42");
    assert!(matches!(r, Err(AppError::NotFound(_))));
}

// =============================================================================
// 7. RETURNING Result FROM main()
// =============================================================================
// `fn main() -> Result<(), Box<dyn Error>>` is the standard "I'm allowed to
// fail and want a proper exit code" shape. Box<dyn Error> means "any error
// type" — useful in main where you don't want to lock down to one error enum.
//
// We can't rewrite main() inside this test file, but here's the shape:
//
//     fn main() -> Result<(), Box<dyn std::error::Error>> {
//         let port: u16 = std::env::var("PORT")?.parse()?;
//         println!("listening on port {port}");
//         Ok(())
//     }
//
// Returning Err prints the error (Debug formatting) and exits with code 1.

// =============================================================================
// 8. Result vs Option — INTERCONVERSION
// =============================================================================
// Sometimes you have an Option but want a Result (e.g., to use `?` in a
// Result-returning function). And vice versa.
//
//   Option::ok_or(e)        — None becomes Err(e), Some(v) becomes Ok(v)
//   Option::ok_or_else(f)   — like ok_or but lazy
//   Result::ok()            — Ok(v) becomes Some(v), Err(_) becomes None
//   Result::err()           — Err(e) becomes Some(e), Ok(_) becomes None

#[test]
fn option_result_interop() {
    let some: Option<i32> = Some(5);
    let none: Option<i32> = None;

    assert_eq!(some.ok_or("missing"), Ok(5));
    assert_eq!(none.ok_or("missing"), Err("missing"));

    let ok: Result<i32, &str> = Ok(7);
    let err: Result<i32, &str> = Err("nope");
    assert_eq!(ok.ok(), Some(7));
    assert_eq!(err.ok(), None);
}

// =============================================================================
// 9. COLLECT-INTO-RESULT — Result<Vec<T>, E>
// =============================================================================
// A neat property of Result: an Iterator of Result<T, E> can collect into
// Result<Vec<T>, E>. The first Err short-circuits the collect.

#[test]
fn collecting_results() {
    let inputs = ["10", "20", "30"];
    let parsed: Result<Vec<i32>, _> = inputs.iter().map(|s| s.parse()).collect();
    assert_eq!(parsed.unwrap(), vec![10, 20, 30]);

    let bad_inputs = ["10", "twenty", "30"];
    let parsed: Result<Vec<i32>, _> = bad_inputs.iter().map(|s| s.parse()).collect();
    assert!(parsed.is_err());

    // If you instead want "skip the bad ones," filter by .ok():
    let skipped: Vec<i32> = bad_inputs.iter().filter_map(|s| s.parse().ok()).collect();
    assert_eq!(skipped, vec![10, 30]);
}

// =============================================================================
// 10. SRE PATTERN: RETRY-WITH-MAX-ATTEMPTS
// =============================================================================
// A common SRE shape: try something up to N times; succeed early on the first
// Ok; return the last Err if all attempts fail.

fn retry<F, T, E>(mut attempts: u32, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    loop {
        match op() {
            Ok(v) => return Ok(v),
            Err(e) if attempts <= 1 => return Err(e),
            Err(_) => attempts -= 1,
        }
    }
}

#[test]
fn retry_pattern() {
    let mut calls = 0;

    // Succeed on the third attempt
    let result = retry(5, || {
        calls += 1;
        if calls < 3 { Err("transient") } else { Ok("ok") }
    });
    assert_eq!(result, Ok("ok"));
    assert_eq!(calls, 3);

    // Always fail — exhaust attempts
    let mut tries = 0;
    let result: Result<&str, &str> = retry(3, || {
        tries += 1;
        Err("permanent")
    });
    assert_eq!(result, Err("permanent"));
    assert_eq!(tries, 3);
}
