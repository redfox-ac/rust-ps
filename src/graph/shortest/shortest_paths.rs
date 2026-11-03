pub struct ShortestPaths<W> {
    pub dist: Vec<W>,
    pub prev: Vec<Option<usize>>,
}