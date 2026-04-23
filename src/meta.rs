
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImpossibleZst {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OneVariant { Only }

#[repr(C)]
pub struct Tuple<T0, T1>(T0, T1);

#[repr(C, align(1))]
pub struct PackedTuple<T0, T1>(T0, T1);