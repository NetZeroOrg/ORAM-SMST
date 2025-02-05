use crate::{CurvePoint, ScalarField};
use ark_ec::AffineRepr;
use ark_serialize::CanonicalSerializeHashExt;
use mina_curves::pasta::curves::pallas::{G_GENERATOR_X, G_GENERATOR_Y};
use num_bigint::BigUint;
use std::ops::Mul;

/// Represents pair of base points in  pallas curve that act as keys
pub struct Pedersen {
    /// Base point for the commited value default as the generator
    pub base: CurvePoint,
    /// Base point for the bliding factor defaults as `hash(gen) * Gen`
    pub base_blinding: CurvePoint,
}
impl Default for Pedersen {
    /// The base point
    fn default() -> Self {
        let gen = CurvePoint::new(G_GENERATOR_X, G_GENERATOR_Y);
        let gen_hash = gen.hash::<sha2::Sha256>();
        let gen_hash_scalar = BigUint::from_bytes_le(&gen_hash);
        let base_blinding = gen.mul_bigint(gen_hash_scalar.to_u64_digits());
        Pedersen {
            base: gen,
            base_blinding: base_blinding.into(),
        }
    }
}
impl Pedersen {
    pub fn commit(&self, value: ScalarField, blinding: ScalarField) -> CurvePoint {
        let res = self.base.mul(&value) + self.base_blinding.mul(&blinding);
        res.into()
    }
}
