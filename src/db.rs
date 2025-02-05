pub mod csv;

pub enum DB {
    /// The path to the csv file to take data from
    Csv(String),
    RocksDb,
    Postgres,
}
