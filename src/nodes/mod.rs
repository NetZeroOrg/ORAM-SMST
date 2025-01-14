use mina_hasher::{create_legacy, Hasher};
use num_bigint::BigUint;

use crate::{
    hasher::Hashables, pedersen::Pedersen, record::Record, CurvePoint, Hash, ScalarField, Secret,
};

pub struct LeafNode<const N_CURR: usize> {
    liability: BigUint,
    blinding_factor: ScalarField, // scalar
    commitment: CurvePoint,
    hash: Hash,
}

impl<const N_CURR: usize> LeafNode<N_CURR> {
    pub fn new(
        liability: BigUint,
        blinding_factor: ScalarField,
        commitment: CurvePoint,
        hash: Hash,
    ) -> Self {
        Self {
            liability,
            blinding_factor,
            commitment,
            hash,
        }
    }

    /// returns a new leaf given a user's db record
    /// the blinding factor secret is calculate as `KDF(wu,salt_b)` where wu = `KDF(master_secret , id_u)`
    /// hash secret is calculate as `KDF(wu,salt_s)` where wu = `KDF(master_secret , id_u)`
    pub fn new_leaf(
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
            hash: hash.into(),
        }
    }

    /// create a new pad node
    /// level is the height at which the pad is required
    /// level_offset is the offset from the left to the point we are inserting the node assuming that
    /// max height of the tree is 64 which is more than enough for our use case
    pub fn new_pad(
        blinding_factor_secret: Secret,
        height: u8,
        level_offset: u64,
        user_salt: Secret,
    ) -> Self {
        let liability = 0u64;

        let blinding_factor = blinding_factor_secret.to_field();

        let commitment = Pedersen::default().commit(liability.into(), blinding_factor);

        //TODO: remove the unncessary vector allocation
        let mut cord_bytes = vec![height];
        cord_bytes.extend_from_slice(&level_offset.to_le_bytes());

        let mut hasher = create_legacy::<Hashables>(());
        hasher.update(&Hashables::from_slice("pad".as_bytes()));
        hasher.update(&Hashables::Bytes(cord_bytes));
        hasher.update(&Hashables::Secret(user_salt));
        let hash: Hash = hasher.digest().into();
        Self {
            liability: liability.into(),
            blinding_factor,
            commitment,
            hash,
        }
    }
}
