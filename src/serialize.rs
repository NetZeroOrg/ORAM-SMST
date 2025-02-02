use ark_serialize::CanonicalSerialize;
use serde_with::Bytes;
use std::fmt::Debug;

pub struct SerdeAs;
pub struct SerdeAffineCurvePoint;

impl<T: CanonicalSerialize + Debug> serde_with::SerializeAs<T> for SerdeAs {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];
        source
            .serialize_uncompressed(&mut bytes)
            .map_err(serde::ser::Error::custom)?;
        Bytes::serialize_as(&bytes, serializer)
    }
}
