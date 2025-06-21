mod commandline;
mod config;
mod errors;

use commandline::CommandLine;
use config::{
    adapter::Adapter,
    logger::{LogLevel, Logger},
    runtimecontext::RuntimeContext,
};
use std::env;

fn usage(s: &str) {
    eprintln!("Usage: {} <config_file_path>", s);
    std::process::exit(1);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 1 && args.len() != 2 {
        usage(&args[0]);
    }
    let file_path = if args.len() == 2 {
        Some(&args[1])
    } else {
        None
    };
    let mut runtime_context = RuntimeContext::default();
    if let Err(e) = Adapter::parse_config(&mut runtime_context, file_path) {
        eprintln!("Error parsing config: {}", e);
    }
    runtime_context.logger.info("starting taskmasterd");
    match commandline::CommandLine::readline() {
        Ok(line) => {
            println!("Command: {:?}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
