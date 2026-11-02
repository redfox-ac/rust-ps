use std::ops::{Bound, Deref, DerefMut, RangeBounds};
use crate::tree::monoid::Monoid;

pub struct SegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    tree: Vec<M::S>,
}

pub struct SegmentTreeGuard<'a, M: Monoid> {
    st: &'a mut SegmentTree<M>,
    idx: usize,
    value: M::S,
    modified: bool,
}

impl<M: Monoid> From<Vec<M::S>> for SegmentTree<M> {
    fn from(vec: Vec<M::S>) -> Self {
        let mut seg = Self::new(vec.len());

        for (i, value) in vec.into_iter().enumerate() {
            seg.tree[seg.size + i] = value;
        }
        for p in (1..seg.size).rev() {
            seg.tree[p] = M::op(&seg.tree[p << 1], &seg.tree[p << 1 | 1]);
        }

        seg
    }
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let tree = (0..(size << 1)).map(|_| M::e()).collect();

        SegmentTree {
            n,
            size,
            tree,
        }
    }

    // =====

    pub fn at(&mut self, idx: usize) -> SegmentTreeGuard<'_, M> {
        assert!(idx < self.n);
        let value = self.tree[self.size + idx].clone();
        SegmentTreeGuard { st: self, idx, value, modified: false }
    }

    // =====

    fn bounds<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let left = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        } + self.size;

        let right = match range.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.n,
        } + self.size;

        (left, right)
    }

    // =====

    pub fn len(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    // =====

    pub fn update(&mut self, idx: usize, value: M::S) {
        assert!(idx < self.n);

        let mut idx = idx | self.size;
        self.tree[idx] = value;
        while idx > 1 {
            idx >>= 1;
            self.tree[idx] = M::op(&self.tree[idx << 1], &self.tree[idx << 1 | 1]);
        }
    }

    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> M::S {
        let (mut left, mut right) = self.bounds(range);

        assert!(left <= right && right <= self.n + self.size);

        let mut sl = M::e();
        let mut sr = M::e();

        while left < right {
            if left & 1 == 1 {
                sl = M::op(&sl, &self.tree[left]);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                sr = M::op(&self.tree[right], &sr);
            }

            left >>= 1;
            right >>= 1;
        }

        M::op(&sl, &sr)
    }
}

impl<M: Monoid> Deref for SegmentTreeGuard<'_, M> {
    type Target = M::S;
    fn deref(&self) -> &M::S {
        &self.value
    }
}

impl<M: Monoid> DerefMut for SegmentTreeGuard<'_, M> {
    fn deref_mut(&mut self) -> &mut M::S {
        self.modified = true;
        &mut self.value
    }
}

impl<M: Monoid> Drop for SegmentTreeGuard<'_, M> {
    fn drop(&mut self) {
        if self.modified {
            self.st.update(self.idx, self.value.clone());
        }
    }
}