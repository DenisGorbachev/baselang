use crate::{LoadTheoryError, load_theory};
use clap::Parser;
use errgonomic::handle;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
#[command(flatten_help = true)]
pub struct CheckCommand {
    pub path: PathBuf,
}

impl CheckCommand {
    pub async fn run(self) -> Result<ExitCode, CheckCommandRunError> {
        use CheckCommandRunError::*;
        handle!(load_theory(self.path).await, LoadTheoryFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum CheckCommandRunError {
    #[error("failed to check the theory")]
    LoadTheoryFailed { source: LoadTheoryError },
}
