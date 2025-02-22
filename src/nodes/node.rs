use super::{partial::PartialNode, TreeNode};
use crate::{
    error::ErrorKind,
    hasher::{poseidon_hash, Hashables},
    node_position::NodePosition,
    pedersen::Pedersen,
    record::Record,
    secret::Secret,
    smt::NodeContent,
    tree_builder::PaddingNodeContent,
    BaseField, CurvePoint, ScalarField,
};
use ark_ec::AffineRepr;
use ark_serialize::CanonicalSerialize;
use mina_hasher::{create_legacy, Hasher};
use num_bigint::BigUint;
use serde::{de::Error, Deserialize, Serialize};
use serde_with::serde_as;
/// A Node for the SMT
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    liability: BigUint,
    /// we are using `SerdeAsUnchecked` so we do not need to do uncompression logic in ts
    #[serde_as(as = "o1_utils::serialization::SerdeAsUnchecked")]
    blinding_factor: ScalarField, // scalar

    #[serde_as(as = "o1_utils::serialization::SerdeAsUnchecked")]
    commitment: CurvePoint,

    #[serde_as(as = "o1_utils::serialization::SerdeAsUnchecked")]
    hash: BaseField,
}

impl Node {
    pub fn new(
        liability: BigUint,
        blinding_factor: ScalarField,
        commitment: CurvePoint,
        hash: BaseField,
    ) -> Self {
        Self {
            liability,
            blinding_factor,
            commitment,
            hash,
        }
    }

    pub fn to_partial(&self) -> PartialNode {
        PartialNode::new(self.commitment, self.hash)
    }
}

impl TreeNode for Node {
    /// returns a new leaf given a user's db record
    /// the blinding factor secret is calculate as `KDF(wu,salt_b)` where wu = `KDF(master_secret , id_u)`
    /// hash secret is calculate as `KDF(wu,salt_s)` where wu = `KDF(master_secret , id_u)`
    fn new_leaf<const N_CURR: usize>(
        blinding_factor_secret: Secret,
        record: &Record<N_CURR>,
        user_secret: Secret,
    ) -> Self {
        let blinding_factor = blinding_factor_secret.to_field();
        let total_liability = record.total_liability();

        let commitment =
            Pedersen::default().commit(total_liability.clone().into(), blinding_factor);

        // compute the hash `H("leaf" | user_id | user_salt)`
        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("leaf".as_bytes()));
        hasher.update(&record.to_hashable());
        hasher.update(&Hashables::Secret(user_secret));
        let hash = hasher.digest();
        Self {
            liability: total_liability,
            blinding_factor,
            commitment,
            hash,
        }
    }

    /// create a new pad node
    /// level is the height at which the pad is required
    /// level_offset is the offset from the left to the point we are inserting the node assuming that
    /// max height of the tree is 64 which is more than enough for our use case
    fn new_pad(padding_node_content: PaddingNodeContent, position: NodePosition) -> Self {
        let liability = 0u64;

        let blinding_factor = padding_node_content.bliding_factor().to_field();

        let commitment = Pedersen::default().commit(liability.into(), blinding_factor);

        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("pad".as_bytes()));
        hasher.update(&Hashables::Position(position));
        hasher.update(&Hashables::Secret(padding_node_content.user_secret()));
        let hash = hasher.digest();
        Self {
            liability: liability.into(),
            blinding_factor,
            commitment,
            hash,
        }
    }

    fn merge(left_child: &Self, right_child: &Self) -> Self {
        //TODO: Think of something better to remote this clone
        let liability = left_child.liability.clone() + right_child.liability.clone();
        let blinding_factor: ScalarField = left_child.blinding_factor + right_child.blinding_factor;
        let commitment = (left_child.commitment + right_child.commitment).into();

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
        Self {
            liability,
            blinding_factor,
            commitment,
            hash,
        }
    }
}

impl Into<NodeContent> for Node {
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
#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use crate::{
        node_position::{Height, NodePosition},
        nodes::{node::Node, TreeNode},
        record::Record,
        secret::Secret,
        tree_builder::PaddingNodeContent,
    };

    #[test]
    fn node_e2e_works() {
        let _record = vec![Record::new(&[1], String::from("S"))];
        let record = _record.last().unwrap();
        let blinding_factor = Secret::from(2u32);
        let user_secret = Secret::from(1u32);
        let leaf = Node::new_leaf(blinding_factor.clone(), &record, user_secret.clone());
        let pad_content = PaddingNodeContent::new(blinding_factor, user_secret);
        let pad = Node::new_pad(pad_content, NodePosition::new(1, Height::new(1)));
        let merged = Node::merge(&leaf, &pad);
        assert_eq!(merged.liability, BigUint::from(1u32));
    }
}
