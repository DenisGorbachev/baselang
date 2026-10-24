use crate::Report;
use errgonomic::{ErrorDisplayer, handle};
use std::io::{self, Write};
use thiserror::Error;

pub fn write_report(report: &impl Report, mut writer: impl Write) -> Result<(), WriteReportError> {
    use WriteReportError::*;
    let result = report
        .outcomes()
        .try_for_each(|outcome| match outcome.result {
            Ok(()) => writeln!(writer, "PASS {}", outcome.name),
            Err(error) => writeln!(writer, "FAIL {}\n{}", outcome.name, ErrorDisplayer(error)),
        });
    handle!(result, WriteFailed);
    Ok(())
}

#[derive(Error, Debug)]
pub enum WriteReportError {
    #[error("failed to write specification results")]
    WriteFailed { source: io::Error },
}
