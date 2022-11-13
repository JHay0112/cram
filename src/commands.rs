/// Defines commands and command management for CRAM

use std::env;
use std::path::Path;

use crate::utils::{NAME, get_wd};

/// Output of a command
pub enum CommandResult {
    Ok,
    Msg(String),
    Err(String),
    Exit
}

/// Parses the passed input
pub fn parse_command(input: String) -> CommandResult {
    
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