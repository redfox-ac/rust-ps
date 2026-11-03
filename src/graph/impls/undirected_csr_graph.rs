use crate::graph::{CsrGraph, Edge, Graph, UndirectedGraph};

pub struct UndirectedCsrGraph<V, W> {
    graph: CsrGraph<V, W>,
    loops: usize,
}

impl<V, W> Graph for UndirectedCsrGraph<V, W> {
    type Vertex = V;
    type Weight = W;

    fn new(n: usize) -> Self where V: Default {
        UndirectedCsrGraph { graph: CsrGraph::new(n), loops: 0 }
    }

    fn len(&self) -> usize {
        self.graph.len()
    }

    fn edge_count(&self) -> usize {
        (self.graph.edge_count() + self.loops) / 2
    }

    fn adj(&self, v: usize) -> impl Iterator<Item = Edge<&W>> {
        self.graph.adj(v)
    }

    fn value(&self, v: usize) -> &V {
        self.graph.value(v)
    }
}

impl<V, W> UndirectedGraph for UndirectedCsrGraph<V, W> {}

impl<V, W: Copy> UndirectedCsrGraph<V, W> {
    pub fn from_edges<E: Into<Edge<W>>>(n: usize, edges: Vec<E>) -> Self where V: Default {
        let mut both = Vec::with_capacity(edges.len() * 2);
        let mut loops = 0;
        for edge in edges {
            let Edge { from, to, weight } = edge.into();
            both.push(Edge { from, to, weight });
            if from != to {
                both.push(Edge { from: to, to: from, weight });
            } else {
                loops += 1;
            }
        }
        UndirectedCsrGraph { graph: CsrGraph::from_edges(n, both), loops }
    }

    pub fn value_mut(&mut self, v: usize) -> &mut V {
        self.graph.value_mut(v)
    }
}
