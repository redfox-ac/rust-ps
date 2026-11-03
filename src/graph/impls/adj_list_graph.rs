use crate::graph::{Edge, Graph};

pub struct AdjListGraph<V, W> {
    edges: Vec<Vec<(usize, W)>>,
    values: Vec<V>,
    size: usize,
}

impl<V, W> Graph for AdjListGraph<V, W> {
    type Vertex = V;
    type Weight = W;

    fn new(n: usize) -> Self where V: Default {
        AdjListGraph {
            edges: (0..n).map(|_| Vec::new()).collect(),
            values: (0..n).map(|_| V::default()).collect(),
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn edge_count(&self) -> usize {
        self.size
    }

    fn adj(&self, v: usize) -> impl Iterator<Item = Edge<&W>> {
        self.edges[v].iter().map(move |(to, weight)| Edge { from: v, to: *to, weight })
    }

    fn value(&self, v: usize) -> &V {
        &self.values[v]
    }
}

impl<V, W> AdjListGraph<V, W> {
    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        assert!(from < self.len() && to < self.len());
        self.edges[from].push((to, weight));
        self.size += 1;
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Option<W> {
        assert!(from < self.len() && to < self.len());
        let idx = self.edges[from].iter().position(|&(t, _)| t == to)?;
        self.size -= 1;
        Some(self.edges[from].remove(idx).1)
    }

    pub fn value_mut(&mut self, v: usize) -> &mut V {
        &mut self.values[v]
    }
}
