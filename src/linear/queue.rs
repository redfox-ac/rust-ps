use crate::linear::node::Node;

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut queue = Self::new();
        for p in vec.into_iter() {
            queue.push(p);
        }
        queue
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { head: None, tail: std::ptr::null_mut(), size: 0 }
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
        let mut node = Box::new(Node { data, link: None });
        let tail: *mut Node<T> = &mut *node;

        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe {
                (*self.tail).link = Some(node);
            }
        }
        self.tail = tail;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.link;
            self.size -= 1;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
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

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}