use std::collections::HashMap;

use crate::{node_position::NodePosition, nodes::node::Node};

pub struct Store{
    map: HashMap<NodePosition, Node>
}
