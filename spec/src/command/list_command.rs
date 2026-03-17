use crate::{ListPackageTypesError, list_package_types};
use clap::{Parser, value_parser};
use errgonomic::handle;
use std::io;
use std::io::{Write, stdout};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub struct ListCommand {
    #[arg(long, value_parser = value_parser!(PathBuf))]
    manifest_path: PathBuf,
    #[arg(long)]
    package: Option<String>,
}

impl ListCommand {
    pub async fn run(self) -> Result<ExitCode, ListCommandRunError> {
        use ListCommandRunError::*;
        let Self {
            manifest_path,
            package,
        } = self;
        let output = handle!(list_package_types(&manifest_path, package.as_deref()), ListPackageTypesFailed, manifest_path, package);
        let mut stdout = stdout().lock();
        handle!(stdout.write_all(output.as_bytes()), WriteOutputFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum ListCommandRunError {
    #[error("failed to list package types for manifest path '{manifest_path}'")]
    ListPackageTypesFailed { source: ListPackageTypesError, manifest_path: PathBuf, package: Option<String> },
    #[error("failed to write output")]
    WriteOutputFailed { source: io::Error },
}
