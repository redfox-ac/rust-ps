use crate::graph::{Distance, Graph, SignedDistance};
use crate::graph::shortest::ShortestPaths;

pub fn bellman_ford<G: Graph>(graph: &G, start: usize) -> Option<ShortestPaths<G::Weight>>
where
    G::Weight: SignedDistance,
{
    let mut dist = vec![G::Weight::INF; graph.len()];
    let mut prev = vec![None; graph.len()];

    dist[start] = G::Weight::ZERO;
    prev[start] = Some(start);

    for _ in 1..graph.len() {
        for v in 0..graph.len() {
            if dist[v] == G::Weight::INF {
                continue;
            }

            for (next, &cost) in graph.adj(v) {
                if dist[next] > dist[v] + cost {
                    dist[next] = dist[v] + cost;
                    prev[next] = Some(v);
                }
            }
        }
    }
    
    for v in 0..graph.len() {
        if dist[v] == G::Weight::INF {
            continue;
        }
        for (next, &cost) in graph.adj(v) {
            if dist[next] > dist[v] + cost {
                return None;
            }
        }
    }

    Some(ShortestPaths { dist, prev })
}
