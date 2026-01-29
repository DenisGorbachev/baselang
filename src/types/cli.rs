use crate::{Command, CommandRunError};
use clap::Parser;
use errgonomic::map_err;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    // #[arg(short, long, value_parser = value_parser!(PathBuf))]
    // root: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> Result<ExitCode, CliRunError> {
        use CliRunError::*;
        let Self {
            command,
        } = self;
        map_err!(command.run().await, CommandRunFailed)
    }
}

#[derive(Error, Debug)]
pub enum CliRunError {
    #[error("failed to run command")]
    CommandRunFailed { source: CommandRunError },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
