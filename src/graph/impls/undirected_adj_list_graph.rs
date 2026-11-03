use crate::graph::{AdjListGraph, Edge, Graph, UndirectedGraph};

pub struct UndirectedAdjListGraph<V, W> {
    graph: AdjListGraph<V, W>,
    loops: usize,
}

impl<V, W> Graph for UndirectedAdjListGraph<V, W> {
    type Vertex = V;
    type Weight = W;

    fn new(n: usize) -> Self where V: Default {
        UndirectedAdjListGraph { graph: AdjListGraph::new(n), loops: 0 }
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

impl<V, W> UndirectedGraph for UndirectedAdjListGraph<V, W> {}

impl<V, W: Copy> UndirectedAdjListGraph<V, W> {
    pub fn add_edge(&mut self, a: usize, b: usize, weight: W) {
        self.graph.add_edge(a, b, weight);
        if a != b {
            self.graph.add_edge(b, a, weight);
        } else {
            self.loops += 1;
        }
    }

    pub fn remove_edge(&mut self, a: usize, b: usize) -> Option<W> {
        let weight = self.graph.remove_edge(a, b)?;
        if a != b {
            self.graph.remove_edge(b, a);
        } else {
            self.loops -= 1;
        }
        Some(weight)
    }

    pub fn value_mut(&mut self, v: usize) -> &mut V {
        self.graph.value_mut(v)
    }
}
