use Command::*;
use clap::Parser;
use errgonomic::map_err;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    RenderPrelude(RenderPreludeCommand),
    Check(CheckCommand),
    Evaluate(EvaluateCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        match self {
            RenderPrelude(command) => map_err!(command.run().await, RenderPreludeCommandRunFailed),
            Check(command) => map_err!(command.run().await, CheckCommandRunFailed),
            Evaluate(command) => map_err!(command.run().await, EvaluateCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run check command")]
    CheckCommandRunFailed { source: CheckCommandRunError },
    #[error("failed to run evaluate command")]
    EvaluateCommandRunFailed { source: EvaluateCommandRunError },
    #[error("failed to run render-prelude command")]
    RenderPreludeCommandRunFailed { source: RenderPreludeCommandRunError },
}

mod render_prelude_command;

pub use render_prelude_command::*;

mod check_command;
pub use check_command::*;

mod evaluate_command;
pub use evaluate_command::*;
