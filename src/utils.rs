/// General purpose functions or static variables
/// For cohesion's and coupling's sake this should not be overused!
 
use std::env;
use std::io;
use std::io::Write;

use crate::commands::wd::get_wd;

static INPUT_MARKER: &str = ">>> ";
pub static ERROR_MARKER: &str = "ERROR: ";
static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static NAME: &str = "CRAM";

// I intend for these to be moved to a data management module later
//pub static ACCOUNT_PATH: &str = "accounts.csv";
//pub static TRANSACTIONS_PATH: &str = "transactions.csv";
//pub static ACCOUNT_HOLDERS_PATH: &str = "account_holders.csv";

/// Prints the application banner
pub fn print_banner() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    println!("{} v{}", NAME, VERSION);
    println!("Working directory: {}", get_wd());
    println!("Type `help` for information on commands");
}

/// Gets and waits for a user input
pub fn wait_for_input() -> String {
    print!("{}", INPUT_MARKER);
    let mut input = String::new();
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin().read_line(&mut input).unwrap();
    return input;
}