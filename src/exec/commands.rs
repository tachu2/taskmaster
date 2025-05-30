use std::collections::LinkedList;

use crate::exec::execcommand::ExecError;

pub fn add(args: &LinkedList<String>) -> Result<(), ExecError> {
    println!("Executing command with args: {:?}", args);
    Ok(())
}
