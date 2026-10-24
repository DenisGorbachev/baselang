use crate::{AlphaEq, PlainParser, Rewrite, Scope, Typ};
use crate::{AssertScopeInvariantsError, PlainParserExpressionError, PlainParserParseScopeError, ScopeInsertRewriteError, ScopeNewSymError, ScopeNormalizeTypError};
use crate::{NATS, assert_scope_invariants};
use errgonomic::{ErrVec, handle, handle_iter};
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_constructors_contract() -> Result<(), MustPreserveConstructorsContractError> {
    use MustPreserveConstructorsContractError::*;
    let parser = PlainParser::default();
    let source = format!("{NATS}\nList : Type -> Type\nNil : (T : Type) -> List T\nAlias => Nat");
    let mut scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let nat = handle!(parser.expression("Nat", &scope), ExpressionFailed);
    let alias = handle!(parser.expression("Alias", &scope), ExpressionFailed);
    let expected = handle!(scope.normalize_typ(&Typ::one(alias)), NormalizeTypFailed);
    let results = [
        ("Zero", true),
        ("Succ", true),
        ("Nat", false),
        ("List", false),
        ("Nil", false),
    ]
    .into_iter()
    .map(|(name, is_constructor)| assert_constructor(&scope, name, &expected, is_constructor));
    handle_iter!(results, AssertConstructorFailed);
    let nil = handle!(parser.expression("Nil", &scope), ExpressionFailed);
    let list = handle!(parser.expression("List", &scope), ExpressionFailed);
    assert!(
        scope
            .constructors_of(list.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, nil.head())),
        "parameterized constructor indexed by type-family head"
    );
    let zero = handle!(parser.expression("Zero", &scope), ExpressionFailed);
    let alias_value = handle!(scope.new_sym("AliasValue", nat.head()), NewSymFailed);
    let result = scope.insert_rewrite(alias_value.clone(), Rewrite::new(Vec::new(), zero));
    handle!(result, InsertRewriteFailed);
    assert!(
        !scope
            .constructors_of(nat.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, &alias_value)),
        "defining rule excludes a symbol from constructors"
    );
    handle!(assert_scope_invariants(&scope), AssertScopeInvariantsFailed);
    Ok(())
}

pub fn assert_constructor(scope: &Scope, name: &str, typ: &Typ, expected: bool) -> Result<(), AssertConstructorError> {
    use AssertConstructorError::*;
    let parser = PlainParser::default();
    let exp = handle!(parser.expression(name, scope), ExpressionFailed);
    let terminal = handle!(scope.normalize_typ(exp.head().typ().last()), NormalizeTypFailed);
    let actual = scope.rewrites_of(exp.head()).is_empty() && matches!(terminal, Typ::One(_)) && terminal.alpha_eq(typ);
    assert_eq!(actual, expected, "{name}/constructor of normalized type");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveConstructorsContractError {
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
    #[error("failed to call insert_rewrite")]
    InsertRewriteFailed { source: ScopeInsertRewriteError },
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call normalize_typ")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("failed to call assert_scope_invariants")]
    AssertScopeInvariantsFailed { source: AssertScopeInvariantsError },
    #[error("failed to call assert_constructor")]
    AssertConstructorFailed { source: ErrVec<AssertConstructorError> },
}

#[derive(Error, Debug)]
pub enum AssertConstructorError {
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call normalize_typ")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
}
