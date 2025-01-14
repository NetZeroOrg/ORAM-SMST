use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use mina_curves::pasta::curves::pallas::{G_GENERATOR_X, G_GENERATOR_Y};
use std::ops::Mul;

use crate::{CurvePoint, ScalarField};

/// Represents pair of base points in  pallas curve that act as keys
pub struct Pedersen {
    /// Base point for the commited value default as the generator
    pub base: CurvePoint,
    /// Base point for the bliding factor defaults as a random point
    pub base_blinding: CurvePoint,
}

impl Pedersen {
    pub fn default() -> Pedersen {
        let mut rng = StdRng::seed_from_u64(0u64);
        Pedersen {
            base: CurvePoint::new(G_GENERATOR_X, G_GENERATOR_Y),
            base_blinding: CurvePoint::rand(&mut rng),
        }
    }

    pub fn commit(&self, value: ScalarField, blinding: ScalarField) -> CurvePoint {
        let res = self.base.mul(&value) + self.base_blinding.mul(&blinding);
        res.into()
    }
}
