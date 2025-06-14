use ini::Error as IniError;
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

#[derive(Error, Debug)]
pub enum ConfigParseError {
    #[error("config file parse error: {0}")]
    IniError(#[from] ini::Error),
    #[error("config file not found.")]
    FileNotFound,
    #[error("cannot read a file: {0}")]
    PermissionDenied(String),
    #[error("unexpected value: {0}")]
    UnexpectedValue(String),
    #[error("duplicated value: {0}")]
    DuplicatedValue(String),
    #[error("command is required in program section: {0}")]
    MissingCommand(String),
    #[error("error: {0}")]
    Critical(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ProgramBuilderError {
    #[error("programname is required.")]
    MissingProgramName,
    #[error("command is required in program section.")]
    MissingCommand,
}
