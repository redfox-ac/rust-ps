use std::ops::{Deref, DerefMut, RangeBounds, Bound};
use crate::tree::monoid::MapMonoid;

pub struct LazySegmentTree<M: MapMonoid> {
    n: usize,
    size: usize,
    tree: Vec<M::S>,
    lazy: Vec<M::F>,
}

pub struct LazySegmentTreeGuard<'a, M: MapMonoid> {
    st: &'a mut LazySegmentTree<M>,
    left: usize,
    right: usize,
    delta: M::F,
    modified: bool,
}

impl<M: MapMonoid> From<Vec<M::S>> for LazySegmentTree<M> {
    fn from(vec: Vec<M::S>) -> Self {
        let mut seg = Self::new(vec.len());

        for (i, value) in vec.into_iter().enumerate() {
            seg.tree[seg.size + i] = value;
        }
        for i in (1..seg.size).rev() {
            seg.tree[i] = M::op(&seg.tree[i << 1], &seg.tree[i << 1 | 1]);
        }

        seg
    }
}

impl<M: MapMonoid> LazySegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();

        LazySegmentTree {
            n,
            size,
            tree: (0..(size << 1)).map(|_| M::e()).collect(),
            lazy: (0..size).map(|_| M::id()).collect(),
        }
    }

    pub fn at<R: RangeBounds<usize>>(&mut self, range: R) -> LazySegmentTreeGuard<'_, M> {
        let (left, right) = self.bounds(&range);
        LazySegmentTreeGuard { st: self, left, right, delta: M::id(), modified: false }
    }

    fn bounds<R: RangeBounds<usize>>(&self, range: &R) -> (usize, usize) {
        let left = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let right = match range.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.n,
        };

        assert!(left <= right && right <= self.n);

        (left, right)
    }

    pub fn update<R: RangeBounds<usize>>(&mut self, range: R, delta: M::F) {
        let (l, r) = self.bounds(&range);
        let (l, r) = (l + self.size, r + self.size);

        if l >= r {
            return;
        }

        self.push(l, r);

        let (mut left, mut right) = (l, r);
        let mut len = 1;
        while left < right {
            if left & 1 == 1 {
                self.apply(left, &delta, len);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                self.apply(right, &delta, len);
            }

            left >>= 1;
            right >>= 1;
            len <<= 1;
        }

        self.pull(l, r);
    }

    pub fn query<R: RangeBounds<usize>>(&mut self, range: R) -> M::S {
        let (l, r) = self.bounds(&range);
        let (mut left, mut right) = (l + self.size, r + self.size);

        self.push(left, right);

        let mut sl = M::e();
        let mut sr = M::e();

        while left < right {
            if left & 1 == 1 {
                sl = M::op(&sl, unsafe { self.tree.get_unchecked(left) });
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                sr = M::op(unsafe { self.tree.get_unchecked(right) }, &sr);
            }

            left >>= 1;
            right >>= 1;
        }

        M::op(&sl, &sr)
    }

    fn push(&mut self, left: usize, right: usize) {
        for i in (1..=self.size.trailing_zeros() as usize).rev() {
            if left >> i << i != left {
                self.propagate(left >> i, 1 << i);
            }
            if right >> i << i != right {
                self.propagate(right >> i, 1 << i);
            }
        }
    }

    fn pull(&mut self, left: usize, right: usize) {
        for i in 1..=self.size.trailing_zeros() as usize {
            if left >> i << i != left {
                unsafe {
                    *self.tree.get_unchecked_mut(left >> i) = M::op(self.tree.get_unchecked(left >> i << 1), self.tree.get_unchecked(left >> i << 1 | 1));
                };
            }
            if right >> i << i != right {
                unsafe {
                    *self.tree.get_unchecked_mut(right >> i) = M::op(self.tree.get_unchecked(right >> i << 1), self.tree.get_unchecked(right >> i << 1 | 1));
                }
            }
        }
    }

    #[inline]
    fn propagate(&mut self, node: usize, len: usize) {
        unsafe {
            let lazy = std::mem::replace(self.lazy.get_unchecked_mut(node), M::id());
            self.apply(node << 1, &lazy, len >> 1);
            self.apply(node << 1 | 1, &lazy, len >> 1);
        }
    }

    fn apply(&mut self, node: usize, delta: &M::F, len: usize) {
        unsafe {
            let value = self.tree.get_unchecked_mut(node);
            *value = M::mapping(delta, value, len);
            if len > 1 {
                let lazy = self.lazy.get_unchecked_mut(node);
                *lazy = M::compose(delta, lazy);
            }
        }
    }
}

impl<'a, M: MapMonoid> Deref for LazySegmentTreeGuard<'a, M> {
    type Target = M::F;
    fn deref(&self) -> &Self::Target { &self.delta }
}

impl<'a, M: MapMonoid> DerefMut for LazySegmentTreeGuard<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.modified = true;
        &mut self.delta
    }
}

impl<'a, M: MapMonoid> Drop for LazySegmentTreeGuard<'a, M> {
    fn drop(&mut self) {
        if self.modified {
            self.st.update(self.left..self.right, self.delta.clone());
        }
    }
}
