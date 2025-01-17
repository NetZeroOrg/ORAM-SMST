use serde_json::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too many records provided for the given height (given: {given:?}, max: {max:?})")]
    TooManyRecords { given: u8, max: u8 },

    #[error("Cannot insert node in store")]
    CannotInsertInStore,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
