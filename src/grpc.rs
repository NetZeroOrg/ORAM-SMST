use crate::db::csv::Csv;
use crate::db::{DBType, DB};
use crate::node_position::NodePosition;
use crate::nodes::partial::PartialNode;
use crate::proofs::MerkleWitness;
use crate::smt::smt_backend_server::SmtBackend;
use crate::smt::{
    NodeContent, Proof, RequestProof, Response as SetRecordResponse, SetRecordRequest,
};
use crate::tree::{self, new_padding_node_content, TreeBuilder};
use sha2::Digest;
use tonic::{Request, Response, Status};

use std::sync::{Arc, Mutex};

const USER_NOT_FOUND: &str = "USER NOT FOUND";
const SERVER_ERROR: &str = "SERVER ERROR";

#[derive(Debug)]
pub struct Server<const N_CURR: usize> {
    db: Arc<Mutex<DBType>>,
    tree_builder: Arc<Mutex<TreeBuilder<PartialNode, N_CURR>>>,
}

impl<const N_CURR: usize> Default for Server<N_CURR> {
    fn default() -> Self {
        let csv_db = Csv::default();
        let tree_params = csv_db.get_tree_params().unwrap();
        let records = csv_db.get_records::<N_CURR>().unwrap();
        let tree_builder = TreeBuilder::from_records(records, tree_params);
        Self {
            db: Arc::new(Mutex::new(DBType::Csv(csv_db))),
            tree_builder: Arc::new(Mutex::new(tree_builder)),
        }
    }
}

#[tonic::async_trait]
impl<const N_CURR: usize> SmtBackend for Server<N_CURR> {
    async fn generate_proof(
        &self,
        request: Request<RequestProof>,
    ) -> Result<Response<Proof>, Status> {
        let request = request.into_inner();
        let mut tree_builder = self
            .tree_builder
            .lock()
            .map_err(|_| Status::aborted(SERVER_ERROR))?;
        let db = self.db.lock().map_err(|_| Status::aborted(SERVER_ERROR))?;
        let tree_params = db.get_tree_params().unwrap();
        let (tree, record_map) = tree_builder
            .build_single_threaded(None)
            .map_err(|err| Status::aborted(err.to_string()))?;
        let hashed_email = hex::encode(sha2::Sha256::digest(request.user_email.clone()));
        if !record_map.contains_key(&hashed_email) {
            return Err(Status::invalid_argument(USER_NOT_FOUND));
        }
        let padding_fn = |pos: &NodePosition| {
            new_padding_node_content(
                &tree_params.master_secret.as_bytes_slice(),
                &tree_params.salt_s.as_bytes(),
                &tree_params.salt_b.as_bytes(),
                pos,
            )
        };

        let witness: MerkleWitness<PartialNode, N_CURR> =
            MerkleWitness::generate_witness(hashed_email, &tree, &record_map, &padding_fn)
                .map_err(|err| Status::aborted(err.to_string()))?;
        let mut node_contents: Vec<NodeContent> = vec![];
        for node in witness.path.0 {
            node_contents.push(node.into());
        }
        let root = if request.fetch_root() {
            Some(witness.root.into())
        } else {
            None
        };
        let user_node = if request.fetch_user_node() {
            Some(witness.user_leaf.into())
        } else {
            None
        };
        let proof = Proof {
            path: node_contents,
            lefts: witness.lefts,
            root,
            user_node,
            for_user: request.user_email,
            master_salt: tree_params.master_secret.as_vec(),
        };
        Ok(Response::new(proof))
    }

    async fn set_user_data(
        &self,
        request: Request<SetRecordRequest>,
    ) -> Result<Response<SetRecordResponse>, Status> {
        let request = request.into_inner();
        let _balances: Vec<String> = request
            .balances
            .iter()
            .map(|balance| balance.to_string())
            .collect();
        let mut balances: [String; N_CURR] = [(); N_CURR].map(|_| String::default());
        for i in 0..N_CURR {
            balances[i] = _balances[i].clone();
        }
        let user_email = request.user_name;
        let mut db = self
            .db
            .lock()
            .map_err(|err| Status::aborted(err.to_string()))?;
        let msg = match db.set_record_new_balances(&user_email, &balances) {
            Ok(_) => "Saved Succesfully".to_string(),
            Err(err) => err.to_string(),
        };
        return Ok(Response::new(SetRecordResponse { msg }));
    }
}
