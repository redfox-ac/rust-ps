pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub trait MapMonoid: Monoid {
    type F: Clone;
    fn id() -> Self::F;
    fn compose(f: &Self::F, g: &Self::F) -> Self::F;
    fn mapping(f: &Self::F, x: &Self::S, len: usize) -> Self::S;
}

pub trait Group: Monoid {
    fn inv(a: &Self::S) -> Self::S;
}
