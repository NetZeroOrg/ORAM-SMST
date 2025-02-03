use std::{collections::HashMap, fmt::Debug};

use serde::Serialize;

use crate::{
    error::{ErrorKind, Result},
    node_position::{Direction, Height, NodePosition},
    nodes::TreeNode,
    tree::SMT,
};

use super::{PaddingNodeContent, Pair};

/// builds the whole tree from nodes and returns the root node
/// store depth indicates the number of nodes to store in the hashmap
/// this will be relative to the machine specification
pub fn single_threaded_tree_builder<
    T: TreeNode + Clone + Debug + Serialize,
    F: Fn(&NodePosition) -> PaddingNodeContent,
>(
    leaf_nodes: Vec<(NodePosition, T)>,
    height: &Height,
    store_depth: u8,
    padding_node_content: &F,
) -> Result<SMT<T>> {
    let mut node_map = HashMap::new();
    let max_leafs = height.max_nodes();
    if leaf_nodes.len() > max_leafs as usize {
        return Err(ErrorKind::TooManyLeafNodesForHeight {
            given: leaf_nodes.len() as u64,
            max: max_leafs,
        });
    }

    let mut nodes = leaf_nodes;
    for y in 0..height.as_u8() {
        let mut pairs = vec![];
        for (node_pos, node) in nodes.iter() {
            if y <= store_depth {
                node_map.insert(*node_pos, node.clone());
            }
            match node_pos.direction() {
                Direction::Left => {
                    pairs.push(Pair {
                        left: Some((*node_pos, node.clone())),
                        right: None,
                    });
                }
                Direction::Right => {
                    let last_pair = pairs.last_mut();
                    match last_pair {
                        Some(pair) => {
                            let is_sibling = if let Some((left_pos, _)) = pair.left {
                                pair.right.is_none() && left_pos.0 == node_pos.0 - 1
                            } else {
                                false
                            };
                            if is_sibling {
                                pair.right = Some((*node_pos, node.clone()));
                            } else {
                                pairs.push(Pair {
                                    left: None,
                                    right: Some((*node_pos, node.clone())),
                                })
                            }
                        }
                        // there was no node in pair
                        None => pairs.push(Pair {
                            left: None,
                            right: Some((*node_pos, node.clone())),
                        }),
                    }
                }
            }
        }
        nodes = pairs
            .iter_mut()
            .map(|pair| {
                pair.pad_if_not_match(padding_node_content).unwrap();
                pair.merge().unwrap()
            })
            .collect();
    }

    Ok(SMT {
        root: nodes.pop().unwrap().1,
        store: crate::store::Store { map: node_map },
        height: *height,
    })
}
