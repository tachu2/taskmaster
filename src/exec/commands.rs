use std::collections::LinkedList;

use crate::config::config::Config;
use crate::config::program_section::ProgramSection;
use crate::exec::execcommand::ExecError;

pub fn add(args: &LinkedList<String>, config: &Config) -> Result<(), ExecError> {
    println!("Executing command with args: {:?}", args);
    Ok(())
}
