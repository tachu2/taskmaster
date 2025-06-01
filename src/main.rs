mod commandline;
mod config;
mod errors;

use commandline::CommandLine;
use config::logger::{LogLevel, Logger};

fn main() {
    Logger::change_level(LogLevel::DEBUG);
    match commandline::CommandLine::readline() {
        Ok(line) => {
            println!("Command: {:?}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
