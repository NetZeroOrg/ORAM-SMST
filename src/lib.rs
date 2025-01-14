use mina_curves::pasta::{Fq, Pallas};
use num_bigint::BigUint;
use o1_utils::FieldHelpers;
pub mod hasher;
pub mod kdf;
pub mod nodes;
pub mod pedersen;
pub mod record;

/// A 256-bit packet for the pedersen commitment
#[derive(Clone)]
pub struct Secret([u8; 32]);

impl Secret {
    pub fn as_bytes_slice(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_bigint(&self) -> BigUint {
        BigUint::from_bytes_le(self.as_bytes_slice())
    }

    pub fn to_field(&self) -> ScalarField {
        ScalarField::from_bytes(&self.0).unwrap()
    }
}

pub(crate) type ScalarField = Fq;
pub(crate) type Hash = BigUint;
pub(crate) type CurvePoint = Pallas;
