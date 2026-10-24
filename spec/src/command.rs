use crate::check;
use aist_core::ContractReportWriteError;
use clap::Parser;
use errgonomic::handle;
use std::io::{self, stdout};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, flatten_help = true)]
pub struct Command {
    #[arg(long, default_value = "..")]
    pub project_root: PathBuf,
    #[cfg(feature = "aist-codex")]
    #[command(flatten)]
    pub review: crate::InvariantReviews,
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let root = handle!(self.project_root.canonicalize(), CanonicalizeFailed, path: self.project_root);
        let report = check(&root);
        #[cfg(feature = "aist-codex")]
        let report = {
            let mut report = report;
            handle!(self.review.run(&root, &mut report).await, RunFailed);
            report
        };
        handle!(report.write(stdout().lock()), WriteFailed);
        Ok(if report.passed() { ExitCode::SUCCESS } else { ExitCode::FAILURE })
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to resolve project root '{path}'", path = path.display())]
    CanonicalizeFailed { source: io::Error, path: PathBuf },
    #[error("failed to print specification report")]
    WriteFailed { source: ContractReportWriteError },
    #[cfg(feature = "aist-codex")]
    #[error("failed to review semantic invariants")]
    RunFailed { source: crate::InvariantReviewsRunError },
}
