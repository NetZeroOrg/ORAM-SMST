use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::{error::Result, node_position::NodePosition, nodes::TreeNode};

pub(crate) type NodeMap<T> = HashMap<NodePosition, T>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store<T: TreeNode + Clone + Debug> {
    pub map: NodeMap<T>,
}

impl<T> Store<T>
where
    T: TreeNode + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get_node(&self, pos: &NodePosition) -> Option<T> {
        self.map.get(pos).map(|n| (*n).clone())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn insert(&mut self, node: T, position: NodePosition) -> Result<()> {
        self.map
            .insert(position, node)
            .ok_or::<crate::error::ErrorKind>(crate::error::ErrorKind::CannotInsertInStore)?;
        Ok(())
    }
}
