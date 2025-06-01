use std::collections::LinkedList;
use std::io::{self, Write};

use crate::errors::CommandLineError;

#[derive(Debug, Clone)]
pub struct CommandLine {
    command: String,
    args: LinkedList<String>,
}

impl CommandLine {
    fn new(command: String, args: LinkedList<String>) -> Self {
        CommandLine { command, args }
    }

    fn new_from_list(line: LinkedList<String>) -> Option<Self> {
        let mut iter = line.into_iter();
        if let Some(command) = iter.next() {
            let args: LinkedList<String> = iter.collect();
            Some(CommandLine::new(command, args))
        } else {
            None
        }
    }

    pub fn readline() -> Result<CommandLine, CommandLineError> {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        eprint!("> ");
        io::stderr().flush()?;
        stdin.read_line(&mut buf)?;
        let list: std::collections::LinkedList<String> =
            buf.split_whitespace().map(|s| s.to_string()).collect();
        CommandLine::new_from_list(list).ok_or_else(|| CommandLineError::EmptyCommand)
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_args(&self) -> &LinkedList<String> {
        &self.args
    }
}
