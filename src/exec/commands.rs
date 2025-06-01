use std::collections::LinkedList;

use crate::config::config::Config;
use crate::config::program_section::ProgramSection;
use crate::errors::ExecError;

pub fn add(args: &LinkedList<String>, config: &Config) -> Result<(), ExecError> {
    if args.len() == 0 {
        return Err(ExecError::InvalidArgs);
    }
    for programname in args.iter() {
        config
            .find_program(&programname)
            .ok_or(ExecError::ProcessNameNotFound)?;
    }
    Ok(())
}
