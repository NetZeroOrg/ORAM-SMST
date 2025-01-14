use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use mina_curves::pasta::curves::pallas::{G_GENERATOR_X, G_GENERATOR_Y};
use std::ops::Mul;

use crate::{CurvePoint, ScalarField};

/// Represents pair of base points in  pallas curve that act as keys
pub struct Pedersen {
    /// Base point for the commited value default as the generator
    pub B: CurvePoint,
    /// Base point for the bliding factor defaults as a random point
    pub B_blinding: CurvePoint,
}

impl Pedersen {
    pub fn default() -> Pedersen {
        let mut rng = StdRng::seed_from_u64(0u64);
        Pedersen {
            B: CurvePoint::new(G_GENERATOR_X, G_GENERATOR_Y),
            B_blinding: CurvePoint::rand(&mut rng),
        }
    }

    pub fn commit(&self, value: ScalarField, blinding: ScalarField) -> CurvePoint {
        let res = self.B.mul(&value) + self.B_blinding.mul(&blinding);
        res.into()
    }
}
