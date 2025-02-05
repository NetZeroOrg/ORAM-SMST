use std::fmt::Debug;

use super::TreeNode;
use crate::{
    hasher::{poseidon_hash, Hashables},
    node_position::NodePosition,
    pedersen::Pedersen,
    smt::NodeContent,
    tree_builder::PaddingNodeContent,
    BaseField, CurvePoint,
};
use ark_ec::AffineRepr;
use ark_serialize::CanonicalSerialize;
use mina_hasher::{create_legacy, Hasher};
use o1_utils::FieldHelpers;
use serde::Serialize;
use serde_with::serde_as;

/// The partial node contains partial information used in the merkle proofs to hide liabilities
#[serde_as]
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct PartialNode {
    #[serde_as(as = "crate::serialize::SerdeAs")]
    pub commitment: CurvePoint,
    #[serde_as(as = "crate::serialize::SerdeAs")]
    hash: BaseField,
}

impl PartialNode {
    pub fn new(commitment: CurvePoint, hash: BaseField) -> Self {
        PartialNode { commitment, hash }
    }
}

impl Debug for PartialNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "commitment {:?} hash {}",
            self.commitment,
            self.hash.to_biguint().to_str_radix(10)
        )
    }
}

impl TreeNode for PartialNode {
    fn new_leaf<const N_CURR: usize>(
        blinding_factor: crate::secret::Secret,
        record: &crate::record::Record<N_CURR>,
        user_salt: crate::secret::Secret,
    ) -> Self {
        let total_lia = record.total_liability();
        let commitment = Pedersen::default().commit(total_lia.into(), blinding_factor.to_field());

        // hash = `H("leaf" | user_id | user_salt)`
        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("leaf".as_bytes()));
        hasher.update(&record.to_hashable());
        hasher.update(&Hashables::Secret(user_salt));
        PartialNode {
            hash: hasher.digest(),
            commitment,
        }
    }

    fn new_pad(padding: PaddingNodeContent, position: NodePosition) -> Self {
        let commitment = Pedersen::default().commit(0.into(), padding.bliding_factor().to_field());

        let mut hasher = create_legacy::<Hashables>(());

        hasher.update(&Hashables::from_slice("pad".as_bytes()));
        hasher.update(&Hashables::Position(position));
        hasher.update(&Hashables::Secret(padding.user_secret()));

        Self {
            hash: hasher.digest(),
            commitment,
        }
    }

    fn merge(left_child: &Self, right_child: &Self) -> Self {
        // commitment left.com + right.com
        let commitment: CurvePoint = (left_child.commitment + right_child.commitment).into();

        // H = H(left.com | right.com | left.hash | right.hash )
        let hash_inputs = [
            *left_child.commitment.x().unwrap(),
            *left_child.commitment.y().unwrap(),
            *right_child.commitment.x().unwrap(),
            *right_child.commitment.y().unwrap(),
            left_child.hash,
            right_child.hash,
        ];
        let hash = poseidon_hash(&hash_inputs);
        Self { hash, commitment }
    }
}

impl Into<NodeContent> for PartialNode {
    fn into(self) -> NodeContent {
        let mut bytes_commitemnt = vec![];
        self.commitment
            .serialize_uncompressed(&mut bytes_commitemnt)
            .unwrap();

        let mut bytes_hash = vec![];
        self.hash.serialize_uncompressed(&mut bytes_hash).unwrap();
        NodeContent {
            commitment: bytes_commitemnt,
            hash: bytes_hash,
        }
    }
}
