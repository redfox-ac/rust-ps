use crate::graph::{Distance, Edge, Graph, SignedDistance};
use crate::graph::shortest::ShortestPaths;

pub trait BellmanFord: Graph {
    fn bellman_ford(&self, start: usize) -> Option<ShortestPaths<Self::Weight>>
    where
        Self::Weight: SignedDistance;
}

impl<G: Graph> BellmanFord for G {
    fn bellman_ford(&self, start: usize) -> Option<ShortestPaths<G::Weight>>
    where
        G::Weight: SignedDistance,
    {
        let mut dist = vec![G::Weight::INF; self.len()];
        let mut prev = vec![None; self.len()];

        dist[start] = G::Weight::ZERO;
        prev[start] = Some(start);

        for _ in 1..self.len() {
            for v in 0..self.len() {
                if dist[v] == G::Weight::INF {
                    continue;
                }

                for Edge { from: _, to: next, weight: &cost } in self.adj(v) {
                    if dist[next] > dist[v] + cost {
                        dist[next] = dist[v] + cost;
                        prev[next] = Some(v);
                    }
                }
            }
        }
        
        for v in 0..self.len() {
            if dist[v] == G::Weight::INF {
                continue;
            }
            for Edge { from: _, to: next, weight: &cost } in self.adj(v) {
                if dist[next] > dist[v] + cost {
                    return None;
                }
            }
        }

        Some(ShortestPaths { dist, prev })
    }
}
