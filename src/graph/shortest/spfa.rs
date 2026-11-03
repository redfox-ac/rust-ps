use crate::graph::{Distance, Graph, SignedDistance};
use crate::graph::shortest::ShortestPaths;
use crate::linear::Queue;

pub fn spfa<G: Graph>(graph: &G, start: usize) -> Option<ShortestPaths<G::Weight>>
where
    G::Weight: SignedDistance,
{
    let mut dist = vec![G::Weight::INF; graph.len()];
    let mut prev = vec![None; graph.len()];

    let mut queue = Queue::new();
    let mut in_queue = vec![false; graph.len()];
    let mut cnt = vec![0; graph.len()];

    dist[start] = G::Weight::ZERO;
    prev[start] = Some(start);
    queue.push(start);
    in_queue[start] = true;

    while let Some(v) = queue.pop() {
        cnt[v] += 1;
        in_queue[v] = false;

        if cnt[v] > graph.len() {
            return None;
        }

        for (next, &cost) in graph.adj(v) {
            if dist[next] > dist[v] + cost {
                dist[next] = dist[v] + cost;
                prev[next] = Some(v);

                // 갱신된 정점이 큐에 없다면 큐에 삽입
                if !in_queue[next] {
                    queue.push(next);
                    in_queue[next] = true;
                }
            }
        }
    }

    Some(ShortestPaths { dist, prev })
}
