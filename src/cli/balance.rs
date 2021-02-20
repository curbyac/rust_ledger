extern crate serde_yaml;

use crate::error::Result;
use crate::ledger::LedgerFile;

/// returns balances of all general ledger accounts
pub fn balance(filename: &str) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_balances(deserialized_file);

    Ok(())
}
