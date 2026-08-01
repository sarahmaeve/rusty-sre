//! Unsafe Rust permits operations the compiler cannot prove sound. A safe API
//! around unsafe code must establish every precondition and preserve invariants.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html>
//! - <https://doc.rust-lang.org/nomicon/what-unsafe-does.html>
//! - <https://doc.rust-lang.org/reference/unsafe-keyword.html>
//! - <https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html>
//! - Source study: <https://github.com/rust-lang/rust/blob/master/library/alloc/src/vec/mod.rs>

#[derive(Debug, PartialEq, Eq)]
struct Empty;

#[derive(Debug, PartialEq, Eq)]
struct NonEmpty<T> {
    values: Vec<T>,
}

impl<T> NonEmpty<T> {
    fn new(values: Vec<T>) -> Result<Self, Empty> {
        if values.is_empty() {
            Err(Empty)
        } else {
            Ok(Self { values })
        }
    }

    fn first(&self) -> &T {
        // SAFETY: `new` is the only constructor and rejects empty vectors.
        // No method can remove elements, so index 0 remains in bounds.
        unsafe { self.values.get_unchecked(0) }
    }
}

fn split_at_mut<T>(values: &mut [T], middle: usize) -> (&mut [T], &mut [T]) {
    assert!(middle <= values.len());
    let length = values.len();
    let pointer = values.as_mut_ptr();

    // SAFETY: `pointer` came from `values` and is valid for `length` elements.
    // The assertion keeps `middle` in bounds. The two ranges are disjoint, and
    // the input borrow prevents any other access for the returned lifetime.
    unsafe {
        (
            std::slice::from_raw_parts_mut(pointer, middle),
            std::slice::from_raw_parts_mut(pointer.add(middle), length - middle),
        )
    }
}

/// Reads one byte without checking bounds.
///
/// # Safety
///
/// `pointer` must be non-null, aligned for `u8`, and valid to read one byte for
/// the duration of this call.
unsafe fn read_byte(pointer: *const u8) -> u8 {
    // An unsafe function's body is not implicitly unsafe in edition 2024.
    // SAFETY: the caller must uphold this function's documented contract.
    unsafe { pointer.read() }
}

fn checked_read(bytes: &[u8]) -> Option<u8> {
    let pointer = bytes.first()? as *const u8;
    // SAFETY: `first` proved that one initialized byte is readable, and the
    // shared slice remains borrowed for the call.
    Some(unsafe { read_byte(pointer) })
}

fn main() {
    assert_eq!(NonEmpty::<u8>::new(Vec::new()), Err(Empty));
    let values = NonEmpty::new(vec![13, 21]).unwrap();
    assert_eq!(values.first(), &13);

    let mut numbers = [1, 2, 3, 4];
    let (left, right) = split_at_mut(&mut numbers, 2);
    left[0] = 10;
    right[0] = 30;
    assert_eq!(numbers, [10, 2, 30, 4]);

    assert_eq!(checked_read(&[]), None);
    assert_eq!(checked_read(b"ok"), Some(b'o'));
}
