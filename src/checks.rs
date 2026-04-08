use std::{num::NonZero, ptr::NonNull};

#[track_caller]
#[inline(always)]
pub const fn const_assert(assert_condition: bool) {
    if !assert_condition {
        panic!("Assertion failed.");
    }
}

/// Assert that the size of `T` matches `SIZE`
#[track_caller]
#[inline(always)]
pub const fn assert_size<T, const SIZE: usize>() {
    if const { size_of::<T>() != SIZE } {
        panic!("Size mismatch");
    }
}
const _: () = assert_size::<u8, 1>();

/// Assert that `L` has the same size as `R`.
#[track_caller]
#[inline(always)]
pub const fn assert_same_size<L, R>() {
    if const { size_of::<L>() != size_of::<R>() } {
        panic!("Size mismatch");
    }
}
const _: () = assert_same_size::<u8, i8>();

/// Assert that `L` has a different size from `R`.
#[track_caller]
#[inline(always)]
pub const fn assert_different_size<L, R>() {
    if const { size_of::<L>() == size_of::<R>() } {
        panic!("Size match");
    }
}
const _: () = assert_different_size::<u8, u16>();

/// Assert that the size of `Min` is less than or equal to the size of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size<Min, Max>() {
    if const { size_of::<Min>() > size_of::<Max>() } {
        panic!("Incompatible size");
    }
}
const _: () = assert_compatible_size::<u8, u16>();

/// Assert that the alignment of `T` matches `ALIGN`.
#[track_caller]
#[inline(always)]
pub const fn assert_align<T, const ALIGN: usize>() {
    if const { align_of::<T>() != ALIGN } {
        panic!("Alignment mismatch");
    }
}
const _: () = assert_align::<u64, 8>();

/// Assert that `L` and `R` have the same alignment.
#[track_caller]
#[inline(always)]
pub const fn assert_same_align<L, R>() {
    if const { align_of::<L>() != align_of::<R>() } {
        panic!("Alignment mismatch");
    }
}
const _: () = assert_same_align::<u8, i8>();

/// Assert that `L` and `R` have different alignments.
#[track_caller]
#[inline(always)]
pub const fn assert_different_align<L, R>() {
    if const { align_of::<L>() == align_of::<R>() } {
        panic!("Same alignment");
    }
}
const _: () = assert_different_align::<u8, u16>();

/// Assert that the alignment of `Min` is less than or equal to the alignment of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_align<Min, Max>() {
    if const { align_of::<Min>() > align_of::<Max>() } {
        panic!("Incompatible align");
    }
}
const _: () = assert_compatible_align::<u8, u16>();

/// Assert that `T` has the size of `SIZE` and the alignment of `ALIGN`.
#[track_caller]
#[inline(always)]
pub const fn assert_size_align<T, const SIZE: usize, const ALIGN: usize>() {
    let size_check = const { size_of::<T>() == SIZE };
    let align_check = const { align_of::<T>() == ALIGN };
    match (size_check, align_check) {
        (false, false) => panic!("Size and alignment mismatch"),
        (true, false) => panic!("Alignment mismatch"),
        (false, true) => panic!("Size mismatch"),
        (true, true) => (),
    }
}

/// Assert that `L` and `R` have both the same size and the same alignment.
#[track_caller]
#[inline(always)]
pub const fn assert_same_size_align<L, R>() {
    let size_check = const { size_of::<L>() == size_of::<R>() };
    let align_check = const { align_of::<L>() == align_of::<R>() };
    match (size_check, align_check) {
        (false, false) => panic!("Size and alignment mismatch"),
        (true, false) => panic!("Alignment mismatch"),
        (false, true) => panic!("Size mismatch"),
        (true, true) => (),
    }
}
const _: () = assert_same_size_align::<i8, u8>();

/// Assert that the size and alignment of `Min` are less than or equal to the size and alignment of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size_align<Min, Max>() {
    let size_check = const { size_of::<Min>() <= size_of::<Max>() };
    let align_check = const { align_of::<Min>() <= align_of::<Max>() };
    match (size_check, align_check) {
        (false, false) => panic!("Incompatible size and alignment"),
        (true, false) => panic!("Incompatible alignment"),
        (false, true) => panic!("Incompatible size"),
        (true, true) => (),
    }
}
const _: () = assert_compatible_size_align::<u8, u32>();

/// Assert that `T` is pointer sized.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_size_align<T>() {
    assert_same_size_align::<T, usize>()
}
const _: () = assert_pointer_size_align::<*const ()>();

/// Assert that the size and alignment of `T` is less than the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_compatible_size_align<T>() {
    assert_compatible_size_align::<T, usize>();
}
const _: () = assert_pointer_compatible_size_align::<u8>();

/// Assert that the size and alignment of `T` is equivalent to the size and alignment of `Option<Niched>`.
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche<T, Niched>() {
    assert_same_size_align::<T, Option<Niched>>();
    // Redundant checks for explicit completeness.
    // This ensures that the invariants never change.
    assert_same_size_align::<T, Result<Niched, ()>>();
    assert_same_size_align::<T, Result<(), Niched>>();
}
const _: () = assert_t_niche::<u8, NonZero<u8>>();

/// Assert that the size and alignment of `T` is less than or equal to the size and alignment of `Option<Niched>`.
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche_compatible<T, Niched>() {
    assert_compatible_size_align::<T, Option<Niched>>();
}
const _: () = assert_t_niche_compatible::<u8, NonZero<u16>>();

/// Assert that the size and alignment of `T` is equivalent to the size and alignment of `Option<T>`.
#[track_caller]
#[inline(always)]
pub const fn assert_niche<T>() {
    assert_t_niche::<T, T>();
}
const _: () = assert_niche::<NonZero<u32>>();

/// Assert that the size and alignment of `Option<T>` is equivalent to the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche<T>() {
    assert_t_niche::<usize, T>();
}
const _: () = assert_pointer_niche::<NonNull<()>>();

/// Assert that the size and alignment of `Option<T>` is less than or equal to the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche_compatible<T>() {
    assert_t_niche_compatible::<usize, T>();
}
const _: () = assert_pointer_niche_compatible::<NonZero<u128>>();

/// Assert that the size and alignment of `Option<T>` is 1.
#[track_caller]
#[inline(always)]
pub const fn assert_byte_niche<T>() {
    assert_t_niche::<u8, T>();
}
const _: () = assert_byte_niche::<NonZero<u8>>();