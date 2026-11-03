pub struct UnionFind {
    id: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self { id: (0..size).collect(), rank: vec![0; size] }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.id[x] != x {
            self.id[x] = self.find(self.id[x]);
        }
        self.id[x]
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }

        if self.rank[a] > self.rank[b] {
            self.id[b] = a;
        }
        else if self.rank[a] < self.rank[b] {
            self.id[a] = b;
        }
        else {
            self.id[b] = a;
            self.rank[a] += 1;
        }
        true
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}
