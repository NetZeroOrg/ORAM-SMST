use mina_curves::pasta::Fp;
use num_bigint::BigUint;

use crate::hasher::Hashables;

// contains the record from databse of the CEX
// balances are the liability of the CEX
// hashed email acts as Id
pub struct Record<const N_CURR: usize> {
    balances: [u64; N_CURR],
    hashed_email: Fp,
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

    pub fn new(balances: &[u64; N_CURR], hashed_email: Fp) -> Self {
        Self {
            balances: *balances,
            hashed_email,
        }
    }
}
