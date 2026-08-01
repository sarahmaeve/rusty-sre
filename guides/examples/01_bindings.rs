//! Bindings name values; expressions produce values; patterns take values apart.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html>
//! - <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions>
//! - <https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html>
//! - <https://doc.rust-lang.org/reference/expressions/match-expr.html>
//! - Source study: <https://github.com/rust-lang/rust-analyzer/tree/master/crates/hir-def/src>

#[derive(Debug, PartialEq)]
enum Reading {
    Value { host: String, percent: u8 },
    Missing(String),
}

fn classify(percent: u8) -> &'static str {
    // A block returns its final expression. A trailing semicolon would change
    // this expression's value to `()`.
    if percent >= 90 {
        "critical"
    } else if percent >= 75 {
        "warning"
    } else {
        "healthy"
    }
}

fn summarize(reading: &Reading) -> String {
    match reading {
        Reading::Value {
            host,
            percent: p @ 90..=100,
        } => {
            format!("{host}: critical at {p}%")
        }
        Reading::Value { host, percent } if *percent >= 75 => {
            format!("{host}: warning at {percent}%")
        }
        Reading::Value { host, percent } => format!("{host}: healthy at {percent}%"),
        Reading::Missing(host) => format!("{host}: no sample"),
    }
}

fn first_host(pair: Option<(String, u8)>) -> String {
    // `let ... else` keeps the successful path unindented. The `else` branch
    // must diverge by returning, breaking, continuing, or panicking.
    let Some((host, _)) = pair else {
        return "unknown".to_owned();
    };
    host
}

fn main() {
    let threshold = 75;
    let mut samples = 0;
    samples += 1;

    // Shadowing creates a new binding and may change its type.
    let threshold = threshold.to_string();
    assert_eq!(threshold, "75");
    assert_eq!(samples, 1);

    let point = (3, 8);
    let (x, y) = point;
    let distance_squared = {
        let x2 = x * x;
        let y2 = y * y;
        x2 + y2
    };
    assert_eq!(distance_squared, 73);

    let readings = [
        Reading::Value {
            host: "api-1".to_owned(),
            percent: 93,
        },
        Reading::Value {
            host: "api-2".to_owned(),
            percent: 78,
        },
        Reading::Missing("api-3".to_owned()),
    ];

    assert_eq!(classify(93), "critical");
    assert_eq!(summarize(&readings[0]), "api-1: critical at 93%");
    assert_eq!(summarize(&readings[1]), "api-2: warning at 78%");
    assert_eq!(summarize(&readings[2]), "api-3: no sample");
    assert_eq!(first_host(Some(("worker-1".to_owned(), 42))), "worker-1");
    assert_eq!(first_host(None), "unknown");
}
