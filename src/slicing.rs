use std::{mem::transmute, ops::Range};

pub const fn subslice<T>(slice: &[T], range: Range<usize>) -> &[T] {
    assert!(range.end >= range.start, "Backwards range");
    assert!(range.end <= slice.len(), "Out of bounds");
    let len = range.end - range.start;
    unsafe { std::slice::from_raw_parts(slice.as_ptr().add(range.start), len) }
}

pub const fn subslice_mut<T>(slice: &mut [T], range: Range<usize>) -> &mut [T] {
    assert!(range.end >= range.start, "Backwards range");
    assert!(range.end <= slice.len(), "Out of bounds");
    let len = range.end - range.start;
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().add(range.start), len) }
}

pub const fn split<'a, T>(slice: &'a [T], split_index: usize) -> (&'a [T], &'a [T]) {
    let end = slice.len();
    assert!(split_index <= end, "Index out of bounds.");
    let right_len = end - split_index;
    unsafe {
        (
            std::slice::from_raw_parts(slice.as_ptr(), split_index),
            std::slice::from_raw_parts(slice.as_ptr().add(split_index), right_len),
        )
    }
}

pub const fn split_mut<'a, T>(
    slice: &'a mut [T],
    split_index: usize,
) -> (&'a mut [T], &'a mut [T]) {
    let end = slice.len();
    assert!(split_index <= end, "Index out of bounds.");
    let right_len = end - split_index;
    unsafe {
        (
            std::slice::from_raw_parts_mut(slice.as_mut_ptr(), split_index),
            std::slice::from_raw_parts_mut(slice.as_mut_ptr().add(split_index), right_len),
        )
    }
}

pub const fn half_split<'a, T>(slice: &'a [T]) -> (&'a [T], &'a [T]) {
    let mid = slice.len() / 2;
    let left_len = mid;
    let right_len = slice.len() - mid;
    unsafe {
        (
            std::slice::from_raw_parts(slice.as_ptr(), left_len),
            std::slice::from_raw_parts(slice.as_ptr().add(mid), right_len),
        )
    }
}

pub const fn half_split_mut<'a, T>(slice: &'a mut [T]) -> (&'a mut [T], &'a mut [T]) {
    let mid = slice.len() / 2;
    let left_len = mid;
    let right_len = slice.len() - mid;
    unsafe {
        (
            std::slice::from_raw_parts_mut(slice.as_mut_ptr(), left_len),
            std::slice::from_raw_parts_mut(slice.as_mut_ptr().add(mid), right_len),
        )
    }
}

pub const fn substr(s: &str, range: Range<usize>) -> &str {
    assert!(
        s.is_char_boundary(range.start) && s.is_char_boundary(range.end),
        "Not utf-8 boundary."
    );
    unsafe { transmute(subslice(s.as_bytes(), range)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        let mut items: [u32; 16] = [
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
        ];
        let (left, right) = half_split_mut(&mut items);
        let mut index = 0;
        while index < left.len() {
            std::mem::swap(&mut left[index], &mut right[right.len() - 1 - index]);
            index += 1;
        }
        for item in items {
            println!("{item}");
        }
    }
}
