use std::collections::HashMap;

use serde_json::map;

use crate::{
    error::{self, Error, Result},
    node_position::{Direction, Height, NodePosition},
    nodes::{node::Node, TreeNode},
    record::Record,
    secret::Secret,
    store::{NodeMap, Store},
};
mod multi;
mod single;

pub struct Pair<T: TreeNode> {
    left: Option<(NodePosition, T)>,
    right: Option<(NodePosition, T)>,
}

impl<T: TreeNode> Pair<T> {
    pub fn new(left: Option<(NodePosition, T)>, right: Option<(NodePosition, T)>) -> Self {
        if left.is_none() && right.is_none() {
            panic!("Both node empty in pair")
        }
        Self { left, right }
    }

    pub fn is_matched(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    pub fn pad_if_not_match(&mut self, blinding_factor: Secret, user_salt: Secret) -> Result<()> {
        match self.left {
            Some((left_pos, _)) => match self.right {
                Some(_) => (),
                None => {
                    let pos = left_pos.get_sibling_pos();
                    self.right = Some((pos, T::new_pad(blinding_factor, pos, user_salt)))
                }
            },
            None => match self.right {
                Some((right_pos, _)) => {
                    let pos = right_pos.get_sibling_pos();
                    self.right = Some((pos, T::new_pad(blinding_factor, pos, user_salt)))
                }
                None => return Err(Error::BothNodesEmpty),
            },
        }
        Ok(())
    }

    pub fn merge(&self) -> Result<(NodePosition, T)> {
        match (&self.left, &self.right) {
            (Some((left_pos, left)), Some((_, right))) => {
                let parent_pos = left_pos.get_parent_node_pos();
                let parent = T::merge(left, right);
                Ok((parent_pos, parent))
            }
            _ => return Err(Error::FoundUnmatchedNodes),
        }
    }
}

pub struct SMTreeBuilder<const N_CURR: usize> {
    records: Vec<Record<N_CURR>>,
    height: Option<u8>,
}

impl<const N_CURR: usize> SMTreeBuilder<N_CURR> {
    pub fn new(records: Vec<Record<N_CURR>>, height: Option<u8>) -> Self {
        Self { records, height }
    }
}

/// builds the whole tree from nodes and returns the root node
/// store depth indicates the number of nodes to store in the hashmap
/// this will be relative to the machine specification
pub fn build_from_nodes<T: TreeNode + Clone>(
    leaf_nodes: Vec<(NodePosition, T)>,
    height: &Height,
    store_depth: u8,
    blinding_factor: Secret,
    user_salt: Secret,
) -> Result<(NodeMap<T>, Node)> {
    let node_map = HashMap::new();
    let max_leafs = height.max_nodes();
    if leaf_nodes.len() > max_leafs as usize {
        return Err(error::Error::TooManyLeafNodesForHeight {
            given: leaf_nodes.len() as u64,
            max: max_leafs,
        });
    }

    let mut nodes = leaf_nodes;
    for y in 0..height.as_u8() {
        let height = Height::new(y);
        let mut pairs = vec![];
        for (node_pos, node) in nodes {
            match node_pos.direction() {
                Direction::Left => {
                    pairs.push(Pair {
                        left: Some((node_pos, node)),
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
                                pair.right = Some((node_pos, node));
                            } else {
                                pairs.push(Pair {
                                    left: None,
                                    right: Some((node_pos, node)),
                                })
                            }
                        }
                        // there was no node in pair
                        None => pairs.push(Pair {
                            left: None,
                            right: Some((node_pos, node)),
                        }),
                    }
                }
            }
            // pad nodes and put nodes = merge
            pairs
                .iter_mut()
                .for_each(|pair| pair.pad_if_not_match(blinding_factor, user_salt).unwrap());

            nodes = pairs.iter().map(|pair| pair.merge().unwrap()).collect();
        }
    }
    let (_, root_node) = nodes.pop().unwrap();
    (node_map, root_node)
}
