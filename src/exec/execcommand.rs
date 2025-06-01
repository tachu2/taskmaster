use crate::commandline::CommandLine;
use crate::config::config::Config;
use crate::exec::commands::add;

pub fn exec(command: CommandLine, config: &Config) {
    add(command.get_args(), config);
}
