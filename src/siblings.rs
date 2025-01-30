use std::fmt::Debug;

use crate::{
    error::{ErrorKind, Result},
    node_position::{Direction, Height, NodePosition},
    nodes::TreeNode,
    tree::{new_padding_node_content, SMT},
    tree_builder::{build_tree, PaddingNodeContent},
};

#[derive(Debug)]
pub struct Siblings<T: TreeNode + Clone + Debug>(pub Vec<T>);

impl<T: TreeNode + Clone + Debug> Siblings<T> {
    pub fn generate_path_single_threaded<F: Fn(&NodePosition) -> PaddingNodeContent>(
        tree: &SMT<T>,
        pos: NodePosition,
        padding_node_content: &F,
    ) -> Result<Siblings<T>> {
        // check even if the tree node consists
        let _ = tree
            .store
            .get_node(&pos)
            .ok_or(ErrorKind::CannotFindLeafNode(pos))?;
        let mut siblings = Vec::with_capacity(tree.height.as_u8() as usize);
        let mut current_pos = pos;
        for y in 0..tree.height.as_u8() {
            let siblings_pos = current_pos.get_sibling_pos();
            let sibling = match tree.store.get_node(&siblings_pos) {
                Some(node) => node,
                None => {
                    if y == 0 {
                        return Err(ErrorKind::CannotFindLeafNode(siblings_pos));
                    }
                    // the min leaf node cordinate for this cordinate as the root for subtree
                    let x_min_cord = 1u64 << current_pos.1.as_u64() * current_pos.0;
                    // the max leaf node cordinate for this cordinate as the root for subtree
                    let x_max_cord = 1u64 << current_pos.1.as_u64() * (current_pos.0 + 1) - 1;
                    let mut leaf_nodes = vec![];
                    for x in x_min_cord..=x_max_cord {
                        let leaf_pos = NodePosition::new(x, Height::new(0));
                        match tree.store.get_node(&leaf_pos) {
                            Some(leaf_node) => leaf_nodes.push((leaf_pos, leaf_node)),
                            None => (),
                        }
                    }
                    if leaf_nodes.is_empty() {
                        let padding_node_content = padding_node_content(&siblings_pos);
                        T::new_pad(padding_node_content, siblings_pos)
                    } else {
                        let sub_tree =
                            build_tree(leaf_nodes, &Height::new(y), 0, padding_node_content)?;
                        sub_tree.root
                    }
                }
            };
            siblings.push(sibling);
            current_pos = current_pos.get_parent_node_pos();
        }

        Ok(Siblings(siblings))
    }

    /// generates the root from the given path
    pub fn get_root_from_path(&self, leaf_node: T, leaf_pos: NodePosition) -> T {
        let mut root = leaf_node;
        let mut current_pos = leaf_pos;
        for node in self.0.iter() {
            match current_pos.direction() {
                Direction::Left => {
                    root = T::merge(&root, node);
                    current_pos = current_pos.get_parent_node_pos();
                }
                Direction::Right => {
                    root = T::merge(node, &root);
                    current_pos = current_pos.get_parent_node_pos();
                }
            }
        }
        root
    }
}
