use crate::{
    nodes::{node::Node, TreeNode},
    store::Store,
};

pub struct SMT<T: TreeNode + Clone> {
    root: Node,
    store: Store<T>,
    height: u8,
}

impl<T: TreeNode + Clone> SMT<T> {}
