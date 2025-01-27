use ark_std::rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use std::str::FromStr;

use crate::error::ErrorKind;

pub struct Salt([u8; 32]);

impl Salt {
    pub fn as_bytes(&self) -> [u8; 32] {
        return self.0;
    }

    pub fn generate_random() -> Self {
        let mut rng = thread_rng();
        let random_str = Alphanumeric.sample_string(&mut rng, 32);
        Self::from_str(&random_str).unwrap()
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
