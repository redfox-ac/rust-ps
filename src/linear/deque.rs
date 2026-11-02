use crate::linear::node::DoublyNode;

pub struct Deque<T> {
    head: *mut DoublyNode<T>,
    tail: *mut DoublyNode<T>,
    size: usize,
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Deque<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut queue = Self::new();
        for p in vec.into_iter() {
            queue.push_back(p);
        }
        queue
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self { head: std::ptr::null_mut(), tail: std::ptr::null_mut(), size: 0 }
    }

    // =====

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // ===

    pub fn push_front(&mut self, data: T) {
        let node = Box::into_raw(Box::new(DoublyNode { data, prev: std::ptr::null_mut(), next: self.head }));

        if self.head.is_null() {
            self.head = node;
            self.tail = node;
        }
        else {
            unsafe {
                (*self.head).prev = node;
                self.head = node;
            }
        }
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let node = Box::into_raw(Box::new(DoublyNode { data, prev: self.tail, next: std::ptr::null_mut() }));

        if self.tail.is_null() {
            self.tail = node;
            self.head = node;
        }
        else {
            unsafe {
                (*self.tail).next = node;
                self.tail = node;
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            let node = Box::from_raw(self.head);
            self.head = node.next;
            self.size -= 1;
            if self.head.is_null() {
                self.tail = std::ptr::null_mut();
            }
            else {
                (*self.head).prev = std::ptr::null_mut();
            }
            Some(node.data)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            let node = Box::from_raw(self.tail);
            self.tail = node.prev;
            self.size -= 1;
            if self.tail.is_null() {
                self.head = std::ptr::null_mut();
            }
            else {
                (*self.tail).next = std::ptr::null_mut();
            }
            Some(node.data)
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.data) }
    }
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.data) }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.data) }
    }
    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.data) }
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}