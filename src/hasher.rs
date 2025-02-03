use crate::{node_position::NodePosition, secret::Secret, BaseField, CurvePoint};
use ark_serialize::CanonicalSerialize;
use mina_hasher::{Hashable, ROInput};
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    pasta::fp_kimchi,
    poseidon::{ArithmeticSponge as Poseidon, Sponge as _},
};

#[derive(Clone)]
pub enum Hashables {
    Bytes(Vec<u8>),
    Secret(Secret),
    Commitment(CurvePoint),
    Hash(BaseField),
    Position(NodePosition),
    Id(String),
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
            Self::Bytes(bytes) => ROInput::new().append_bytes(bytes),
            Self::Secret(sec) => ROInput::new().append_bytes(sec.as_bytes_slice()),
            Self::Commitment(point) => {
                let mut compressed_bytes = Vec::new();
                point.serialize_compressed(&mut compressed_bytes).unwrap();
                ROInput::new().append_bytes(&compressed_bytes)
            }
            Self::Hash(h) => ROInput::new().append_field(*h),
            Self::Position(node_pos) => {
                let mut bytes = node_pos.0.to_le_bytes().to_vec();
                bytes.push(node_pos.1.as_u8());
                ROInput::new().append_bytes(&bytes)
            }
            Self::Id(s) => ROInput::new().append_bytes(s.as_bytes()),
        }
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        "Bytes hashed".to_string().into()
    }
}

pub fn poseidon_hash(field_elems: &[BaseField]) -> BaseField {
    let mut hash =
        Poseidon::<BaseField, PlonkSpongeConstantsKimchi>::new(fp_kimchi::static_params());
    hash.absorb(field_elems);
    hash.squeeze()
}
