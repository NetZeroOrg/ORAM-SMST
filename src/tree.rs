use crate::{
    node_position::Height,
    nodes::{node::Node, TreeNode},
    salt::Salt,
    secret::Secret,
    store::Store,
};

pub struct SMT<T: TreeNode + Clone> {
    root: Node,
    store: Store<T>,
    height: Height,
}

pub struct Tree<T: TreeNode + Clone> {
    inner: SMT<T>,
    master_secret: Secret,
    salt_s: Salt,
    salt_b: Salt,
}
