pub(crate) struct Node<T> {
    pub(crate) data: T,
    pub(crate) link: Option<Box<Node<T>>>
}

pub(crate) struct DoublyNode<T> {
    pub(crate) data: T,
    pub(crate) prev: *mut DoublyNode<T>,
    pub(crate) next: *mut DoublyNode<T>,
}