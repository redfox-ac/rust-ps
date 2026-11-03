use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Edge<W> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
}

impl<W> From<(usize, usize, W)> for Edge<W> {
    fn from((from, to, weight): (usize, usize, W)) -> Self {
        Self { from, to, weight }
    }
}

impl<W> From<Edge<W>> for (usize, usize, W) {
    fn from(edge: Edge<W>) -> Self {
        (edge.from, edge.to, edge.weight)
    }
}

impl<W: Debug> Debug for Edge<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} → {}, {:?})", self.from, self.to, self.weight)
    }
}

impl<W: Ord> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
            .then(self.from.cmp(&other.from))
            .then(self.to.cmp(&other.to))
    }
}

impl<W: Ord> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
