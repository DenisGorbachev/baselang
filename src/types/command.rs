use Command::*;
use clap::Parser;
use errgonomic::map_err;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    RenderPrelude(RenderPreludeCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        match self {
            RenderPrelude(command) => map_err!(command.run().await, RenderPreludeCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run render-prelude command")]
    RenderPreludeCommandRunFailed { source: RenderPreludeCommandRunError },
}

mod render_prelude_command;

pub use render_prelude_command::*;
