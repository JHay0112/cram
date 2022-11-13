/// CRAM
/// Command line Rust based Account Manager
/// 
/// CRAM (sometimes referred to as cram) is a command line account management software.
/// It is intended for personal management of accounts and expenses.

mod utils;
use utils::{ERROR_MARKER, wait_for_input, print_banner};

mod commands;
use commands::{parse_command, CommandResult};

fn main() {

    let mut input;

    print_banner();

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
