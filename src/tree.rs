use std::{collections::HashMap, marker::PhantomData};

use crate::{
    error::Result,
    kdf,
    node_position::{Height, NodePosition},
    nodes::{node::Node, TreeNode},
    record::Record,
    salt::Salt,
    secret::Secret,
    store::Store,
    tree_builder::PaddingNodeContent,
};
use log::info;
use rand::{rngs::SmallRng, Rng, SeedableRng};
pub struct SMT<T: TreeNode + Clone> {
    pub root: T,
    pub store: Store<T>,
    pub height: Height,
}
/// The tree paramets such as `master_salt` , `salt_s` , `salt_b`
pub struct TreeParams {
    master_salt: Secret,
    pub salt_s: Salt,
    pub salt_b: Salt,
}

pub struct TreeBuilder<T: TreeNode + Clone, const N_CURR: usize> {
    records: Vec<Record<N_CURR>>,
    /// The position of leaf node in the vector to the hashmap
    x_cord_generator: XCordGenerator,
    tree_params: TreeParams,
    height: Height,
    _marker: PhantomData<T>,
}

impl<T: TreeNode + Clone, const N_CURR: usize> TreeBuilder<T, N_CURR> {
    pub fn new(records: Vec<Record<N_CURR>>, height: Height, tree_params: TreeParams) -> Self {
        Self {
            records,
            x_cord_generator: XCordGenerator::new(height),
            tree_params,
            height,
            _marker: PhantomData,
        }
    }

    /// build the tree single threaded and with given records
    pub fn build_single_threaded(&mut self, store_depth: Option<u8>) -> Result<SMT<T>> {
        use crate::tree_builder::build_tree;
        info!(
            "ORAM-SMT Configuration
            +----------------+------------------------+
            | Height         | {height:16} |
            | Entities       | {num_entities:16} |
            | Master Secret  | <REDACTED>            |
            | Salt B         | 0x{salt_b:14} |
            | Salt S         | 0x{salt_s:14} |
            +----------------+------------------------+",
            height = self.height.as_u32(),
            num_entities = self.records.len(),
            salt_b = &self.tree_params.salt_b.as_hex(),
            salt_s = &self.tree_params.salt_s.as_hex()
        );
        let mut leaf_nodes = Vec::with_capacity(self.records.len());

        for i in 0..self.records.len() {
            let new_x_cord = self.x_cord_generator.gen_x_cord()?;
            let node_pos = NodePosition::new(new_x_cord, self.height);
            // `w` in dapol +
            let master_secret = kdf::kdf(
                None,
                Some(&new_x_cord.to_le_bytes()),
                self.tree_params.master_salt.as_bytes_slice(),
            );
            // `b` in dapol +
            let blinding_factor = kdf::kdf(
                Some(&self.tree_params.salt_b.as_bytes()),
                None,
                &master_secret,
            );
            // `s` in dapol +
            let user_salt = kdf::kdf(
                Some(&self.tree_params.salt_s.as_bytes()),
                None,
                &master_secret,
            );
            let node = T::new_leaf(blinding_factor.into(), &self.records[i], user_salt.into());
            leaf_nodes.push((node_pos, node));
        }
        let padding_fn = |pos: &NodePosition| {
            new_padding_node_content(
                &self.tree_params.master_salt.as_bytes_slice(),
                &self.tree_params.salt_s.as_bytes(),
                &self.tree_params.salt_b.as_bytes(),
                pos,
            )
        };

        build_tree(
            leaf_nodes,
            &self.height,
            store_depth.unwrap_or_default(),
            &padding_fn,
        )
    }
}

pub fn new_padding_node_content(
    master_secret: &[u8; 32],
    salt_s: &[u8; 32],
    salt_b: &[u8; 32],
    position: &NodePosition,
) -> PaddingNodeContent {
    let cord_bytes = position.to_bytes();
    let pad_secret = kdf::kdf(None, Some(&cord_bytes), master_secret);
    let blinding_factor = kdf::kdf(Some(salt_b), None, &pad_secret);
    let user_secret = kdf::kdf(Some(salt_s), None, &pad_secret);
    PaddingNodeContent::new(blinding_factor.into(), user_secret.into())
}

/// An X cordinate generator
pub struct XCordGenerator {
    rng: SmallRng,
    x_cords: HashMap<u64, u64>,
    max_x_cord: u64,
    i: u64,
}
impl XCordGenerator {
    pub fn new(height: Height) -> Self {
        Self {
            rng: SmallRng::from_os_rng(),
            x_cords: HashMap::new(),
            max_x_cord: height.max_nodes(),
            i: 0,
        }
    }

    pub fn gen_x_cord(&mut self) -> Result<u64> {
        if self.i >= self.max_x_cord {
            return Err(crate::error::ErrorKind::MaxNumNodesReached(self.max_x_cord));
        }
        let random_x = self.rng.random_range(self.i..self.max_x_cord);
        let x = match self.x_cords.get(&random_x) {
            Some(mut existing_x) => {
                while self.x_cords.contains_key(existing_x) {
                    existing_x = self.x_cords.get(existing_x).unwrap();
                }
                *existing_x
            }
            None => random_x,
        };
        self.x_cords.insert(x, self.i);
        self.i += 1;
        Ok(x)
    }
}
