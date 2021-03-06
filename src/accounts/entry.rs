// entry.rs
// Your entries per account.

use crate::args::arg::Arg;
use crate::accounts::account::Account;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum EntryType {
    Withdrawal,
    Deposit,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub label: String,
    pub amount: f64,
    pub entry_type: EntryType,
    pub date: String, // TODO: rresearch if the format is available.
    pub note: String
}

pub fn entry_exists(ac: &Account, args: &Vec<Arg>) -> bool {
    ac.entries
        .iter()
        .find(|&entry| entry.label == args[1].value)
        .is_some()
}
