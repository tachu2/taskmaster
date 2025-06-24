use crate::errors::CommandLineError::EmptyCommand;
use crate::{commandline::CommandLine, config::runtimecontext::RuntimeContext};

pub fn command_loop(rc: RuntimeContext) -> Result<(), String> {
    loop {
        match CommandLine::readline() {
            Ok(line) => {
                println!("Command: {:?}", line);
            }
            Err(e) => match e {
                EmptyCommand => {
                    continue;
                }
                _ => {
                    eprintln!("Error reading line: {}", e);
                    break;
                }
            },
        }
    }
    Ok(())
}
