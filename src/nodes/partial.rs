use crate::{
    hasher::Hashables,
    node_position::{self, NodePosition},
    pedersen::Pedersen,
    tree_builder::PaddingNodeContent,
    BaseField, CurvePoint,
};
use mina_hasher::{create_legacy, Hasher};

use super::TreeNode;

/// The partial node contains partial information used in the merkle proofs to hide liabilities
pub struct PartialNode {
    commitment: CurvePoint,
    hash: BaseField,
}

impl PartialNode {
    pub fn new(commitment: CurvePoint, hash: BaseField) -> Self {
        PartialNode { commitment, hash }
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

        // H = H(left.com | right.com )
        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::Commitment(left_child.commitment));
        hasher.update(&Hashables::Commitment(right_child.commitment));
        hasher.update(&Hashables::Hash(left_child.hash));
        hasher.update(&Hashables::Hash(right_child.hash));
        Self {
            hash: hasher.digest(),
            commitment,
        }
    }
}
