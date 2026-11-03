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

    fn adj(&self, v: usize) -> impl Iterator<Item = (usize, &Self::Weight)>;

    // =====

    fn value(&self, v: usize) -> &Self::Vertex;
}
