use serde::{Deserialize, Serialize};
use serde_with::NoneAsEmptyString;
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash, Debug)]
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

    pub fn as_u8(&self) -> u8 {
        self.0
    }
    pub fn get_parent_height(&self) -> Self {
        Height::new(self.0 - 1)
    }
}

pub enum Direction {
    Left,
    Right,
}

/// A node position in the tree is (x, y) pair
/// where x = 0 is the leftmost position and y = 0 is the root
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct NodePosition(pub u64, pub Height);

impl std::fmt::Display for NodePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{}", self.0, self.1.as_u8())
    }
}

impl NodePosition {
    pub fn new(x: u64, y: Height) -> Self {
        NodePosition(x, y)
    }

    pub fn direction(&self) -> Direction {
        if self.0 % 2 == 0 {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    pub fn get_parent_node_pos(&self) -> Self {
        NodePosition::new(self.0 >> 1, self.1.get_parent_height())
    }

    /// right node position for left node
    /// left node position for right position
    pub fn get_sibling_pos(&self) -> Self {
        match self.direction() {
            Direction::Left => NodePosition::new(self.0 + 1, self.1),
            Direction::Right => NodePosition::new(self.0 - 1, self.1),
        }
    }
}
