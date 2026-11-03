use crate::graph::Edge;

pub trait Graph {
    type Vertex;
    type Weight;

    fn new(n: usize) -> Self where Self: Sized, Self::Vertex: Default;

    // =====

    fn len(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // =====

    fn adj(&self, v: usize) -> impl Iterator<Item = Edge<&Self::Weight>>;

    // =====

    fn value(&self, v: usize) -> &Self::Vertex;
}


pub trait UndirectedGraph: Graph {
    fn edges(&self) -> impl Iterator<Item = Edge<&Self::Weight>> {
        (0..self.len()).flat_map(move |u| self.adj(u).filter(|e| e.from <= e.to))
    }
}