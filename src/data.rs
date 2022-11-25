/// Manages loading and modifying data for CRAM

use csv;
use std::path::Path;
use std::fs::File;
use crate::utils::get_wd;

/// Database file extension
static EXT: &str = ".csv";

/// Types of data to be stored
enum Data {
    Accounts = "accounts",
    Transactions = "transactions",
    AccountHolders = "account_holders"
}

/// Gets the path of a data file
fn get_path(d: Data) -> Path {
    return Path::new(get_wd() + d + EXT);
}

/// Gets a data file object
fn get_file(d: Data) -> File {
    return File::open(&get_path(d));
}

/// Checks if the working directory has been setup
/// i.e. all the files that store data exist
pub fn ready() -> bool {
    for d in Data {
        if !path(d).exist() {
            return false;
        }
    }
    return true;
}

/// Creates all necessary files
pub fn prepare() {
    for d in Data {
        let p = path(d);
        if !p.exist() {
            File::create(&p);
        }
    }
}