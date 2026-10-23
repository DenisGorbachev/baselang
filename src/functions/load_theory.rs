use crate::{Parse, PlainParser, PlainParserParseError, Theory};
use errgonomic::handle;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs::read_to_string;

pub async fn load_theory(path: PathBuf) -> Result<Theory, LoadTheoryError> {
    use LoadTheoryError::*;
    let source = handle!(read_to_string(&path).await, ReadToStringFailed, path);
    Ok(handle!(PlainParser::default().parse(&source), ParseFailed, path, input: source))
}

#[derive(Error, Debug)]
pub enum LoadTheoryError {
    #[error("failed to read source file '{path}'", path = path.display())]
    ReadToStringFailed { source: io::Error, path: PathBuf },
    #[error("failed to compile source file '{path}'", path = path.display())]
    ParseFailed { source: PlainParserParseError, path: PathBuf, input: String },
}
