// helpers.rs
// Method that output helps for the user.

use std::vec::Vec;
use crate::args::arg;
use crate::accounts::record;

// Function pointer for a block.
pub fn display_help(_rd: &mut record::Record, _args: &mut Vec<arg::Arg>) {
    help();
}

pub fn help() {
    println!("here is the usage: accountscli
	     [--new | -n <account> [<label> <amount> [<date> <note>]]]
	     [--list | -l [<account>]]
	     [--rename | -re <account> <new-name>]
             [--remove | -r [<account> [<label>]]]
             [--balance | -b [<account> [<new-balance>]]]
	     [--currency | -c <currency> [<account>]]

new          creates a new account or entry.
list         list all accounts or entries in an account.
rename       renames an account.
remove       removes an account or entry.
balance      set or show balance of an account.
currency     set or show currency of an account.
");
}
