use std::cmp::Reverse;
use crate::graph::{Edge, UndirectedGraph};
use crate::linear::Heap;

pub trait Prim: UndirectedGraph {
    fn prim(&self) -> Vec<Edge<Self::Weight>>
    where
        Self::Weight: Ord + Copy;
}

impl<G: UndirectedGraph> Prim for G {
    fn prim(&self) -> Vec<Edge<G::Weight>>
    where
        G::Weight: Ord + Copy,
    {
        let mut mst = Vec::new();

        let mut visited = vec![false; self.len()];
        let mut best = vec![None; self.len()];
        let mut heap = Heap::new();

        for start in 0..self.len() {
            if visited[start] {
                continue;
            }

            visited[start] = true;
            for Edge { from: u, to: v, weight: &w } in self.adj(start) {
                if !visited[v] && best[v].is_none_or(|b| w < b) {
                    best[v] = Some(w);
                    heap.push(Reverse(Edge { from: u, to: v, weight: w }));
                }
            }

            while let Some(Reverse(Edge { from: u, to: v, weight: w })) = heap.pop() {
                if !visited[v] {
                    visited[v] = true;
                    mst.push(Edge { from: u, to: v, weight: w });

                    if mst.len() + 1 == self.len() {
                        break;
                    }

                    for Edge { from: u, to: v, weight: &w } in self.adj(v) {
                        if !visited[v] && best[v].is_none_or(|b| w < b) {
                            best[v] = Some(w);
                            heap.push(Reverse(Edge { from: u, to: v, weight: w }));
                        }
                    }
                }
            }
        }
        mst
    }
}