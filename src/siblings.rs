use std::fmt::Debug;

use serde::Serialize;

use crate::{
    error::{ErrorKind, Result},
    node_position::{Direction, Height, NodePosition},
    nodes::TreeNode,
    tree::SMT,
    tree_builder::{build_tree, PaddingNodeContent},
};

#[derive(Debug, Serialize)]
pub struct Siblings<T: TreeNode + Clone + Debug + Serialize>(pub Vec<T>);

impl<T: TreeNode + Clone + Debug + Serialize> Siblings<T> {
    pub fn generate_path_single_threaded<F: Fn(&NodePosition) -> PaddingNodeContent>(
        tree: &SMT<T>,
        pos: NodePosition,
        padding_node_content: &F,
    ) -> Result<(Siblings<T>, Vec<bool>)> {
        // check even if the tree node consists
        let _ = tree
            .store
            .get_node(&pos)
            .ok_or(ErrorKind::CannotFindLeafNode(pos))?;
        let mut siblings = Vec::with_capacity(tree.height.as_u8() as usize);
        let mut current_pos = pos;
        let mut lefts = vec![];
        for y in 0..tree.height.as_u8() {
            let siblings_pos = current_pos.get_sibling_pos();
            lefts.push(siblings_pos.is_left());
            let sibling = match tree.store.get_node(&siblings_pos) {
                Some(node) => node,
                None => {
                    // if at zero level we do not find node it was a padding node
                    if y == 0 {
                        let padding_node_content = padding_node_content(&siblings_pos);
                        T::new_pad(padding_node_content, siblings_pos)
                    } else {
                        // the min leaf node cordinate for this cordinate as the root for subtree
                        let x_min_cord = (1u64 << siblings_pos.1.as_u64()) * siblings_pos.0;
                        // the max leaf node cordinate for this cordinate as the root for subtree
                        let x_max_cord =
                            (1u64 << siblings_pos.1.as_u64()) * (siblings_pos.0 + 1) - 1;
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
                }
            };
            siblings.push(sibling);
            current_pos = current_pos.get_parent_node_pos();
        }

        Ok((Siblings(siblings), lefts))
    }

    /// generates the root from the given path
    pub fn get_root_from_path(&self, leaf_node: T, lefts: &Vec<bool>) -> T {
        let mut root = leaf_node;
        for (node, left) in self.0.iter().zip(lefts) {
            match left {
                true => {
                    root = T::merge(node, &root);
                }
                false => {
                    root = T::merge(&root, node);
                }
            }
        }
        root.clone()
    }
}
