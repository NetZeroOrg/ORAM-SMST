#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too many records provided for the given height (given: {given:?}, max: {max:?})")]
    TooManyLeafNodesForHeight { given: u64, max: u64 },

    #[error("Cannot insert node in store")]
    CannotInsertInStore,

    #[error("Both nodes empty in a pair at position")]
    BothNodesEmpty,

    #[error("Found unmatched nodes when merging")]
    FoundUnmatchedNodes,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
