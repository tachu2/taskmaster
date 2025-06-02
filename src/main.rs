mod commandline;
mod config;
mod errors;

use commandline::CommandLine;
use config::logger::{get_logger, LogLevel, Logger};

fn main() {
    let logger = get_logger();
    logger.log(LogLevel::INFO, "Application started");
    match commandline::CommandLine::readline() {
        Ok(line) => {
            println!("Command: {:?}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
