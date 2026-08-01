//! Patterns destructure values and can bind, copy, borrow, or move their parts.
//! `let`, parameters, and `for` require irrefutable patterns; `match`, `if let`,
//! `while let`, and `let ... else` can test refutable patterns.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/reference/patterns.html>
//! - <https://doc.rust-lang.org/book/ch19-00-patterns.html>
//! - <https://doc.rust-lang.org/nightly/edition-guide/rust-2024/match-ergonomics.html>
//! - <https://doc.rust-lang.org/std/macro.matches.html>
//! - Source study: <https://github.com/rust-lang/rust/tree/master/compiler/rustc_pattern_analysis>
//! - Source study: <https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src>

#[derive(Debug, PartialEq, Eq)]
struct Sample {
    service: String,
    value: i64,
    labels: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    Sample(Sample),
    Heartbeat { node: String, sequence: u64 },
    Stopped,
}

fn coordinates((x, y): (i32, i32)) -> [i32; 2] {
    [x, y]
}

fn severity(event: &Event) -> &'static str {
    match event {
        Event::Sample(Sample { value, .. }) if *value < 0 => "invalid",
        Event::Sample(Sample { value: 0..=79, .. }) => "normal",
        Event::Sample(Sample {
            value: 80..=100, ..
        }) => "high",
        Event::Sample(_) => "out-of-range",
        Event::Heartbeat { sequence: 0, .. } => "first-heartbeat",
        Event::Heartbeat { .. } => "heartbeat",
        Event::Stopped => "stopped",
    }
}

fn main() {
    // Tuple, struct, and array patterns are irrefutable when every value matches.
    let (service, port) = ("collector", 9090);
    let [primary, secondary] = ["edge-a", "edge-b"];
    assert_eq!(
        (service, port, primary, secondary),
        ("collector", 9090, "edge-a", "edge-b")
    );
    assert_eq!(coordinates((3, 5)), [3, 5]);

    // A refutable pattern needs an alternate path.
    let input = Some("42");
    let Some(text) = input else {
        panic!("missing sample");
    };
    let Ok(value) = text.parse::<i64>() else {
        panic!("invalid sample");
    };
    assert_eq!(value, 42);

    let event = Event::Sample(Sample {
        service: "api".to_owned(),
        value: 91,
        labels: vec!["region=west".to_owned()],
    });
    assert_eq!(severity(&event), "high");

    // Match ergonomics bind through a shared reference by reference.
    if let Event::Sample(Sample {
        service, labels, ..
    }) = &event
    {
        let _: &String = service;
        let [first, rest @ ..] = labels.as_slice() else {
            panic!("missing labels");
        };
        let _: &String = first;
        let _: &[String] = rest;
        assert_eq!((service.as_str(), first.as_str()), ("api", "region=west"));
    }

    // Match ergonomics borrow elements and a variable-length middle slice.
    let numbers: &[i32] = &[10, 20, 30];
    let [first, middle @ .., last] = numbers else {
        panic!("need at least two numbers");
    };
    let _: &i32 = first;
    let _: &[i32] = middle;
    assert_eq!((*first, middle, *last), (10, &[20][..], 30));

    // A fully explicit reference pattern copies the bound scalar elements.
    let &[copied_first, .., copied_last] = &[10, 20, 30];
    let _: i32 = copied_first;
    assert_eq!((copied_first, copied_last), (10, 30));

    let mut counters = [1, 2];
    let [first, _] = &mut counters;
    let _: &mut i32 = first;
    *first += 9;
    assert_eq!(counters, [10, 2]);

    // Explicit `ref` borrows a field while another field moves. `ref mut`
    // creates a mutable field borrow when the default binding mode is `move`.
    let owned = Event::Sample(Sample {
        service: "worker".to_owned(),
        value: 4,
        labels: vec!["tier=batch".to_owned()],
    });
    let Event::Sample(Sample {
        ref service,
        labels,
        ..
    }) = owned
    else {
        unreachable!();
    };
    let _: &String = service;
    let _: Vec<String> = labels;
    assert_eq!((service.as_str(), labels.len()), ("worker", 1));

    let mut mutable = Event::Heartbeat {
        node: "edge-b".to_owned(),
        sequence: 7,
    };
    if let Event::Heartbeat {
        ref mut sequence, ..
    } = mutable
    {
        *sequence += 1;
    }
    assert!(matches!(mutable, Event::Heartbeat { sequence: 8, .. }));

    // `@` tests and retains the same value. Or-pattern alternatives must bind
    // the same names with compatible types and binding modes.
    let response = 503;
    let class = match response {
        code @ (500..=599 | 429) => Some(code),
        _ => None,
    };
    assert_eq!(class, Some(503));

    // Guards run after a structural match and do not make an arm exhaustive.
    let threshold = 10;
    let guarded = match ("queue", 12) {
        (name @ ("queue" | "workers"), value) if value > threshold => Some(name),
        _ => None,
    };
    assert_eq!(guarded, Some("queue"));

    let heartbeat = Event::Heartbeat {
        node: "edge-a".to_owned(),
        sequence: 0,
    };
    assert!(matches!(heartbeat, Event::Heartbeat { sequence: 0, .. }));
    assert_eq!(severity(&heartbeat), "first-heartbeat");
    assert_eq!(severity(&Event::Stopped), "stopped");

    let mut pending = vec![1, 2, 3];
    let mut total = 0;
    while let Some(next) = pending.pop() {
        total += next;
    }
    assert_eq!(total, 6);
}
