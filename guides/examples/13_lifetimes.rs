//! Lifetimes describe relationships between borrows. Read them as constraints on
//! returned references, not as instructions for how long a value should live.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html>
//! - <https://doc.rust-lang.org/reference/lifetime-elision.html>
//! - <https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds>
//! - Source study: <https://github.com/serde-rs/serde/blob/master/serde/src/de/mod.rs>

use std::fmt::Display;

fn first_word(input: &str) -> &str {
    // Elision ties the one output borrow to the one input borrow.
    input.split_whitespace().next().unwrap_or("")
}

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    // Both inputs must remain valid for the lifetime chosen for the result.
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Span<'source> {
    source: &'source str,
    start: usize,
    end: usize,
}

impl<'source> Span<'source> {
    fn text(&self) -> &'source str {
        // The explicit output lifetime ties the result to `source`, rather than
        // to the shorter borrow of `self` selected by method elision.
        &self.source[self.start..self.end]
    }

    fn bounds(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

fn shorten<'long: 'short, 'short>(value: &'long str, _scope: &'short ()) -> &'short str {
    // `'long: 'short` means that `long` outlives `short` and may be shortened.
    value
}

fn display_borrow<'a, T>(value: &'a T) -> String
where
    T: Display + 'a,
{
    format!("{value}")
}

fn measure_all<F>(values: &[&str], measure: F) -> Vec<usize>
where
    // The callback must accept a borrow with any lifetime chosen by this call.
    F: for<'text> Fn(&'text str) -> usize,
{
    values.iter().map(|value| measure(value)).collect()
}

fn main() {
    let message = String::from("queue depth high");
    assert_eq!(first_word(&message), "queue");

    let primary = String::from("api");
    let selected;
    {
        let secondary = String::from("worker");
        selected = longer(&primary, &secondary);
        assert_eq!(selected, "worker");
    } // `selected` cannot be used after the shorter possible source is dropped.

    let span = Span {
        source: &message,
        start: 6,
        end: 11,
    };
    assert_eq!(span.text(), "depth");
    assert_eq!(span.bounds(), (6, 11));

    let local_scope = ();
    assert_eq!(shorten(&message, &local_scope), "queue depth high");
    assert_eq!(display_borrow(&503), "503");
    assert_eq!(measure_all(&["api", "worker"], str::len), [3, 6]);

    // String literals have type `&'static str`. A `T: 'static` bound instead
    // means `T` contains no borrowed data with a shorter lifetime.
    let durable: &'static str = "ready";
    assert_eq!(durable, "ready");
}
