use std::error::Error;

use csv::Csv;

use crate::{record::Record, tree::TreeParams};

pub mod csv;

#[derive(Debug)]
pub enum DBType {
    /// The path to the csv file to take data from
    Csv(Csv),
    RocksDb,
    Postgres,
}
impl DBType {
    pub fn get_tree_params(&self) -> Result<TreeParams, Box<dyn Error>> {
        match &self {
            Self::Csv(db) => db.get_tree_params(),
            _ => todo!("Implement Db"),
        }
    }

    pub fn set_record_new_balances<const N_CURR: usize>(
        &mut self,
        email: &str,
        balances: &[String; N_CURR],
    ) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Csv(db) => db.set_record_new_balances(email, balances),
            _ => todo!("Implement db"),
        }
    }
}

//TODO: Add More things later
pub trait DB {
    fn get_tree_params(&self) -> Result<TreeParams, Box<dyn Error>>;
    fn get_records<const N_CURR: usize>(&self) -> Result<Vec<Record<N_CURR>>, Box<dyn Error>>;
    fn set_record_new_balances<const N_CURR: usize>(
        &mut self,
        email: &str,
        balances: &[String; N_CURR],
    ) -> Result<(), Box<dyn Error>>;
}
