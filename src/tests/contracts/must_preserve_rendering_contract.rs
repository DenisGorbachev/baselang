use crate::BOOLS;
use crate::{PlainParser, PlainRenderer, Render, Rewrite};
use crate::{PlainParserExpressionError, PlainParserParseScopeError};
use errgonomic::handle;
use thiserror::Error;

#[test]
fn must_preserve_rendering_contract() -> Result<(), MustPreserveRenderingContractError> {
    use MustPreserveRenderingContractError::*;
    let parser = PlainParser::default();
    let scope = handle!(parser.parse_scope(BOOLS), ParseScopeFailed);
    let renderer = PlainRenderer::new("Type");
    scope.bindings().values().for_each(|sym| {
        let name = sym.nym().short.en.singular.to_string();
        let typ = renderer.render_typ_inner(sym.typ());
        assert_eq!(renderer.render_sym(sym), Some(format!("{name} : {typ}")), "render_sym equation");
    });
    let truth = handle!(parser.expression("True", &scope), ExpressionFailed);
    let rule = Rewrite::new(Vec::new(), truth.clone());
    assert_eq!(renderer.render_rewrite(truth.head(), &rule), Some("True => True".to_owned()), "zero-argument rewrite equation");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveRenderingContractError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
}
