use crate::node_position::NodePosition;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Too many records provided for the given height (given: {given:?}, max: {max:?})")]
    TooManyLeafNodesForHeight { given: u64, max: u64 },

    #[error("Cannot insert node in store")]
    CannotInsertInStore,

    #[error("Both nodes empty in a pair at position")]
    BothNodesEmpty,

    #[error("Found unmatched nodes when merging")]
    FoundUnmatchedNodes,

    #[error("Cannot parse salt due to long string (given: {given:?}, max: {max:?})")]
    StringTooLong { given: usize, max: usize },

    #[error("Maximum number of nodes is {0} which is reached")]
    MaxNumNodesReached(u64),

    #[error("Cannot find the node at {0} for which path is required")]
    CannotFindLeafNode(NodePosition),

    #[error("The record for the user {0} was not used in constructing the tree")]
    UserNotFound(String),

    #[error("Cannot find file {0} to save merkle proofs for {1}")]
    FileNotFound(String, String),

    #[error("Expected the field {0} in the csv")]
    CsvParserErrorFieldNotFound(String),

    #[error("File {0} not found")]
    CsvParserErrorFiledNotFound(String),
}

pub(crate) type Result<T> = std::result::Result<T, ErrorKind>;
