use std::ops::{Deref, DerefMut};
use crate::tree::monoid::{Group, Monoid};

pub struct FenwickTree<M: Monoid> {
    tree: Vec<M::S>
}

pub struct FenwickTreeGuard<'a, M: Group> {
    ft: &'a mut FenwickTree<M>,
    idx: usize,
    delta: M::S
}


impl<M: Monoid> FenwickTree<M> {
    pub fn new(n: usize) -> Self {
        Self { tree: (0..=n).map(|_| M::e()).collect() }
    }

    pub fn at(&mut self, idx: usize) -> FenwickTreeGuard<'_, M> where M: Group {
        let delta = self.tree[idx].clone();
        FenwickTreeGuard { ft: self, idx, delta }
    }

    pub fn add(&mut self, mut idx: usize, x: M::S) {
        while idx < self.tree.len() {
            self.tree[idx] = M::op(&self.tree[idx], &x);
            idx += idx & idx.wrapping_neg();
        }
    }

    pub fn sub(&mut self, idx: usize, x: M::S) where M: Group {
        Self::add(self, idx, M::inv(&x));
    }

    pub fn query(&self, mut idx: usize) -> M::S {
        let mut res = M::e();
        while 1 <= idx {
            res = M::op(&res, &self.tree[idx]);
            idx -= idx & idx.wrapping_neg();
        }
        res
    }
}

impl<'a, M: Group> Deref for FenwickTreeGuard<'a, M> {
    type Target = M::S;
    fn deref(&self) -> &Self::Target { &self.delta }
}

impl<'a, M: Group> DerefMut for FenwickTreeGuard<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.delta }
}

impl<'a, M: Group> Drop for FenwickTreeGuard<'a, M> {
    fn drop(&mut self) {
        self.delta = M::op(&self.delta, &M::inv(&self.ft.tree[self.idx]));
        self.ft.add(self.idx, self.delta.clone());
    }
}
