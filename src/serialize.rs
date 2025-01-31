use std::fmt::Debug;

use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use serde_with::Bytes;

pub struct SerdeAs;

impl<T: CanonicalSerialize + Debug> serde_with::SerializeAs<T> for SerdeAs {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];
        println!("source {:?}", source);
        source
            .serialize_uncompressed(&mut bytes)
            .map_err(serde::ser::Error::custom)?;
        if serializer.is_human_readable() {
            Bytes::serialize_as(&bytes, serializer)
        } else {
            Bytes::serialize_as(&bytes, serializer)
        }
    }
}
