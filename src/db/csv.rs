use csv::{ReaderBuilder, WriterBuilder};
use sha2::Digest;
use std::error::Error;
use std::fs::File;

use crate::error::ErrorKind;
use crate::record::Record;
use crate::tree::TreeParams;

use super::DB;
#[derive(Debug)]
pub struct Csv {
    pub file: String,
}

impl Default for Csv {
    fn default() -> Self {
        let file_path = format!("{}/data/data.csv", env!("CARGO_MANIFEST_DIR"));
        Self { file: file_path }
    }
}
impl DB for Csv {
    fn get_records<const N_CURR: usize>(&self) -> Result<Vec<Record<N_CURR>>, Box<dyn Error>> {
        let file = File::open(&self.file)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);
        let mut records = vec![];
        for result in rdr.records() {
            let record = result?;
            let plain_email = record
                .get(0)
                .ok_or(ErrorKind::CsvParserErrorFieldNotFound("email".to_string()))?;
            let hashed_email = hex::encode(sha2::Sha256::digest(plain_email));
            let mut balances = [0u64; N_CURR];
            for (i, balance) in record.iter().skip(1).take(N_CURR).enumerate() {
                balances[i] = balance.parse::<u64>().unwrap();
            }
            records.push(Record::new(&balances, hashed_email));
        }

        Ok(records)
    }

    fn set_record_new_balances<const N_CURR: usize>(
        &mut self,
        email: &str,
        balances: &[String; N_CURR],
    ) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        let mut records: Vec<Vec<String>> = Vec::new();

        // Read the CSV into memory
        for result in rdr.records() {
            let record = result?;
            let mut row: Vec<String> = record.iter().map(String::from).collect();

            // If the email matches, update balances
            if row[0] == email {
                row.extend_from_slice(balances);
            }

            records.push(row);
        }

        // Write the updated data back to the file
        let mut wtr = WriterBuilder::new().from_path(&self.file)?;

        let mut headers = vec![String::from("UserEmail")];
        for i in 0..N_CURR {
            headers.push(format!("Asset_{}", i));
        }

        // Write header row
        wtr.write_record(&headers)?;

        // Write each record
        for record in records {
            wtr.write_record(&record)?;
        }

        wtr.flush()?;
        Ok(())
    }

    // For csv we have hard corded as csv is not recomemded use for production
    fn get_tree_params(&self) -> Result<TreeParams, Box<dyn Error>> {
        let master_secret: [u8; 32] = "Mina Blockchain enables privacy!"
            .as_bytes()
            .try_into()
            .unwrap();
        let salt_s: [u8; 32] = "message_encryption_salt_value_12"
            .as_bytes()
            .try_into()
            .unwrap();
        let salt_b: [u8; 32] = "Bright stars illuminate dark sky"
            .as_bytes()
            .try_into()
            .unwrap();
        Ok(TreeParams {
            master_secret: master_secret.into(),
            salt_b: salt_b.into(),
            salt_s: salt_s.into(),
        })
    }
}

#[test]
fn should_parse() {
    let file_path = format!("{}/data/data.csv", env!("CARGO_MANIFEST_DIR"));
    let db = Csv { file: file_path };
    let recs = db.get_records::<3>().unwrap();
    println!("Records {:?}", recs)
}
