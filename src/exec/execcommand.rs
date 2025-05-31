use std::error::Error;

use crate::commandline::CommandLine;
use crate::config::config::Config;
use crate::exec::commands::add;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("invalid arguments.")]
    InvalidArgs,
    #[error("invalid arguments length.")]
    InvalidLength,
    #[error("process name not found.")]
    ProcessNameNotFound,
}

pub fn exec(command: CommandLine, config: &Config) {
    add(command.get_args(), config);
}
