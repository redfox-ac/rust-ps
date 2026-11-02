pub mod string {
    mod kmp;
    
    pub use kmp::KMP;
}

pub mod tree {
    mod monoid;
    mod segment_tree;
    mod lazy_segment_tree;
    mod fenwick_tree;

    pub use monoid::{Group, MapMonoid, Monoid};
    pub use segment_tree::{SegmentTree, SegmentTreeGuard};
    pub use lazy_segment_tree::{LazySegmentTree, LazySegmentTreeGuard};
    pub use fenwick_tree::{FenwickTree, FenwickTreeGuard};
}