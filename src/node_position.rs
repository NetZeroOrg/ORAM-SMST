/// A node position in the tree is (x, y) pair
/// where x = 0 is the leftmost position and y = 0 is the root
#[derive(Clone, Copy)]
pub struct NodePosition(pub u64, pub u8);

impl NodePosition {
    pub fn new(x: u64, y: u8) -> Self {
        NodePosition(x, y)
    }
}
