use std::{fmt::Debug, fs::OpenOptions, io::Write};

use serde::Serialize;

use crate::{
    error::{ErrorKind, Result},
    node_position::NodePosition,
    nodes::TreeNode,
    siblings::Siblings,
    tree::{RecordMap, SMT},
    tree_builder::PaddingNodeContent,
};

///Creates a merkle witness for the given leaf node in JSON form and writes to the given path or defaults to proofs/user_id.json
#[derive(Serialize)]
pub struct MerkleWitness<T: TreeNode + Clone + Debug + Serialize, const N_CURR: usize> {
    pub path: Siblings<T>,
    pub lefts: Vec<bool>,
    pub user_leaf: T,
    pub root: T,
    #[serde(skip_serializing)]
    pub _user_id: String,
}

impl<T: TreeNode + Clone + Debug + Serialize, const N_CURR: usize> MerkleWitness<T, N_CURR> {
    pub fn generate_witness<F: Fn(&NodePosition) -> PaddingNodeContent>(
        user_id: String,
        tree: &SMT<T>,
        record_map: &RecordMap,
        padding_fn: &F,
    ) -> Result<MerkleWitness<T, N_CURR>> {
        let node_pos = record_map
            .get(&user_id)
            .ok_or(ErrorKind::UserNotFound(user_id.clone()))?;
        let user_leaf = tree.store.get_node(node_pos).unwrap();
        let (siblings, lefts) =
            Siblings::generate_path_single_threaded(tree, *node_pos, padding_fn)?;
        Ok(MerkleWitness {
            path: siblings,
            user_leaf,
            lefts,
            root: tree.root.clone(),
            _user_id: user_id,
        })
    }
    /// writes a the path in json format
    pub fn save(&self, path: Option<&str>) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.unwrap_or(&format!("proofs/{}.json", self._user_id)))
            .unwrap();
        let proof_json = serde_json::ser::to_string(&self).unwrap();
        file.write_all(proof_json.as_bytes()).unwrap();
        Ok(())
    }
}
