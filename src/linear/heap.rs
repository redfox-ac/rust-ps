pub struct Heap<T: Ord> {
    vec: Vec<T>,
}

impl<T: Ord> Default for Heap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for Heap<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut heap = Self { vec };
        for idx in (0..(heap.len() >> 1)).rev() {
            heap.sift_down(idx)
        }
        heap
    }
}

impl<T: Ord> Heap<T> {

    pub fn new() -> Self {
        Heap { vec: Vec::new() }
    }

    // =====

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    // =====

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.sift_up(self.vec.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ret = self.vec.swap_remove(0);
        if self.len() > 1 {
            self.sift_down(0);
        }
        Some(ret)
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    // =====

    fn sift_down(&mut self, root: usize) {
        let len = self.len();
        let mut parent = root;
        let mut child = root << 1 | 1;

        while child < len {
            if child + 1 != len && self.vec[child] < self.vec[child + 1] {
                child += 1;
            }

            if self.vec[parent] < self.vec[child] {
                self.vec.swap(parent, child);
                parent = child;
            }
            else {
                break;
            }
            child = parent << 1 | 1;
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) >> 1;
            if self.vec[parent] >= self.vec[idx] {
                break;
            }
            self.vec.swap(parent, idx);
            idx = parent;
        }
    }
}