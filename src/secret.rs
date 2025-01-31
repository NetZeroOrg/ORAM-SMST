use std::str::FromStr;

use crate::{error::ErrorKind, ScalarField};
use num_bigint::BigUint;
use rand::Rng;

/// A 256-bit packet for the pedersen commitment
#[derive(Clone, Debug, Copy)]
pub struct Secret([u8; 32]);

impl Secret {
    pub fn as_bytes_slice(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_bigint(&self) -> BigUint {
        BigUint::from_bytes_le(self.as_bytes_slice())
    }

    pub fn to_field(&self) -> ScalarField {
        let biguint = BigUint::from_bytes_be(&self.0);
        ScalarField::from(biguint)
    }
}

impl From<u32> for Secret {
    fn from(value: u32) -> Self {
        let u32_bytes = value.to_be_bytes();
        let mut secret = vec![0; 28];
        secret.extend_from_slice(&u32_bytes);
        Secret(secret.try_into().unwrap())
    }
}

impl From<[u8; 32]> for Secret {
    fn from(value: [u8; 32]) -> Self {
        Secret(value)
    }
}

impl FromStr for Secret {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 32 {
            Err(ErrorKind::StringTooLong {
                given: s.len(),
                max: 32,
            })
        } else {
            let mut arr = [0u8; 32];
            arr[..s.len()].copy_from_slice(s.as_bytes());
            Ok(Secret(arr))
        }
    }
}

pub fn random_secret() -> Secret {
    return Secret::from(rand::rng().random::<u32>());
}
