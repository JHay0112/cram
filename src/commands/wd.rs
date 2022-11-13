/// Commands for management of the working directory

use std::env;
use std::path::Path;

use crate::CommandResult;

/// Gets a string representation of the current working directory
pub fn get_wd() -> String {
    return env::current_dir().unwrap().to_string_lossy().to_string();
}

/// Prints the current working directory
pub fn print_wd() -> CommandResult {
    println!("{}", get_wd());
    return CommandResult::Ok;
}

pub fn set_wd(new_wd: &str) -> CommandResult {
    let path = Path::new(new_wd);
    return match env::set_current_dir(&path) {
        Ok(()) => CommandResult::Msg(format!("New working directory is {}", get_wd())),
        Err(_) => CommandResult::Err(format!("Directory \"{}\" does not exist!", path.to_str().unwrap()))
    };
}