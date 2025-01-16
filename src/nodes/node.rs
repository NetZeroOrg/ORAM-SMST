use super::{partial::PartialNode, TreeNode};
use crate::{
    hasher::Hashables, node_position::NodePosition, pedersen::Pedersen, record::Record,
    secret::Secret, BaseField, CurvePoint, ScalarField,
};
use mina_hasher::{create_legacy, Hasher};
use num_bigint::BigUint;

/// A Node for the SMT
#[derive(Debug)]
pub struct Node {
    liability: BigUint,
    blinding_factor: ScalarField, // scalar
    commitment: CurvePoint,
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
        record: Record<N_CURR>,
        user_salt: Secret,
    ) -> Self {
        let blinding_factor = blinding_factor_secret.to_field();
        let total_liability = record.total_liability();

        let commitment =
            Pedersen::default().commit(total_liability.clone().into(), blinding_factor);

        // compute the hash `H("leaf" | user_id | user_salt)`
        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("leaf".as_bytes()));
        hasher.update(&record.to_hashable());
        hasher.update(&Hashables::Secret(user_salt));
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
    fn new_pad(blinding_factor_secret: Secret, position: NodePosition, user_salt: Secret) -> Self {
        let liability = 0u64;

        let blinding_factor = blinding_factor_secret.to_field();

        let commitment = Pedersen::default().commit(liability.into(), blinding_factor);

        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("pad".as_bytes()));
        hasher.update(&Hashables::Position(position));
        hasher.update(&Hashables::Secret(user_salt));
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

        // hash = `H(com_l | com_r | hash_l | hash_r)`
        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::Commitment(left_child.commitment));
        hasher.update(&Hashables::Commitment(right_child.commitment));
        hasher.update(&Hashables::Hash(left_child.hash));
        hasher.update(&Hashables::Hash(right_child.hash));

        let hash = hasher.digest();
        Self {
            liability,
            blinding_factor,
            commitment,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use crate::{
        node_position::NodePosition,
        nodes::{node::Node, TreeNode},
        record::Record,
        secret::Secret,
        BaseField,
    };

    #[test]
    fn node_e2e_works() {
        const N_CURR: usize = 1;
        let balances = &[1u64];
        let record = Record::<N_CURR>::new(balances, BaseField::from(1));
        let bliding_factor = Secret::from(2u32);
        let user_salt = Secret::from(1u32);
        let leaf = Node::new_leaf(bliding_factor.clone(), record, user_salt.clone());
        let pad = Node::new_pad(bliding_factor, NodePosition::new(1, 1), user_salt);
        let merged = Node::merge(&leaf, &pad);
        assert_eq!(merged.liability, BigUint::from(1u32));
    }
}
