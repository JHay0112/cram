/// Defines commands and command management for CRAM

use crate::utils::NAME;

pub mod wd;

/// Output of a command
pub enum CommandResult {
    Ok,
    Msg(String),
    Err(String),
    Exit
}

/// Prints help for all the commands
fn print_help() -> CommandResult {
    println!("Universal Commands");
    println!(" help                  Produces this help page");
    println!(" pwd                   Prints the current working directory");
    println!(" swd <wd>              Sets a new working directory");
    println!(" exit                  Exits {}", NAME);
    println!("Further commands are detailed when accessible");

    return CommandResult::Ok;
}

/// Parses the passed input
pub fn parse_command(input: String) -> CommandResult {
    
    let mut parts = input.trim().split_whitespace();
    let command = match parts.next() {
        None => return CommandResult::Ok,
        Some(cmd) => cmd
    };
    let mut args = parts;

    return match command {
        "pwd" => wd::print_wd(),
        "swd" => match args.next() {
                None => CommandResult::Err(format!("Directory is required!")),
                Some(wd) => wd::set_wd(wd)
        },
        "exit" => CommandResult::Exit,
        "help" => print_help(),
        _ => CommandResult::Err(format!("\"{}\" is not a recognised command!", command))
    };
}