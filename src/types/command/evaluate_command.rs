use crate::{LoadTheoryError, PlainParser, PlainParserExpressionError, PlainRenderer, ScopeNormalizeError, load_theory};
use clap::Parser;
use errgonomic::handle;
use std::io::{self, Write, stdout};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
#[command(flatten_help = true)]
pub struct EvaluateCommand {
    pub path: PathBuf,
    pub expression: String,
}

impl EvaluateCommand {
    pub async fn run(self) -> Result<ExitCode, EvaluateCommandRunError> {
        use EvaluateCommandRunError::*;
        let theory = handle!(load_theory(self.path).await, LoadTheoryFailed);
        let exp = handle!(PlainParser::default().expression(&self.expression, theory.scope()), ExpressionFailed, expression: self.expression);
        let result = handle!(theory.scope().normalize(&exp), NormalizeFailed, expression: self.expression);
        let text = PlainRenderer::default().render_exp_inner(&result, false, false);
        handle!(writeln!(stdout().lock(), "{text}"), WritelnFailed, text);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum EvaluateCommandRunError {
    #[error("failed to load the theory")]
    LoadTheoryFailed { source: LoadTheoryError },
    #[error("failed to parse expression '{expression}'")]
    ExpressionFailed { source: PlainParserExpressionError, expression: String },
    #[error("failed to evaluate expression '{expression}'")]
    NormalizeFailed { source: ScopeNormalizeError, expression: String },
    #[error("failed to write the result")]
    WritelnFailed { source: io::Error, text: String },
}
