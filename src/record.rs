use num_bigint::BigUint;
use rand::Rng;

use crate::{hasher::Hashables, BaseField};
use ark_std::UniformRand;
// contains the record from databse of the CEX
// balances are the liability of the CEX
// hashed email acts as Id
#[derive(Debug)]
pub struct Record<const N_CURR: usize> {
    balances: [u64; N_CURR],
    hashed_email: BaseField,
}

impl<const N_CURR: usize> Record<N_CURR> {
    pub fn total_liability(&self) -> BigUint {
        self.balances
            .iter()
            .map(|&balance| BigUint::from(balance))
            .sum()
    }

    pub fn to_hashable(&self) -> Hashables {
        Hashables::UserId(self.hashed_email)
    }

    pub fn new(balances: &[u64; N_CURR], hashed_email: BaseField) -> Self {
        Self {
            balances: *balances,
            hashed_email,
        }
    }
}

pub fn random_records<const N_CURR: usize>(num: u64) -> Vec<Record<N_CURR>> {
    let mut records = Vec::with_capacity(num as usize);
    let mut rng = ark_std::rand::thread_rng();
    for _ in 0..num {
        let mut balances = [u64::default(); N_CURR];
        rand::rng().fill(&mut balances);
        let rand_id = BaseField::rand(&mut rng);
        records.push(Record::new(&balances, rand_id));
    }
    records
}
