use std::{num::NonZero, ptr::NonNull};

use crate::{meta::{ImpossibleZst}, niche::{Niche255}};

/* All Checks Available:
pub const fn const_assert(assert_condition: bool)

pub const fn assert_size<T, const SIZE: usize>()
pub const fn assert_not_size<T, const SIZE: usize>()
pub const fn assert_same_size<L, R>()
pub const fn assert_different_size<L, R>()
pub const fn assert_compatible_size<Min, Max>()
pub const fn assert_incompatible_size<Max, Min>()

pub const fn assert_align<T, const ALIGN: usize>()
pub const fn assert_not_align<T, const ALIGN: usize>()
pub const fn assert_same_align<L, R>()
pub const fn assert_different_align<L, R>()
pub const fn assert_compatible_align<Min, Max>()
pub const fn assert_incompatible_align<Max, Min>()

pub const fn assert_size_align<T, const SIZE: usize, const ALIGN: usize>()
pub const fn assert_not_size_align<T, const SIZE: usize, const ALIGN: usize>()
pub const fn assert_same_size_align<L, R>()
pub const fn assert_different_size_align<L, R>()
pub const fn assert_compatible_size_align<Min, Max>()
pub const fn assert_incompatible_size_align<Max, Min>()

pub const fn assert_pointer_size<T>()
pub const fn assert_not_pointer_size<T>()
pub const fn assert_pointer_align<T>()
pub const fn assert_not_pointer_align<T>()
pub const fn assert_pointer_size_align<T>()
pub const fn assert_not_pointer_size_align<T>()
pub const fn assert_pointer_compatible_size_align<T>()
pub const fn assert_pointer_incompatible_size_align<T>()

pub const fn assert_t_niche<T, Niched>()
pub const fn assert_no_t_niche<T, Niched>()
pub const fn assert_t_niche_compatible<T, Niched>()
pub const fn assert_niche<T>()
pub const fn assert_no_niche<T>()

pub const fn assert_pointer_niche<T>()
pub const fn assert_pointer_niche_compatible<T>()

pub const fn assert_byte_niche<T>()

pub const fn assert_byte_size_align<T>()
pub const fn assert_byte_compatible<T>()
pub const fn assert_byte_incompatible<T>()

pub const fn assert_zst<T>()
*/

#[track_caller]
#[inline(always)]
pub const fn const_assert(assert_condition: bool) {
    _ = assert_condition || panic!("Assertion failed");
}
const _: () = const_assert(true);

#[must_use]
#[inline(always)]
pub const fn all_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] || return false;
        index += 1;
    }
    true
}

#[track_caller]
#[inline(always)]
pub const fn const_assert_all<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = all_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_all([
    true,
    true,
]);

#[must_use]
#[inline(always)]
pub const fn any_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] && return true;
        index += 1;
    }
    false
}

#[track_caller]
#[inline(always)]
pub const fn const_assert_any<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = any_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_any([
    false,
    false,
    true,
]);

#[must_use]
#[inline(always)]
pub const fn none_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] && return false;
        index += 1;
    }
    true
}

#[track_caller]
#[inline(always)]
pub const fn const_assert_none<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = none_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_none([
    false,
    false,
    false,
]);

#[must_use]
#[inline(always)]
pub const fn size_condition<T, const SIZE: usize>() -> bool {
    const { size_of::<T>() == SIZE }
}

/// Assert that the size of `T` matches `SIZE`
#[track_caller]
#[inline(always)]
pub const fn assert_size<T, const SIZE: usize>() {
    _ = size_condition::<T, SIZE>() || panic!("Size mismatch");
}
const _: () = assert_size::<u8, 1>();

