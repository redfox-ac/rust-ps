pub mod linear {
    mod node;
    mod stack;
    mod queue;
    mod deque;
    mod heap;

    pub use stack::Stack;
    pub use queue::Queue;
    pub use deque::Deque;
    pub use heap::Heap;
}