use std::collections::HashMap;

use crate::{
    node_position::Direction,
    nodes::{node::Node, TreeNode},
    record::Record,
    store::{NodeMap, Store},
};
mod multi;
mod single;

pub struct MaybeMatched {}

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
    leaf_nodes: Vec<T>,
    height: u8,
    store_depth: u8,
) -> (NodeMap<T>, Node) {
    let node_map = HashMap::new();

    // for y in 0..height {
    //     for x in
    // }

    (node_map, root_node)
}
