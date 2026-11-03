use crate::graph::{Edge, Graph};

pub struct CsrGraph<V, W> {
    offset: Vec<usize>,
    edges: Vec<(usize, W)>,
    values: Vec<V>,
}

impl<V, W> Graph for CsrGraph<V, W> {
    type Vertex = V;
    type Weight = W;

    fn new(n: usize) -> Self where V: Default {
        CsrGraph {
            offset: vec![0; n + 1],
            edges: Vec::new(),
            values: (0..n).map(|_| V::default()).collect(),
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn adj(&self, v: usize) -> impl Iterator<Item = Edge<&W>> {
        self.edges[self.offset[v]..self.offset[v + 1]]
            .iter()
            .map(move |(to, weight)| Edge { from: v, to: *to, weight })
    }

    fn value(&self, v: usize) -> &V {
        &self.values[v]
    }
}

impl<V, W> CsrGraph<V, W> {
    pub fn from_edges<E: Into<Edge<W>>>(n: usize, edges: Vec<E>) -> Self where V: Default {
        let edges: Vec<Edge<W>> = edges.into_iter().map(Into::into).collect();

        let mut offset = vec![0; n + 1];
        for e in &edges {
            assert!(e.from < n && e.to < n);
            offset[e.from + 1] += 1;
        }
        for v in 0..n {
            offset[v + 1] += offset[v];
        }
        
        let mut next = offset.clone();
        let mut slots: Vec<Option<(usize, W)>> = (0..edges.len()).map(|_| None).collect();
        for Edge { from, to, weight } in edges {
            slots[next[from]] = Some((to, weight));
            next[from] += 1;
        }

        CsrGraph {
            offset,
            edges: slots.into_iter().map(|e| e.unwrap()).collect(),
            values: (0..n).map(|_| V::default()).collect(),
        }
    }

    pub fn value_mut(&mut self, v: usize) -> &mut V {
        &mut self.values[v]
    }
}
