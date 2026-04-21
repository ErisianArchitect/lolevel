#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pad<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Pad<SIZE> {
    pub const ZEROES: Self = Self::zeroes();

    #[must_use]
    #[inline]
    pub const fn zeroes() -> Self {
        Self([0u8; SIZE])
    }

    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self::zeroes()
    }
}

#[must_use]
#[inline]
pub const fn pad<const SIZE: usize>() -> Pad<SIZE> {
    Pad::zeroes()
}
