use crate::{BOOLS, same_exp, same_scope};
use crate::{Parse, PlainParser, PlainParserParseScopeError, PlainParserParseScopeLineError};
use crate::{PlainParserExpressionError, PlainParserParseError, ScopeNormalizeError};
use errgonomic::handle;
use thiserror::Error;

#[test]
fn must_preserve_parser_contract() -> Result<(), MustPreserveParserContractError> {
    use MustPreserveParserContractError::*;
    let mut parser = PlainParser::default();
    let source = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False\nNot False => True");
    let provisional = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let theory = handle!(parser.parse(&source), ParseFailed);
    assert_eq!(theory.scope().bindings().len(), provisional.bindings().len(), "parse retains all declarations");
    assert_eq!(
        theory
            .scope()
            .rewrites()
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        2,
        "parse retains all rewrite rules"
    );
    let actual = handle!(parser.expression("Not True", theory.scope()), ExpressionFailed);
    let normalized = handle!(theory.scope().normalize(&actual), NormalizeFailed);
    let expected = handle!(parser.expression("False", theory.scope()), ExpressionFailed);
    assert!(same_exp(&normalized, &expected), "expression resolves supplied scope");
    assert!(parser.expression("Not Bool", theory.scope()).is_err(), "expression checks applications in supplied scope");
    let incomplete = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False");
    assert!(parser.parse_scope(&incomplete).is_ok(), "parse_scope permits incomplete coverage");
    assert!(parser.parse(&incomplete).is_err(), "parse validates before success");
    [
        "Bool : Type\nTrue : Missing".to_owned(),
        format!("{BOOLS}\nNot : Bool -> Bool\nNot True => Missing"),
    ]
    .into_iter()
    .for_each(|source| {
        let failed = parser.parse_scope(&source);
        let expected_line = source.lines().count();
        assert!(matches!(failed, Err(PlainParserParseScopeError::ParseScopeLineFailed { input, source: error }) if input == source && matches!(*error, PlainParserParseScopeLineError::ParseLineFailed { line_number, .. } if line_number == expected_line)), "parse error retains source and failing line");
    });
    let before = theory.scope().clone();
    handle!(parser.expression("Not True", theory.scope()), ExpressionFailed);
    assert!(same_scope(theory.scope(), &before), "expression lookup does not mutate scope");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveParserContractError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call parse")]
    ParseFailed { source: PlainParserParseError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call normalize")]
    NormalizeFailed { source: ScopeNormalizeError },
}
