use crate::graph::{Edge, UndirectedGraph};
use crate::set::UnionFind;

pub trait Kruskal: UndirectedGraph {
    fn kruskal(&self) -> Vec<Edge<Self::Weight>>
    where
        Self::Weight: Ord + Copy;
}

impl<G: UndirectedGraph> Kruskal for G {
    fn kruskal(&self) -> Vec<Edge<G::Weight>>
    where
        G::Weight: Ord + Copy,
    {
        let mut edge = Vec::with_capacity(self.edge_count());
        edge.extend(self.edges().map(|e| Edge { from: e.from, to: e.to, weight: *e.weight }));
        edge.sort_by_key(|e| e.weight);

        let mut node = UnionFind::new(self.len());
        let mut mst = Vec::new();

        for Edge { from: u, to: v, weight: w } in edge {
            if node.unite(u, v) {
                mst.push(Edge { from: u, to: v, weight: w });
            }

            if mst.len() + 1 == self.len() {
                break;
            }
        }
        mst
    }
}
