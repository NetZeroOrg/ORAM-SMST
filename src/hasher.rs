use mina_hasher::{Fp, Hashable, ROInput};

use crate::Secret;

#[derive(Clone)]
pub struct Bytes(Vec<u8>);

#[derive(Clone)]
pub enum Hashables {
    Bytes(Vec<u8>),
    Secret(Secret),
    UserId(Fp),
}

impl Hashables {
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.to_vec())
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        Self::Bytes(bytes)
    }
}

impl Hashable for Hashables {
    type D = ();
    fn to_roinput(&self) -> ROInput {
        match &self {
            Self::Bytes(bytes) => ROInput::new().append_bytes(&bytes),
            Self::Secret(sec) => ROInput::new().append_bytes(sec.as_bytes_slice()),
            Self::UserId(s) => ROInput::new().append_field(*s),
        }
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        format!("Bytes hashed").into()
    }
}
