/// CRAM
/// Command line Rust based Account Manager
/// 
/// CRAM (sometimes referred to as cram) is a command line account management software.
/// It is intended for personal management of accounts and expenses.

use std::io;
use std::io::Write;
use std::process;
use std::path::Path;
use std::env;
use std::fs;

const INPUT_MARKER: &str = ">>> ";
const ERROR_MARKER: &str = "ERROR: ";
const VERSION: &str = env!("CARGO_PKG_VERSION");

enum CommandResult {
    Ok,
    Message(String),
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
    let command = parts.next().unwrap();
    let mut args = parts;

    return match command {
        "exit" => CommandResult::Exit,
        "cd" => {
            let new_dir = args.next().unwrap();
            let path = Path::new(new_dir);
            return match env::set_current_dir(&path) {
                Ok(()) => CommandResult::Ok,
                Err(e) => CommandResult::Err(e.to_string())
            };
        },
        "ls" => {
            let paths = fs::read_dir("./").unwrap();
            for path in paths {
                println!("{}", path.unwrap().path().display())
            }
            return CommandResult::Ok;
        }
        _ => CommandResult::Err(format!("\"{}\" is not a recognised command!", command))
    };
}

fn main() {

    let mut input;

    println!("CRAM v{}", VERSION);
    println!("Account Management Tool");
    println!();

    loop {
        input = wait_for_input();
        match parse_command(input) {
            CommandResult::Ok => continue,
            CommandResult::Message(s) => println!("{}", s),
            CommandResult::Err(s) => println!("{}{}", ERROR_MARKER, s),
            CommandResult::Exit => {
                println!("exiting...");
                break;
            }
        };
    }
}
