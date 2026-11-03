use std::ops::Add;

pub trait Distance: Copy + Ord + Add<Output = Self> {
    const ZERO: Self;
    const INF: Self;
}

pub trait SignedDistance: Distance {
    const NEG_INF: Self;
}

macro_rules! impl_distance {
    ($($t:ty),*) => { $(
        impl Distance for $t {
            const ZERO: Self = 0;
            const INF: Self = <$t>::MAX;
        }
    )* };
}
impl_distance!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_signed_distance {
    ($($t:ty),*) => { $(
        impl Distance for $t {
            const ZERO: Self = 0;
            const INF: Self = <$t>::MAX;
        }
        impl SignedDistance for $t {
            const NEG_INF: Self = <$t>::MIN;
        }
    )* };
}
impl_signed_distance!(i8, i16, i32, i64, i128, isize);