#[must_use]
#[inline(always)]
pub const fn not_size_condition<T, const SIZE: usize>() -> bool {
    const { size_of::<T>() != SIZE }
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_size<T, const SIZE: usize>() {
    _=not_size_condition::<T, SIZE>() || panic!("Same size");
}
const _: () = assert_not_size::<u64, 4>();

#[must_use]
#[inline(always)]
pub const fn same_size_condition<L, R>() -> bool {
    const { size_of::<L>() == size_of::<R>() }
}

/// Assert that `L` has the same size as `R`.
#[track_caller]
#[inline(always)]
pub const fn assert_same_size<L, R>() {
    _ = same_size_condition::<L, R>() || panic!("Size mismatch");
}
const _: () = assert_same_size::<u8, i8>();

#[must_use]
#[inline(always)]
pub const fn different_size_condition<L, R>() -> bool {
    const { size_of::<L>() != size_of::<R>() }
}

/// Assert that `L` has a different size from `R`.
#[track_caller]
#[inline(always)]
pub const fn assert_different_size<L, R>() {
    _ = different_size_condition::<L, R>() || panic!("Size match");
}
const _: () = assert_different_size::<u8, u16>();

#[must_use]
#[inline(always)]
pub const fn compatible_size_condition<Min, Max>() -> bool {
    const { size_of::<Min>() <= size_of::<Max>() }
}

/// Assert that the size of `Min` is less than or equal to the size of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size<Min, Max>() {
    _ = compatible_size_condition::<Min, Max>() || panic!("Incompatible size");
}
const _: () = assert_compatible_size::<u8, u16>();

#[must_use]
#[inline(always)]
pub const fn incompatible_size_condition<Max, Min>() -> bool {
    const { size_of::<Max>() > size_of::<Min>() }
}

/// Assert that the size of `Max` is greater than the size of `Min`.
#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_size<Max, Min>() {
    _ = incompatible_size_condition::<Max, Min>() || panic!("Compatible size");
}
const _: () = assert_incompatible_size::<u64, u32>();

#[must_use]
#[inline(always)]
pub const fn align_condition<T, const ALIGN: usize>() -> bool {
    const { align_of::<T>() == ALIGN }
}

/// Assert that the alignment of `T` matches `ALIGN`.
#[track_caller]
#[inline(always)]
pub const fn assert_align<T, const ALIGN: usize>() {
    _=align_condition::<T, ALIGN>() || panic!("Alignment mismatch");
}
const _: () = assert_align::<u64, 8>();

#[must_use]
#[inline(always)]
pub const fn not_align_condition<T, const ALIGN: usize>() -> bool {
    const { align_of::<T>() != ALIGN }
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_align<T, const ALIGN: usize>() {
    _=not_align_condition::<T, ALIGN>() || panic!("Same alignment");
}
const _: () = assert_not_align::<u64, 4>();

#[must_use]
#[inline(always)]
pub const fn same_align_condition<L, R>() -> bool {
    const { align_of::<L>() == align_of::<R>() }
}

/// Assert that `L` and `R` have the same alignment.
#[track_caller]
#[inline(always)]
pub const fn assert_same_align<L, R>() {
    _=same_align_condition::<L, R>() || panic!("Alignment mismatch");
}
const _: () = assert_same_align::<u8, i8>();

#[must_use]
#[inline(always)]
pub const fn different_align_condition<L, R>() -> bool {
    const { align_of::<L>() != align_of::<R>() }
}

/// Assert that `L` and `R` have different alignments.
#[track_caller]
#[inline(always)]
pub const fn assert_different_align<L, R>() {
    _=different_align_condition::<L, R>() || panic!("Same alignment");
}
const _: () = assert_different_align::<u8, u16>();

#[must_use]
#[inline(always)]
pub const fn compatible_align_condition<Min, Max>() -> bool {
    const { align_of::<Min>() <= align_of::<Max>() }
}

/// Assert that the alignment of `Min` is less than or equal to the alignment of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_align<Min, Max>() {
    _=compatible_align_condition::<Min, Max>() || panic!("Incompatible align");
}
const _: () = assert_compatible_align::<u8, u16>();

#[must_use]
#[inline(always)]
pub const fn incompatible_align_condition<Max, Min>() -> bool {
    const { align_of::<Max>() > align_of::<Min>() }
}

/// Assert that the alignment of `Max` is greater than the alignment of `Min`.
#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_align<Max, Min>() {
    _=incompatible_align_condition::<Max, Min>() || panic!("Compatible align");
}
const _: () = assert_incompatible_align::<u32, u16>();

#[track_caller]
#[inline(always)]
pub const fn size_align_condition<T, const SIZE: usize, const ALIGN: usize>() -> bool {
    const {
        size_of::<T>() == SIZE
        && align_of::<T>() == ALIGN
    }
}

