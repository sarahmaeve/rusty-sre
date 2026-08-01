//! FFI crosses a boundary where Rust cannot verify ABI, pointer, lifetime,
//! ownership, thread, or unwind contracts. Keep the raw boundary small and put
//! every assumption in a `# Safety` contract before exposing a safe wrapper.
//!
//! `repr(C)` stabilizes field order and C-compatible layout rules; `extern "C"`
//! selects a calling convention. Neither validates pointers or makes types safe
//! to share across the boundary.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/nomicon/ffi.html>
//! - <https://doc.rust-lang.org/reference/type-layout.html#the-c-representation>
//! - <https://doc.rust-lang.org/reference/items/external-blocks.html>
//! - <https://doc.rust-lang.org/std/ffi/struct.CStr.html>
//! - <https://doc.rust-lang.org/std/primitive.pointer.html>
//! - Source study: <https://github.com/bytecodealliance/wasmtime/tree/main/crates/c-api>
//! - Source study: <https://github.com/rustls/rustls-ffi/tree/main/src>

use std::{
    ffi::{CStr, CString, c_char},
    mem::{align_of, offset_of, size_of},
    slice,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MetricBatch {
    pub values: *const i64,
    pub len: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FfiStatus {
    Ok = 0,
    NullPointer = 1,
    Overflow = 2,
}

// External blocks are `unsafe` in edition 2024 because their declarations are
// promises about symbols supplied elsewhere. A build script often supplies the
// corresponding native link directives.
unsafe extern "C" {
    pub fn sre_remote_sum(batch: *const MetricBatch, output: *mut i64) -> FfiStatus;
}

/// Sums a borrowed array into caller-owned output storage.
///
/// # Safety
///
/// If `len` is nonzero, `values` must be aligned and valid to read `len`
/// initialized `i64` values for this call. `output` must be aligned, non-null,
/// and valid to write one `i64`. The two regions must not overlap incompatibly.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sre_sum(values: *const i64, len: usize, output: *mut i64) -> FfiStatus {
    if output.is_null() || (values.is_null() && len != 0) {
        return FfiStatus::NullPointer;
    }

    let values = if len == 0 {
        &[]
    } else {
        // SAFETY: the caller's contract guarantees a readable region, and the
        // null case was rejected above.
        unsafe { slice::from_raw_parts(values, len) }
    };
    let Some(total) = values
        .iter()
        .try_fold(0_i64, |sum, value| sum.checked_add(*value))
    else {
        return FfiStatus::Overflow;
    };

    // SAFETY: the caller's contract guarantees writable output, and null was
    // rejected above.
    unsafe { output.write(total) };
    FfiStatus::Ok
}

/// Returns the byte length of a borrowed, NUL-terminated string.
///
/// # Safety
///
/// `name` must point to a NUL-terminated byte sequence readable for this call.
/// `output` must be aligned, non-null, and valid to write one `usize`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sre_name_len(name: *const c_char, output: *mut usize) -> FfiStatus {
    if name.is_null() || output.is_null() {
        return FfiStatus::NullPointer;
    }
    // SAFETY: the caller guarantees a readable NUL-terminated sequence.
    let name = unsafe { CStr::from_ptr(name) };
    // SAFETY: the caller guarantees writable output, and null was rejected.
    unsafe { output.write(name.to_bytes().len()) };
    FfiStatus::Ok
}

fn checked_sum(values: &[i64]) -> Result<i64, FfiStatus> {
    let mut output = 0;
    // SAFETY: the slice supplies a valid pointer and length; `output` is a live,
    // aligned `i64`, and the immutable input cannot overlap its mutable borrow.
    let status = unsafe { sre_sum(values.as_ptr(), values.len(), &mut output) };
    match status {
        FfiStatus::Ok => Ok(output),
        error => Err(error),
    }
}

fn main() {
    assert_eq!(offset_of!(MetricBatch, values), 0);
    assert!(offset_of!(MetricBatch, len) >= size_of::<*const i64>());
    assert_eq!(align_of::<MetricBatch>(), align_of::<usize>());

    assert_eq!(checked_sum(&[13, 21, 34]), Ok(68));
    assert_eq!(checked_sum(&[i64::MAX, 1]), Err(FfiStatus::Overflow));

    let name = CString::new("queue.depth").unwrap();
    let mut length = 0;
    // SAFETY: `CString` supplies the terminator and keeps the bytes alive;
    // `length` is valid output storage for this call.
    let status = unsafe { sre_name_len(name.as_ptr(), &mut length) };
    assert_eq!(status, FfiStatus::Ok);
    assert_eq!(length, 11);

    // Empty inputs do not require a readable values pointer. The output pointer
    // remains mandatory, and null is rejected before any dereference.
    let mut empty_total = -1;
    // SAFETY: a zero length permits a null input; `empty_total` is valid output.
    let status = unsafe { sre_sum(std::ptr::null(), 0, &mut empty_total) };
    assert_eq!((status, empty_total), (FfiStatus::Ok, 0));
}
