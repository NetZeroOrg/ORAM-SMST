use crate::{
    node_position::NodePosition, record::Record, secret::Secret, tree_builder::PaddingNodeContent,
};

pub mod node;
pub mod partial;

// Common trait for both partial and full node

pub trait TreeNode {
    fn new_leaf<const N_CURR: usize>(
        blinding_factor: Secret,
        record: &Record<N_CURR>,
        user_salt: Secret,
    ) -> Self;

    fn new_pad(padding_node_content: PaddingNodeContent, position: NodePosition) -> Self;

    fn merge(left_child: &Self, right_child: &Self) -> Self;
}
