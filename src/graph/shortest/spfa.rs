use crate::graph::{Distance, Graph, SignedDistance};
use crate::graph::shortest::ShortestPaths;
use crate::linear::Queue;

pub trait Spfa: Graph {
    fn spfa(&self, start: usize) -> Option<ShortestPaths<Self::Weight>>
    where
        Self::Weight: SignedDistance;
}

impl<G: Graph> Spfa for G {
    fn spfa(&self, start: usize) -> Option<ShortestPaths<G::Weight>>
    where
        G::Weight: SignedDistance,
    {
        let mut dist = vec![G::Weight::INF; self.len()];
        let mut prev = vec![None; self.len()];

        let mut queue = Queue::new();
        let mut in_queue = vec![false; self.len()];
        let mut cnt = vec![0; self.len()];

        dist[start] = G::Weight::ZERO;
        prev[start] = Some(start);
        queue.push(start);
        in_queue[start] = true;

        while let Some(v) = queue.pop() {
            cnt[v] += 1;
            in_queue[v] = false;

            if cnt[v] > self.len() {
                return None;
            }

            for (next, &cost) in self.adj(v) {
                if dist[next] > dist[v] + cost {
                    dist[next] = dist[v] + cost;
                    prev[next] = Some(v);
                    
                    if !in_queue[next] {
                        queue.push(next);
                        in_queue[next] = true;
                    }
                }
            }
        }

        Some(ShortestPaths { dist, prev })
    }
}
