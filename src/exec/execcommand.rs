use std::error::Error;

use crate::commandline::CommandLine;
use crate::exec::commands::add;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No command")]
    NoCommand,
}

pub fn exec(command: CommandLine) {
    add(command.get_args()).unwrap_or_else(|e| {
        eprintln!("Error executing command: {}", e);
    });
}
