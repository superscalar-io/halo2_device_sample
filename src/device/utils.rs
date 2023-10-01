use crate::poly::Basis;
use crate::poly::Polynomial;

use super::*;
///
pub fn transmute_values<'a, U: std::fmt::Debug>(values: &'a [U]) -> &'a [u8] {
    let ptr = values.as_ptr();
    let len = values.len();

    assert!(
        (ptr as usize) % std::mem::align_of::<u8>() == 0,
        "trying to cast with mismatched layout"
    );

    let size = std::mem::size_of::<U>() * len;
    let out: &'a [u8] = unsafe { std::slice::from_raw_parts(ptr as *const u8, size) };

    out
}

///
pub fn transmute_values_mut<'a, U>(values: &'a [U]) -> &'a mut [u8] {
    let ptr = values.as_ptr();
    let len = values.len();

    assert!(
        (ptr as usize) % std::mem::align_of::<u8>() == 0,
        "trying to cast with mismatched layout"
    );

    let size = std::mem::size_of::<U>() * len;
    let out: &'a mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, size) };

    out
}