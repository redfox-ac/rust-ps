use std::cmp::Reverse;
use crate::graph::{Distance, Graph};
use crate::graph::shortest::ShortestPaths;
use crate::linear::Heap;

pub trait Dijkstra: Graph {
    fn dijkstra(&self, start: usize) -> ShortestPaths<Self::Weight>
    where
        Self::Weight: Distance;
}

impl<G: Graph> Dijkstra for G {
    fn dijkstra(&self, start: usize) -> ShortestPaths<G::Weight>
    where
        G::Weight: Distance,
    {
        let mut dist = vec![G::Weight::INF; self.len()];
        let mut prev = vec![None; self.len()];
        let mut visited = vec![false; self.len()];

        dist[start] = G::Weight::ZERO;
        prev[start] = Some(start);

        let mut pq = Heap::new();
        pq.push(Reverse((G::Weight::ZERO, start)));

        while let Some(Reverse((d, idx))) = pq.pop() {
            visited[idx] = true;

            for (next, &cost) in self.adj(idx) {
                if !visited[next] && dist[next] > d + cost {
                    dist[next] = d + cost;
                    prev[next] = Some(idx);

                    pq.push(Reverse((dist[next], next)));
                }
            }
        }

        ShortestPaths { dist, prev }
    }
}
