use crate::Outcome;
use clap::Parser;
use cli_util::command_enum;

command_enum!(
    #[derive(Parser, Clone, Debug)]
    pub enum Command {
        Print(PrintCommand),
    }
);

mod print_command;

pub use print_command::*;
