mod commandline;
mod config;
mod errors;
mod exec;

use commandline::CommandLine;

fn main() {
    match commandline::CommandLine::readline() {
        Ok(line) => {
            println!("Command: {:?}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
