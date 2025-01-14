use crate::ScalarField;
use num_bigint::BigUint;
use o1_utils::FieldHelpers;

/// A 256-bit packet for the pedersen commitment
#[derive(Clone, Debug)]
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

impl From<u32> for Secret {
    fn from(value: u32) -> Self {
        let u32_bytes = value.to_be_bytes();
        let mut secret = vec![0; 28];
        secret.extend_from_slice(&u32_bytes);
        Secret(secret.try_into().unwrap())
    }
}
