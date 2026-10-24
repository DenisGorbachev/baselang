use crate::{BOOLS, NATS, same_exp, same_scope};
use crate::{PlainParser, Rewrite, ScopeInsertRewriteError, ScopeInsertSymError, Sym, SymRc, Typ};
use crate::{PlainParserExpressionError, PlainParserParseScopeError};
use errgonomic::handle;
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_rollback_contract() -> Result<(), MustPreserveRollbackContractError> {
    use MustPreserveRollbackContractError::*;
    let parser = PlainParser::default();
    let source = format!("{NATS}\nLoop : Nat -> Nat\nLoop x => Loop x");
    let mut scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let looping = handle!(parser.expression("Loop Zero", &scope), ExpressionFailed);
    let rejected: SymRc = Sym::new_rc("Rejected", Typ::one(looping));
    let before = scope.clone();
    let result = scope.insert_sym(rejected.clone());
    assert!(matches!(result, Err(ScopeInsertSymError::BuildConstructorsFailed { sym, .. }) if Rc::ptr_eq(&sym, &rejected)), "constructor-index failure returns rejected symbol");
    assert!(same_scope(&scope, &before), "constructor-index failure rolls back registration");
    let source = format!("{BOOLS}\nAlias : Type\nValue : Alias");
    let mut scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let alias = handle!(parser.expression("Alias", &scope), ExpressionFailed);
    let before = scope.clone();
    let result = scope.insert_rewrite(alias.head().clone(), Rewrite::new(Vec::new(), alias.clone()));
    assert!(matches!(result, Err(ScopeInsertRewriteError::BuildConstructorsFailed { sym, rewrite, .. }) if Rc::ptr_eq(&sym, alias.head()) && rewrite.patterns.is_empty() && same_exp(&rewrite.body, &alias)), "rule index failure returns both rejected inputs");
    assert!(same_scope(&scope, &before), "constructor-index failure rolls back defining rules");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveRollbackContractError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
}
