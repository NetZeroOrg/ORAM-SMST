use serde::{Deserialize, Serialize};

use crate::nodes::TreeNode;

// represetns if the node is left or right child for the lowers levels
pub enum Direction<T: TreeNode> {
    Left(T),
    Right(T),
}

impl<T: TreeNode> Direction<T> {
    pub fn new(node: T, position: NodePosition) -> Self {
        if position.0 % 2 == 0 {
            Direction::Left(node)
        } else {
            Direction::Right(node)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Height(u8);

impl Height {
    pub fn new(y: u8) -> Self {
        Self(y)
    }

    /// returns maximum number of nodes at this level
    pub fn max_nodes(&self) -> u64 {
        1u64 << self.0
    }

    /// validated leaf node given
    pub fn enough_leaf_nodes_given(&self, len: u64) -> bool {
        (1u64 << self.0) > len
    }
}

/// A node position in the tree is (x, y) pair
/// where x = 0 is the leftmost position and y = 0 is the root
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct NodePosition(pub u64, Height);

impl NodePosition {
    pub fn new(x: u64, y: Height) -> Self {
        NodePosition(x, y)
    }
}
