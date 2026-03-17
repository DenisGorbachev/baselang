use Subcommand::*;
use errgonomic::map_err;
use std::process::ExitCode;
use thiserror::Error;

use crate::command::{ListCommand, ListCommandRunError};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, propagate_version = true)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommand {
    List(ListCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            subcommand,
        } = self;
        match subcommand {
            List(command) => map_err!(command.run().await, ListCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run list command")]
    ListCommandRunFailed { source: ListCommandRunError },
}

mod list_command;
pub use list_command::*;
