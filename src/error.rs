#[derive(thiserror::Error, Debug)]
pub enum TreeBuildError {
    #[error("Too many records provided for the given height (given: {given:?}, max: {max:?})")]
    TooManyRecords { given: u8, max: u8 },
}
