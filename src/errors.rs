use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandLineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("empty command.")]
    EmptyCommand,
}

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("invalid arguments.")]
    InvalidArgs,
    #[error("invalid arguments length.")]
    InvalidLength,
    #[error("process name not found.")]
    ProcessNameNotFound,
}
