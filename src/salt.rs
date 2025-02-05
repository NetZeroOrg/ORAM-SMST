use ark_std::rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use std::str::FromStr;

use crate::error::ErrorKind;

#[derive(Clone, Debug)]
pub struct Salt([u8; 32]);

impl Salt {
    pub fn as_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn generate_random() -> Self {
        let mut rng = thread_rng();
        let random_str = Alphanumeric.sample_string(&mut rng, 32);
        Self::from_str(&random_str).unwrap()
    }

    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl FromStr for Salt {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 32 {
            Err(ErrorKind::StringTooLong {
                given: s.len(),
                max: 32usize,
            })
        } else {
            let mut arr = [0u8; 32];
            arr[..s.len()].copy_from_slice(s.as_bytes());
            Ok(Salt(arr))
        }
    }
}

impl From<[u8; 32]> for Salt {
    fn from(value: [u8; 32]) -> Self {
        Salt(value)
    }
}
