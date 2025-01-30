use std::{collections::HashMap, fmt::Debug};

use crate::{
    error::{ErrorKind, Result},
    node_position::{Direction, Height, NodePosition},
    nodes::TreeNode,
    record::Record,
    secret::Secret,
    tree::SMT,
};
mod multi;
mod single;

#[derive(Debug)]
pub struct Pair<T: TreeNode + Debug> {
    left: Option<(NodePosition, T)>,
    right: Option<(NodePosition, T)>,
}

impl<T: TreeNode + Debug> Pair<T> {
    pub fn new(left: Option<(NodePosition, T)>, right: Option<(NodePosition, T)>) -> Self {
        if left.is_none() && right.is_none() {
            panic!("Both node empty in pair")
        }
        Self { left, right }
    }

    pub fn is_matched(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    pub fn pad_if_not_match<F: Fn(&NodePosition) -> PaddingNodeContent>(
        &mut self,
        padding_node_content: &F,
    ) -> Result<()> {
        match self.left {
            Some((left_pos, _)) => match self.right {
                Some(_) => (),
                None => {
                    let pos = left_pos.get_sibling_pos();
                    let content = padding_node_content(&pos);
                    self.right = Some((pos, T::new_pad(content, pos)))
                }
            },
            None => match self.right {
                Some((right_pos, _)) => {
                    let pos = right_pos.get_sibling_pos();
                    let content = padding_node_content(&pos);
                    self.left = Some((pos, T::new_pad(content, pos)))
                }
                None => return Err(ErrorKind::BothNodesEmpty),
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
            _ => return Err(ErrorKind::FoundUnmatchedNodes),
        }
    }
}

/// Padding node content
/// 1 - blinding factor
/// 2 - user secret
#[derive(Clone)]
pub struct PaddingNodeContent(Secret, Secret);

impl PaddingNodeContent {
    pub fn bliding_factor(&self) -> Secret {
        self.0.clone()
    }
    pub fn user_secret(&self) -> Secret {
        self.1.clone()
    }

    pub fn new(blinding_factor: Secret, user_secret: Secret) -> Self {
        PaddingNodeContent(blinding_factor, user_secret)
    }
}

/// builds the whole tree from nodes and returns the root node
/// store depth indicates the number of nodes to store in the hashmap
/// this will be relative to the machine specification
pub fn build_tree<T: TreeNode + Clone + Debug, F: Fn(&NodePosition) -> PaddingNodeContent>(
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
