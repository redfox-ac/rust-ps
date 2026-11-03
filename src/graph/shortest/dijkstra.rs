use std::cmp::Reverse;
use crate::graph::{Distance, Graph};
use crate::graph::shortest::ShortestPaths;
use crate::linear::Heap;

pub fn dijkstra<G: Graph>(graph: &G, start: usize) -> ShortestPaths<G::Weight>
where
    G::Weight: Distance,
{
    let mut dist = vec![G::Weight::INF; graph.len()];
    let mut prev = vec![None; graph.len()];
    let mut visited = vec![false; graph.len()];

    dist[start] = G::Weight::ZERO;
    prev[start] = Some(start);

    let mut pq = Heap::new();
    pq.push(Reverse((G::Weight::ZERO, start)));

    while let Some(Reverse((d, idx))) = pq.pop() {
        visited[idx] = true;

        // 최단 거리 배열 갱신
        for (next, &cost) in graph.adj(idx) {
            if !visited[next] && dist[next] > d + cost {
                dist[next] = d + cost;
                prev[next] = Some(idx);

                pq.push(Reverse((dist[next], next)));
            }
        }
    }

    ShortestPaths { dist, prev }
}