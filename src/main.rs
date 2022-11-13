/// CRAM
/// Command line Rust based Account Manager
/// 
/// CRAM (sometimes referred to as cram) is a command line account management software.
/// It is intended for personal management of accounts and expenses.

use std::io;
use std::io::Write;

use std::env;

const INPUT_MARKER: &str = ">>> ";
const ERROR_MARKER: &str = "ERROR: ";
const VERSION: &str = env!("CARGO_PKG_VERSION");

enum CommandResult {
    Ok(String),
    Err(String),
    Exit
}

/// Gets and waits for a user input
fn wait_for_input() -> String {
    print!("{}", INPUT_MARKER);
    let mut input = String::new();
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

/// Parses the passed input
fn parse_command(input: String) -> CommandResult {
    
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap().to_string();
    let args = parts;

    if command == "exit" {
        return CommandResult::Exit;
    }

    return CommandResult::Err(format!("{}\"{}\" is not a recognised command!", ERROR_MARKER, command));
}

fn main() {

    let mut input;

    println!("CRAM v{}", VERSION);
    println!("Account Management Tool");
    println!();

    loop {
        input = wait_for_input();
        match parse_command(input) {
            CommandResult::Ok(s) => println!("{}", s),
            CommandResult::Err(s) => println!("{}{}", ERROR_MARKER, s),
            CommandResult::Exit => {
                println!("exiting...");
                break;
            }
        };
    }
}
