use std::{
    fmt::{
        Binary,
        LowerExp, UpperExp,
        LowerHex, UpperHex,
        Octal,
        Pointer,
        Write as FmtWrite,
        Debug,  Display,
    },
    marker::{
        Send, Sync,
    },
    net::{
        ToSocketAddrs,
    },
    ops::{
        Deref,  DerefMut,
        Index,  IndexMut,
        Add,    AddAssign,
        Sub,    SubAssign,
        Mul,    MulAssign,
        Div,    DivAssign,
        Rem,    RemAssign,
        Shl,    ShlAssign,
        Shr,    ShrAssign,
        BitAnd, BitAndAssign,
        BitOr,  BitOrAssign,
        BitXor, BitXorAssign,
        Neg,    Not,
    },
    borrow::{
        Borrow, BorrowMut,
    },
    convert::{
        AsRef, AsMut,
    },
    cmp::{
        PartialEq,
        PartialOrd,
    },
    error::Error,
};


macro_rules! aligns {
    ($(
        $name:ident($align:literal)
    ),*$(,)?) => {
        $(
            #[repr(C, align($align))]
            #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name<T = ()>(pub T);

            impl<T> $name<T> {
                #[must_use]
                #[inline]
                pub const fn new(inner: T) -> Self {
                    Self(inner)
                }

                pub fn map<F: FnOnce(T) -> T>(self, map: F) -> Self {
                    Self((map)(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align1<F: FnOnce(T) -> R, R>(self, map: F) -> Align1<R> {
                    Align1(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align2<F: FnOnce(T) -> R, R>(self, map: F) -> Align2<R> {
                    Align2(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align4<F: FnOnce(T) -> R, R>(self, map: F) -> Align4<R> {
                    Align4(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align8<F: FnOnce(T) -> R, R>(self, map: F) -> Align8<R> {
                    Align8(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align16<F: FnOnce(T) -> R, R>(self, map: F) -> Align16<R> {
                    Align16(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align32<F: FnOnce(T) -> R, R>(self, map: F) -> Align32<R> {
                    Align32(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align64<F: FnOnce(T) -> R, R>(self, map: F) -> Align64<R> {
                    Align64(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align128<F: FnOnce(T) -> R, R>(self, map: F) -> Align128<R> {
                    Align128(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align256<F: FnOnce(T) -> R, R>(self, map: F) -> Align256<R> {
                    Align256(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align512<F: FnOnce(T) -> R, R>(self, map: F) -> Align512<R> {
                    Align512(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align1024<F: FnOnce(T) -> R, R>(self, map: F) -> Align1024<R> {
                    Align1024(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align2048<F: FnOnce(T) -> R, R>(self, map: F) -> Align2048<R> {
                    Align2048(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align4096<F: FnOnce(T) -> R, R>(self, map: F) -> Align4096<R> {
                    Align4096(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn map_align8192<F: FnOnce(T) -> R, R>(self, map: F) -> Align8192<R> {
                    Align8192(map(self.0))
                }

                #[must_use]
                #[inline]
                pub fn to_align1(self) -> Align1<T> {
                    Align1(self.0)
                }

                #[must_use]
                #[inline]
                pub fn to_align2(self) -> Align2<T> {
                    Align2(self.0)
                }

                #[must_use]
                #[inline]
                pub fn to_align4(self) -> Align4<T> {
                    Align4(self.0)
                }

                #[must_use]
                #[inline]
                pub fn to_align8(self) -> Align8<T> {
                    Align8(self.0)
                }

                #[must_use]
                #[inline]
                pub fn to_align16(self) -> Align16<T> {
                    Align16(self.0)
                }
            }

            impl<T: PartialEq<T>> PartialEq<T> for $name<T> {
                #[inline]
                fn eq(&self, other: &T) -> bool {
                    self.0 == *other
                }

                #[inline]
                fn ne(&self, other: &T) -> bool {
                    self.0 != *other
                }
            }

            impl<T: PartialOrd<T>> PartialOrd<T> for $name<T> {
                #[inline]
                fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                    self.0.partial_cmp(other)
                }

                #[inline]
                fn ge(&self, other: &T) -> bool {
                    self.0.ge(other)
                }

                #[inline]
                fn gt(&self, other: &T) -> bool {
                    self.0.gt(other)
                }

                #[inline]
                fn le(&self, other: &T) -> bool {
                    self.0.le(other)
                }

                #[inline]
                fn lt(&self, other: &T) -> bool {
                    self.0.lt(other)
                }
            }

            impl<T: Binary> Binary for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: LowerExp> LowerExp for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: UpperExp> UpperExp for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: LowerHex> LowerHex for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: UpperHex> UpperHex for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: Octal> Octal for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: Pointer> Pointer for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: FmtWrite> FmtWrite for $name<T> {
                #[inline]
                fn write_char(&mut self, c: char) -> std::fmt::Result {
                    self.0.write_char(c)
                }

                #[inline]
                fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
                    self.0.write_fmt(args)
                }

                #[inline]
                fn write_str(&mut self, s: &str) -> std::fmt::Result {
                    self.0.write_str(s)
                }
            }

            impl<T: Debug> Debug for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: Display> Display for $name<T> {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            unsafe impl<T: Send> Send for $name<T> {}
            unsafe impl<T: Sync> Sync for $name<T> {}
            
            impl<T: ToSocketAddrs> ToSocketAddrs for $name<T> {
                type Iter = T::Iter;

                #[inline]
                fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
                    self.0.to_socket_addrs()
                }
            }

            impl<T> Deref for $name<T> {
                type Target = T;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> DerefMut for $name<T> {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<I, T: Index<I>> Index<I> for $name<T> {
                type Output = T::Output;
                #[inline]
                fn index(&self, index: I) -> &Self::Output {
                    &self.0[index]
                }
            }

            impl<I, T: IndexMut<I>> IndexMut<I> for $name<T> {
                #[inline]
                fn index_mut(&mut self, index: I) -> &mut Self::Output {
                    &mut self.0[index]
                }
            }

            impl<T: Add<T, Output = T>> Add<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }

            impl<T: Add<T, Output = T>> Add<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn add(self, rhs: T) -> Self::Output {
                    Self(self.0 + rhs)
                }
            }

            impl<T: AddAssign<T>> AddAssign<Self> for $name<T> {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0;
                }
            }

            impl<T: AddAssign<T>> AddAssign<T> for $name<T> {
                #[inline]
                fn add_assign(&mut self, rhs: T) {
                    self.0 += rhs;
                }
            }

            impl<T: Sub<T, Output = T>> Sub<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    Self(self.0 - rhs.0)
                }
            }

            impl<T: Sub<T, Output = T>> Sub<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn sub(self, rhs: T) -> Self::Output {
                    Self(self.0 - rhs)
                }
            }

            impl<T: SubAssign<T>> SubAssign<Self> for $name<T> {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0;
                }
            }

            impl<T: SubAssign<T>> SubAssign<T> for $name<T> {
                #[inline]
                fn sub_assign(&mut self, rhs: T) {
                    self.0 -= rhs;
                }
            }

            impl<T: Mul<T, Output = T>> Mul<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn mul(self, rhs: Self) -> Self::Output {
                    Self(self.0.mul(rhs.0))
                }
            }

            impl<T: Mul<T, Output = T>> Mul<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn mul(self, rhs: T) -> Self::Output {
                    Self(self.0.mul(rhs))
                }
            }

            impl<T: MulAssign<T>> MulAssign<Self> for $name<T> {
                #[inline]
                fn mul_assign(&mut self, rhs: Self) {
                    self.0.mul_assign(rhs.0);
                }
            }

            impl<T: MulAssign<T>> MulAssign<T> for $name<T> {
                #[inline]
                fn mul_assign(&mut self, rhs: T) {
                    self.0.mul_assign(rhs);
                }
            }

            impl<T: Div<T, Output = T>> Div<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn div(self, rhs: Self) -> Self::Output {
                    Self(self.0.div(rhs.0))
                }
            }

            impl<T: Div<T, Output = T>> Div<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn div(self, rhs: T) -> Self::Output {
                    Self(self.0.div(rhs))
                }
            }
            
            impl<T: DivAssign<T>> DivAssign<Self> for $name<T> {
                #[inline]
                fn div_assign(&mut self, rhs: Self) {
                    self.0.div_assign(rhs.0);
                }
            }
            
            impl<T: DivAssign<T>> DivAssign<T> for $name<T> {
                #[inline]
                fn div_assign(&mut self, rhs: T) {
                    self.0.div_assign(rhs);
                }
            }

            impl<T: Rem<T, Output = T>> Rem<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn rem(self, rhs: Self) -> Self::Output {
                    Self(self.0.rem(rhs.0))
                }
            }

            impl<T: Rem<T, Output = T>> Rem<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn rem(self, rhs: T) -> Self::Output {
                    Self(self.0.rem(rhs))
                }
            }

            impl<T: RemAssign<T>> RemAssign<Self> for $name<T> {
                #[inline]
                fn rem_assign(&mut self, rhs: Self) {
                    self.0.rem_assign(rhs.0);
                }
            }

            impl<T: RemAssign<T>> RemAssign<T> for $name<T> {
                #[inline]
                fn rem_assign(&mut self, rhs: T) {
                    self.0.rem_assign(rhs);
                }
            }

            impl<R, T: Shl<R, Output = T>> Shl<R> for $name<T> {
                type Output = Self;
                #[inline]
                fn shl(self, rhs: R) -> Self::Output {
                    Self(self.0.shl(rhs))
                }
            }

            impl<R, T: ShlAssign<R>> ShlAssign<R> for $name<T> {
                #[inline]
                fn shl_assign(&mut self, rhs: R) {
                    self.0.shl_assign(rhs);
                }
            }

            impl<R, T: Shr<R, Output = T>> Shr<R> for $name<T> {
                type Output = Self;
                #[inline]
                fn shr(self, rhs: R) -> Self::Output {
                    Self(self.0.shr(rhs))
                }
            }

            impl<R, T: ShrAssign<R>> ShrAssign<R> for $name<T> {
                #[inline]
                fn shr_assign(&mut self, rhs: R) {
                    self.0.shr_assign(rhs);
                }
            }

            impl<T: BitAnd<T, Output = T>> BitAnd<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitand(self, rhs: Self) -> Self::Output {
                    Self(self.0.bitand(rhs.0))
                }
            }

            impl<T: BitAnd<T, Output = T>> BitAnd<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitand(self, rhs: T) -> Self::Output {
                    Self(self.0.bitand(rhs))
                }
            }

            impl<T: BitAndAssign<T>> BitAndAssign<Self> for $name<T> {
                #[inline]
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0);
                }
            }

            impl<T: BitAndAssign<T>> BitAndAssign<T> for $name<T> {
                #[inline]
                fn bitand_assign(&mut self, rhs: T) {
                    self.0.bitand_assign(rhs);
                }
            }

            impl<T: BitOr<T, Output = T>> BitOr<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self(self.0.bitor(rhs.0))
                }
            }

            impl<T: BitOr<T, Output = T>> BitOr<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitor(self, rhs: T) -> Self::Output {
                    Self(self.0.bitor(rhs))
                }
            }

            impl<T: BitOrAssign<T>> BitOrAssign<Self> for $name<T> {
                #[inline]
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0);
                }
            }

            impl<T: BitOrAssign<T>> BitOrAssign<T> for $name<T> {
                #[inline]
                fn bitor_assign(&mut self, rhs: T) {
                    self.0.bitor_assign(rhs);
                }
            }

            impl<T: BitXor<T, Output = T>> BitXor<Self> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitxor(self, rhs: Self) -> Self::Output {
                    Self(self.0.bitxor(rhs.0))
                }
            }

            impl<T: BitXor<T, Output = T>> BitXor<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn bitxor(self, rhs: T) -> Self::Output {
                    Self(self.0.bitxor(rhs))
                }
            }

            impl<T: BitXorAssign<T>> BitXorAssign<Self> for $name<T> {
                #[inline]
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0.bitxor_assign(rhs.0);
                }
            }

            impl<T: BitXorAssign<T>> BitXorAssign<T> for $name<T> {
                #[inline]
                fn bitxor_assign(&mut self, rhs: T) {
                    self.0.bitxor_assign(rhs);
                }
            }

            impl<T: Neg<Output = T>> Neg for $name<T> {
                type Output = Self;
                #[inline]
                fn neg(self) -> Self::Output {
                    Self(self.0.neg())
                }
            }

            impl<T: Not<Output = T>> Not for $name<T> {
                type Output = Self;
                #[inline]
                fn not(self) -> Self::Output {
                    Self(self.0.not())
                }
            }

            impl<E: Error> Error for $name<E>
            where Self: Display + Debug {
                fn source(&self) -> Option<&(dyn Error + 'static)> {
                    self.0.source()
                }
            }

            impl<T> Borrow<T> for $name<T> {
                #[inline]
                fn borrow(&self) -> &T {
                    &self.0
                }
            }

            impl<T> BorrowMut<T> for $name<T> {
                #[inline]
                fn borrow_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }

            impl<R, T: AsRef<R>> AsRef<R> for $name<T> {
                #[inline]
                fn as_ref(&self) -> &R {
                    self.0.as_ref()
                }
            }

            impl<R, T: AsMut<R>> AsMut<R> for $name<T> {
                #[inline]
                fn as_mut(&mut self) -> &mut R {
                    self.0.as_mut()
                }
            }
        )*
    };
}


aligns!{
    Align1(1),
    Align2(2),
    Align4(4),
    Align8(8),
    Align16(16),
    Align32(32),
    Align64(64),
    Align128(128),
    Align256(256),
    Align512(512),
    Align1024(1024),
    Align2048(2048),
    Align4096(4096),
    Align8192(8192),
}

#[cfg(test)]
mod tests {
    use std::mem::offset_of;

    use crate::pad::Pad;

    use super::*;

    struct Temp<T>(pub T);

    #[repr(C)]
    struct Fnoof {
        low: u32,
        _align0: Align8,
        _pad0: Pad<2>,
        high: u8,
    }

    #[test]
    #[ignore]
    fn sandbox0() {
        let low_offset = offset_of!(Fnoof, low);
        let high_offset = offset_of!(Fnoof, high);
        assert_eq!(low_offset, 0);
        assert_eq!(high_offset, 10);
    }
    
    #[test]
    fn coverage_test() {
        // #[derive(Debug)]
        // struct Fnord {
        //     foo: u64,
        //     bar: &'static str,
        //     baz: bool,
        // }
        // let aligned = Align16(Fnord {
        //     foo: 1234,
        //     bar: "hello, world!",
        //     baz: false,
        // });
        let aligned_u64 = Align16(100u64).map_align32(|v| v);
        let result = aligned_u64 + 10;
        match result.0 {
            110 => {
                println!("It's 110 time.");
            },
            _ => (),
        }
        assert_eq!(result, 110);
        let mut aligned = Align16([0u8; 16]);
        for i in 0..16 {
            aligned[i] = i as u8;
        }
        println!("{aligned:#?}");
    }
}