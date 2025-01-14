use crate::{secret::Secret, BaseField, CurvePoint};
use ark_serialize::CanonicalSerialize;
use mina_hasher::{Hashable, ROInput};

#[derive(Clone)]
pub enum Hashables {
    Bytes(Vec<u8>),
    Secret(Secret),
    UserId(BaseField),
    Commitment(CurvePoint),
    Hash(BaseField),
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
            Self::Commitment(point) => {
                let mut compressed_bytes = Vec::new();
                point.serialize_compressed(&mut compressed_bytes).unwrap();
                ROInput::new().append_bytes(&compressed_bytes)
            }
            Self::Hash(h) => ROInput::new().append_field(*h),
        }
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        format!("Bytes hashed").into()
    }
}
