use crate::{
    node_position::Height,
    nodes::{node::Node, TreeNode},
    store::Store,
};

pub struct SMT<T: TreeNode + Clone> {
    root: Node,
    store: Store<T>,
    height: Height,
}

impl<T: TreeNode + Clone> SMT<T> {}
