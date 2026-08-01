//! Advanced trait syntax resolves ambiguity and expresses relationships among
//! types. Newtypes create distinct types; aliases only create alternate names.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch20-02-advanced-traits.html>
//! - <https://doc.rust-lang.org/book/ch20-03-advanced-types.html>
//! - <https://doc.rust-lang.org/reference/items/associated-items.html>
//! - <https://doc.rust-lang.org/reference/dynamically-sized-types.html>
//! - Source study: <https://github.com/tower-rs/tower/blob/master/tower-service/src/lib.rs>

use std::{error::Error, fmt, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Milliseconds(u64);

impl fmt::Display for Milliseconds {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}ms", self.0)
    }
}

trait Summary: fmt::Display {
    fn summary(&self) -> String {
        format!("duration={self}")
    }
}

impl Summary for Milliseconds {}

impl Add for Milliseconds {
    type Output = Self;

    fn add(self, right: Self) -> Self::Output {
        Self(self.0 + right.0)
    }
}

trait Metric {
    fn kind(&self) -> &'static str;
}

trait Resource {
    fn kind(&self) -> &'static str;
}

impl Metric for Milliseconds {
    fn kind(&self) -> &'static str {
        "latency"
    }
}

impl Resource for Milliseconds {
    fn kind(&self) -> &'static str {
        "time"
    }
}

struct Csv<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for Csv<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, value) in self.0.iter().enumerate() {
            if index > 0 {
                formatter.write_str(",")?;
            }
            write!(formatter, "{value}")?;
        }
        Ok(())
    }
}

trait View {
    // A generic associated type may borrow from each particular `self` borrow.
    type Window<'a>
    where
        Self: 'a;

    fn first(&self, length: usize) -> Self::Window<'_>;
}

struct Buffer(Vec<u8>);

impl View for Buffer {
    type Window<'a> = &'a [u8];

    fn first(&self, length: usize) -> Self::Window<'_> {
        &self.0[..length.min(self.0.len())]
    }
}

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn offline_error() -> BoxError {
    std::io::Error::other("offline").into()
}

fn debug_unsized<T: fmt::Debug + ?Sized>(value: &T) -> String {
    // `?Sized` removes the usual implicit `Sized` bound. Slices, `str`, and
    // trait objects are dynamically sized and are used behind a pointer.
    format!("{value:?}")
}

fn main() {
    let elapsed = Milliseconds(20) + Milliseconds(30);
    assert_eq!(elapsed, Milliseconds(50));
    assert_eq!(elapsed.summary(), "duration=50ms");

    // Fully qualified syntax selects between identically named trait methods.
    assert_eq!(<Milliseconds as Metric>::kind(&elapsed), "latency");
    assert_eq!(<Milliseconds as Resource>::kind(&elapsed), "time");

    // The newtype permits a local `Display` implementation around `Vec<T>`.
    // A type alias would remain exactly the underlying type.
    assert_eq!(Csv(vec![1, 2, 3]).to_string(), "1,2,3");

    let buffer = Buffer(b"ready".to_vec());
    assert_eq!(buffer.first(3), b"rea");
    assert_eq!(offline_error().to_string(), "offline");
    assert_eq!(debug_unsized("queue"), "\"queue\"");
    assert_eq!(debug_unsized(&[2, 3, 5][..]), "[2, 3, 5]");
}
