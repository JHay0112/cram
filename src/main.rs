/// CRAM
/// Command line Rust based Account Manager
/// 
/// CRAM (sometimes referred to as cram) is a command line account management software.
/// It is intended for personal management of accounts and expenses.

use std::io;
use std::io::Write;
use std::path::Path;
use std::env;

enum CommandResult {
    Ok,
    Msg(String),
    Err(String),
    Exit
}

/// Gets a string representation of the current working directory
fn get_wd() -> String {
    return env::current_dir().unwrap().to_string_lossy().to_string();
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

    match command {
        "pwd" => return CommandResult::Msg(get_wd()),
        "swd" => {
            let new_wd: &str = match args.next() {
                None => return CommandResult::Err("Directory is required!".to_string()),
                Some(wd) => wd
            };
            let path = Path::new(new_wd);
            return match env::set_current_dir(&path) {
                Ok(()) => CommandResult::Msg(format!("New working directory is {}", get_wd())),
                Err(_) => CommandResult::Err(format!("Directory \"{}\" does not exist!", path.to_str().unwrap()))
            };
        },
        "exit" => return CommandResult::Exit,
        "help" => {

            println!("Universal Commands");
            println!(" help                  Produces this help page");
            println!(" pwd                   Prints the current working directory");
            println!(" swd <wd>              Sets a new working directory");
            println!(" exit                  Exits {}", NAME);
            println!("Further commands are detailed when accessible");
        
            return CommandResult::Ok;
        },
        _ => return CommandResult::Err(format!("\"{}\" is not a recognised command!", command))
    };
}

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
