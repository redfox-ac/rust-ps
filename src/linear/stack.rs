use crate::linear::node::Node;

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut stack = Self::new();
        for p in vec.into_iter() {
            stack.push(p);
        }
        stack
    }
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    // =====

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // ===

    pub fn push(&mut self, data: T) {
        let node = Box::new(Node { data, link: self.head.take() });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.link;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}