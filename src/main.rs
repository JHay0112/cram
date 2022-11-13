/// CRAM
/// Command line Rust based Account Manager
/// 
/// CRAM (sometimes referred to as cram) is a command line account management software.
/// It is intended for personal management of accounts and expenses.

mod utils;
use utils::{NAME, VERSION, ERROR_MARKER, get_wd, wait_for_input};

mod commands;
use commands::{parse_command, CommandResult};

fn main() {

    let mut input;

    println!("{} v{}", NAME, VERSION);
    println!("Working directory: {}", get_wd());
    println!("Type `help` for information on commands");

    loop {
        input = wait_for_input();
        match parse_command(input) {
            CommandResult::Ok => continue,
            CommandResult::Msg(s) => println!("{}", s),
            CommandResult::Err(s) => println!("{}{}", ERROR_MARKER, s),
            CommandResult::Exit => {
                println!("exiting...");
                break;
            }
        };
    }
}
