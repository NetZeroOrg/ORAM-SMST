use crate::record::Record;
mod multi;
mod single;

pub struct SMTreeBuilder<const N_CURR: usize> {
    records: Vec<Record<N_CURR>>,
    height: Option<u8>,
}

impl<const N_CURR: usize> SMTreeBuilder<N_CURR> {
    pub fn new(records: Vec<Record<N_CURR>>, height: Option<u8>) -> Self {
        Self { records, height }
    }
}
