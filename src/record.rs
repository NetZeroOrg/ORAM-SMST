use num_bigint::BigUint;
use rand::{distr::Alphanumeric, Rng};

use crate::hasher::Hashables;
// contains the record from databse of the CEX
// balances are the liability of the CEX
// hashed email acts as Id
#[derive(Debug, Clone)]
pub struct Record<const N_CURR: usize> {
    balances: [u64; N_CURR],
    pub hashed_email: String,
}

impl<const N_CURR: usize> Record<N_CURR> {
    pub fn total_liability(&self) -> BigUint {
        self.balances
            .iter()
            .map(|&balance| BigUint::from(balance))
            .sum()
    }

    pub fn to_hashable(&self) -> Hashables {
        Hashables::Id(self.hashed_email.clone())
    }

    pub fn new(balances: &[u64; N_CURR], hashed_email: String) -> Self {
        Self {
            balances: *balances,
            hashed_email,
        }
    }
}

pub fn random_records<const N_CURR: usize>(num: u64) -> Vec<Record<N_CURR>> {
    let mut records = Vec::with_capacity(num as usize);
    let mut rng = rand::rng();
    for _ in 0..num {
        let mut balances = [u64::default(); N_CURR];
        rng.fill(&mut balances);
        // assume 256 bit hash 1 hex -> 4 bits 256 / 4 =
        let rand_id = (0..32)
            .map(|_| format!("{:?}", rng.random_range(0..16)))
            .collect();
        records.push(Record::new(&balances, rand_id));
    }
    records
}