/// Assert that `T` has the size of `SIZE` and the alignment of `ALIGN`.
#[track_caller]
#[inline(always)]
pub const fn assert_size_align<T, const SIZE: usize, const ALIGN: usize>() {
    // This function doesn't use the `size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<T>() == SIZE };
    let align_check = const { align_of::<T>() == ALIGN };
    match (size_check, align_check) {
        (false, false) => panic!("Size and alignment mismatch"),
        (true, false) => panic!("Alignment mismatch"),
        (false, true) => panic!("Size mismatch"),
        (true, true) => (),
    }
}
const _: () = assert_size_align::<u32, 4, 4>();

#[must_use]
#[inline(always)]
pub const fn not_size_align_condition<T, const SIZE: usize, const ALIGN: usize>() -> bool {
    const {
        size_of::<T>() != SIZE
        || align_of::<T>() != ALIGN
    }
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_size_align<T, const SIZE: usize, const ALIGN: usize>() {
    const {
        let not_size = size_of::<T>() != SIZE;
        let not_align = align_of::<T>() != ALIGN;
        match (not_size, not_align) {
            (true, true) => (),
            (true, false) => panic!("Same alignment"),
            (false, true) => panic!("Same size"),
            (false, false) => panic!("Same size and alignment"),
        }
    }
}
const _: () = assert_not_size_align::<u8, 2, 2>();

#[must_use]
#[inline(always)]
pub const fn same_size_align_condition<L, R>() -> bool {
    const {
        size_of::<L>() == size_of::<R>()
        && align_of::<L>() == align_of::<R>()
    }
}

/// Assert that `L` and `R` have both the same size and the same alignment.
#[track_caller]
#[inline(always)]
pub const fn assert_same_size_align<L, R>() {
    // This function doesn't use the `same_size_align_condition` function
    // because you can more easily customize the panic message this way.
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

#[must_use]
#[inline(always)]
pub const fn different_size_align_condition<L, R>() -> bool {
    const {
        size_of::<L>() != size_of::<R>()
        && align_of::<L>() != align_of::<R>()
    }
}

/// Assert that `L` and `R` have different sizes and alignments.
#[track_caller]
#[inline(always)]
pub const fn assert_different_size_align<L, R>() {
    // This function doesn't use the `different_size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<L>() != size_of::<R>() };
    let align_check = const { align_of::<L>() != align_of::<R>() };
    match (size_check, align_check) {
        (false, false) => panic!("Same size and alignment"),
        (true, false) => panic!("Same alignment"),
        (false, true) => panic!("Same size"),
        (true, true) => (),
    }
}
const _: () = assert_different_size_align::<u8, u32>();

#[must_use]
#[inline(always)]
pub const fn compatible_size_align_condition<Min, Max>() -> bool {
    const {
        size_of::<Min>() <= size_of::<Max>()
        && align_of::<Min>() <= align_of::<Max>()
    }
}

/// Assert that the size and alignment of `Min` are less than or equal to the size and alignment of `Max`.
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size_align<Min, Max>() {
    // This function doesn't use the `compatible_size_align_condition` function
    // because you can more easily customize the panic message this way.
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

/// Note that this only returns true when both the size and the alignment are incompatible.
/// 
/// If one of them is compatible, it will return false.
#[must_use]
#[inline(always)]
pub const fn incompatible_size_align_condition<Max, Min>() -> bool {
    const {
        size_of::<Max>() > size_of::<Min>()
        || align_of::<Max>() > align_of::<Min>()
    }
}

#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_size_align<Max, Min>() {
    const {
        let size_incompatible = incompatible_size_condition::<Max, Min>();
        let align_incompatible = incompatible_align_condition::<Max, Min>();
        match (size_incompatible, align_incompatible) {
            (true, true) => (),
            (true, false) => panic!("Alignment is compatible"),
            (false, true) => panic!("Size is compatible"),
            (false, false) => panic!("Size and alignment are compatible"),
        }
    }
}
const _: () = assert_incompatible_size_align::<u64, u32>();

#[must_use]
#[inline(always)]
pub const fn pointer_size_condition<T>() -> bool {
    same_size_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_pointer_size<T>() {
    assert_same_size::<T, usize>();
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_size::<[u8; 8]>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_size::<[u8; 4]>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_size::<[u8; 2]>();

#[must_use]
#[inline(always)]
pub const fn not_pointer_size_condition<T>() -> bool {
    different_size_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_size<T>() {
    assert_different_size::<T, usize>();
}
const _: () = assert_not_pointer_size::<u8>();

#[must_use]
#[inline(always)]
pub const fn pointer_align_condition<T>() -> bool {
    same_align_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_pointer_align<T>() {
    assert_same_align::<T, usize>();
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_align::<u64>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_align::<u32>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_align::<u16>();

#[must_use]
#[inline(always)]
pub const fn not_pointer_align_condition<T>() -> bool {
    different_align_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_align<T>() {
    assert_different_align::<T, usize>()
}
const _: () = assert_not_pointer_align::<[u8; 8]>();

#[must_use]
#[inline(always)]
pub const fn pointer_size_align_condition<T>() -> bool {
    same_size_align_condition::<T, usize>()
}

/// Assert that `T` is pointer sized.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_size_align<T>() {
    assert_same_size_align::<T, usize>()
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_size_align::<u64>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_size_align::<u32>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_size_align::<u16>();

#[must_use]
#[inline(always)]
pub const fn not_pointer_size_align_condition<T>() -> bool {
    different_size_align_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_size_align<T>() {
    assert_different_size_align::<T, usize>();
}
const _: () = assert_not_pointer_size_align::<[u8; 7]>();

#[must_use]
#[inline(always)]
pub const fn pointer_compatible_size_align_condition<T>() -> bool {
    compatible_size_align_condition::<T, usize>()
}

/// Assert that the size and alignment of `T` is less than the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_compatible_size_align<T>() {
    assert_compatible_size_align::<T, usize>();
}
const _: () = assert_pointer_compatible_size_align::<u8>();

#[must_use]
#[inline(always)]
pub const fn pointer_incompatible_size_align_condition<T>() -> bool {
    incompatible_size_align_condition::<T, usize>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_pointer_incompatible_size_align<T>() {
    assert_incompatible_size_align::<T, usize>();
}

#[must_use]
#[inline(always)]
pub const fn t_niche_condition<T, Niched>() -> bool {
    same_size_align_condition::<T, Option<Niched>>()
}

/// Assert that the size and alignment of `T` is equivalent to the size and alignment of `Option<Niched>`.
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche<T, Niched>() {
    assert_same_size_align::<T, Option<Niched>>();
}
const _: () = assert_t_niche::<u8, NonZero<u8>>();

#[must_use]
#[inline(always)]
pub const fn no_t_niche_condition<T, Niched>() -> bool {
    incompatible_size_condition::<Option<Niched>, T>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_no_t_niche<T, Niched>() {
    assert_incompatible_size::<Option<Niched>, T>();
}
const _: () = assert_no_t_niche::<u8, u8>();

#[must_use]
#[inline(always)]
pub const fn t_niche_compatible_condition<T, Niched>() -> bool {
    compatible_size_align_condition::<T, Option<Niched>>()
}

/// Assert that the size and alignment of `T` is less than or equal to the size and alignment of `Option<Niched>`.
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche_compatible<T, Niched>() {
    assert_compatible_size_align::<T, Option<Niched>>();
}
const _: () = assert_t_niche_compatible::<u8, NonZero<u16>>();

#[must_use]
#[inline(always)]
pub const fn niche_condition<T>() -> bool {
    t_niche_condition::<T, T>()
}

/// Assert that the size and alignment of `T` is equivalent to the size and alignment of `Option<T>`.
#[track_caller]
#[inline(always)]
pub const fn assert_niche<T>() {
    assert_t_niche::<T, T>();
}
const _: () = assert_niche::<NonZero<u32>>();

#[must_use]
#[inline(always)]
pub const fn no_niche_condition<T>() -> bool {
    different_size_condition::<T, Option<T>>()
}

/// Asserts that the size of `T` is different from the size of `Option<T>`.
#[track_caller]
#[inline(always)]
pub const fn assert_no_niche<T>() {
    assert_different_size::<T, Option<T>>();
}
const _: () = assert_no_niche::<usize>();

#[must_use]
#[inline(always)]
pub const fn pointer_niche_condition<T>() -> bool {
    t_niche_condition::<usize, T>()
}

/// Assert that the size and alignment of `Option<T>` is equivalent to the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche<T>() {
    assert_t_niche::<usize, T>();
}
const _: () = assert_pointer_niche::<NonNull<()>>();

#[must_use]
#[inline(always)]
pub const fn no_pointer_niche_condition<T>() -> bool {
    no_t_niche_condition::<usize, T>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_no_pointer_niche<T>() {
    assert_no_t_niche::<usize, T>();
}
const _: () = assert_no_pointer_niche::<usize>();

#[must_use]
#[inline(always)]
pub const fn pointer_niche_compatible_condition<T>() -> bool {
    t_niche_compatible_condition::<usize, T>()
}

/// Assert that the size and alignment of `Option<T>` is less than or equal to the size and alignment of a pointer.
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche_compatible<T>() {
    assert_t_niche_compatible::<usize, T>();
}
const _: () = assert_pointer_niche_compatible::<NonZero<u128>>();

#[must_use]
#[inline(always)]
pub const fn byte_niche_condition<T>() -> bool {
    t_niche_condition::<u8, T>()
}

/// Assert that the size and alignment of `Option<T>` is 1.
#[track_caller]
#[inline(always)]
pub const fn assert_byte_niche<T>() {
    assert_t_niche::<u8, T>();
}
const _: () = assert_byte_niche::<NonZero<u8>>();

#[must_use]
#[inline(always)]
pub const fn byte_size_align_condition<T>() -> bool {
    same_size_align_condition::<T, u8>()
}

/// Assert that the size and alignment of `T` is 1.
#[track_caller]
#[inline(always)]
pub const fn assert_byte_size_align<T>() {
    assert_same_size_align::<T, u8>();
}
const _: () = assert_byte_size_align::<bool>();

#[must_use]
#[inline(always)]
pub const fn byte_compatible_condition<T>() -> bool {
    compatible_size_align_condition::<T, u8>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_byte_compatible<T>() {
    assert_compatible_size_align::<T, u8>()
}

#[must_use]
#[inline(always)]
pub const fn byte_incompatible_condition<T>() -> bool {
    incompatible_size_align_condition::<T, u8>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_byte_incompatible<T>() {
    assert_incompatible_size_align::<T, u8>();
}
const _: () = assert_byte_incompatible::<u16>();

#[must_use]
#[inline(always)]
pub const fn zst_condition<T>() -> bool {
    size_condition::<T, 0>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_zst<T>() {
    assert_size::<T, 0>()
}
const _: () = assert_zst::<()>();

#[must_use]
#[inline(always)]
pub const fn not_zst_condition<T>() -> bool {
    const { !size_condition::<T, 0>() }
}

#[track_caller]
#[inline(always)]
pub const fn assert_not_zst<T>() {
    assert_not_size::<T, 0>();
}

#[must_use]
#[inline(always)]
pub const fn impossible_zst_condition<T>() -> bool {
    zst_condition::<T>() && byte_niche_condition::<Niche255<T>>()
}

#[track_caller]
#[inline(always)]
pub const fn assert_impossible_zst<T>() {
    _=zst_condition::<T>() || panic!("Not a zst.");
    _=byte_niche_condition::<Niche255<T>>() || panic!("Not impossible to create");
}
const _: () = assert_impossible_zst::<ImpossibleZst>();

// const _: () = {
//     const_assert_all([
//         size_of::<PackedTuple<ImpossibleZst, u8>>() == 1,
//         size_of::<Option<Niche255<PackedTuple<ImpossibleZst, u64>>>>() == 9,
//         size_of::<Option<Niche255<PackedTuple<OneVariant, u64>>>>() == 10,
//         size_of::<Option<Niche255<PackedTuple<(), u64>>>>() == 10,
//     ]);
// };

// const _: () = {
//     assert_niche::<Niche255<Tuple<ImpossibleZst, ImpossibleZst>>>();
//     assert_niche::<Niche255<Tuple<ImpossibleZstStruct, ImpossibleZstStruct>>>();
//     assert_niche::<Niche255<Tuple<ImpossibleZstStruct, ImpossibleZstStruct>>>();
// };

// TODO: When const traits become available, add assert_same_type.
