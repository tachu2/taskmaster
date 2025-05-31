use std::error::Error;

use crate::commandline::CommandLine;
use crate::config::config::Config;
use crate::exec::commands::add;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No command")]
    NoCommand,
}

pub fn exec(command: CommandLine, config: &Config) {
    add(command.get_args(), config);
}
