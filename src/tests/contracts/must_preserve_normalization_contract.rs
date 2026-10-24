use crate::{AssertScopeInvariantsError, PlainParserExpressionError, PlainParserParseScopeError, ScopeApplyManyError, ScopeInsertRewriteError, ScopeNormalizeError};
use crate::{BOOLS, NATS, assert_scope_invariants, same_exp};
use crate::{Exp, PlainParser, Rewrite, Scope};
use errgonomic::{ErrVec, handle, handle_iter};
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_normalization_contract() -> Result<(), MustPreserveNormalizationContractError> {
    use MustPreserveNormalizationContractError::*;
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False\nNot False => True\nAlias => True\nNotAlias => Not\nFirst : Bool -> Bool -> Bool\nFirst x y => x\nReturnNot : Bool -> Bool -> Bool\nReturnNot x => Not");
    let scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let results = [
        ("Not Alias", "False"),
        ("Not (Not True)", "True"),
        ("NotAlias True", "False"),
        ("First False True", "False"),
        ("First True", "First True"),
        ("ReturnNot True False", "True"),
    ]
    .into_iter()
    .map(|(input, expected)| assert_normalizes(&parser, &scope, input, expected));
    handle_iter!(results, AssertNormalizesFailed);
    let first = handle!(parser.expression("First", &scope), ExpressionFailed);
    let [rule] = scope.rewrites_of(first.head()) else {
        return Err(RulesInvalid {
            actual: scope.rewrites_of(first.head()).len(),
        });
    };
    let [_, second] = rule.patterns.as_slice() else {
        return Err(PatternsInvalid {
            actual: rule.patterns.len(),
        });
    };
    let truth = handle!(parser.expression("True", &scope), ExpressionFailed);
    let input = handle!(scope.apply_many(&first, [second, &truth]), ApplyManyFailed);
    let actual = handle!(scope.normalize(&input), NormalizeFailed);
    assert!(same_exp(&actual, second), "pattern substitution is simultaneous");
    let source = format!("{NATS}\nIdentity : Nat -> Nat\nIdentity Zero => Zero\nIdentity (Succ n) => Succ (Identity n)");
    let scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let input = handle!(parser.expression("Identity (Succ (Succ Zero))", &scope), ExpressionFailed);
    let expected = handle!(parser.expression("Succ (Succ Zero)", &scope), ExpressionFailed);
    let actual = handle!(scope.normalize(&input), NormalizeFailed);
    assert!(same_exp(&actual, &expected), "structural descent normalizes recursively");
    let source = format!("{BOOLS}\nLoop : Bool -> Bool\nLoop x => Loop x");
    let scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let input = handle!(parser.expression("Loop True", &scope), ExpressionFailed);
    assert!(scope.normalize(&input).is_err(), "recursive cycle without structural descent errors");
    handle!(assert_constructor_changes(), AssertConstructorChangesFailed);
    Ok(())
}

pub fn assert_constructor_changes() -> Result<(), AssertConstructorChangesError> {
    use AssertConstructorChangesError::*;
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nAlias : Type\nValue : Alias");
    let mut scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let bool_ = handle!(parser.expression("Bool", &scope), ExpressionFailed);
    let alias = handle!(parser.expression("Alias", &scope), ExpressionFailed);
    let value = handle!(parser.expression("Value", &scope), ExpressionFailed);
    assert!(
        scope
            .constructors_of(alias.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, value.head())),
        "constructor indexed under original terminal head"
    );
    let result = scope.insert_rewrite(alias.head().clone(), Rewrite::new(Vec::new(), bool_.clone()));
    handle!(result, InsertRewriteFailed);
    assert!(
        scope.constructors_of(alias.head()).is_empty()
            && scope
                .constructors_of(bool_.head())
                .iter()
                .any(|sym| Rc::ptr_eq(sym, value.head())),
        "index follows normalized terminal type"
    );
    let truth = handle!(parser.expression("True", &scope), ExpressionFailed);
    let result = scope.insert_rewrite(value.head().clone(), Rewrite::new(Vec::new(), truth));
    handle!(result, InsertRewriteFailed);
    assert!(
        !scope
            .constructors_of(bool_.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, value.head())),
        "defined symbols cease to be constructors"
    );
    assert!(
        !scope
            .constructors()
            .values()
            .flatten()
            .any(|sym| Rc::ptr_eq(sym, bool_.head())),
        "Top-returning symbols are not constructors"
    );
    handle!(assert_scope_invariants(&scope), AssertScopeInvariantsFailed);
    assert!(Rc::ptr_eq(Exp::sol(value.head()).head(), value.head()), "symbol spine preserves head");
    Ok(())
}

pub fn assert_normalizes(parser: &PlainParser, scope: &Scope, input: &str, expected: &str) -> Result<(), AssertNormalizesError> {
    use AssertNormalizesError::*;
    let equation = format!("normalize({input}) = {expected}");
    let input = handle!(parser.expression(input, scope), ExpressionFailed);
    let expected = handle!(parser.expression(expected, scope), ExpressionFailed);
    let actual = handle!(scope.normalize(&input), NormalizeFailed);
    assert!(same_exp(&actual, &expected), "{equation}");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveNormalizationContractError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call normalize")]
    NormalizeFailed { source: ScopeNormalizeError },
    #[error("failed to call apply_many")]
    ApplyManyFailed { source: ScopeApplyManyError },
    #[error("failed to call assert_normalizes")]
    AssertNormalizesFailed { source: ErrVec<AssertNormalizesError> },
    #[error("failed to call assert_constructor_changes")]
    AssertConstructorChangesFailed { source: AssertConstructorChangesError },
    #[error("unexpected rules count: {actual} for First")]
    RulesInvalid { actual: usize },
    #[error("unexpected patterns count: {actual} for First")]
    PatternsInvalid { actual: usize },
}

#[derive(Error, Debug)]
pub enum AssertConstructorChangesError {
    #[error("failed to call insert_rewrite")]
    InsertRewriteFailed { source: ScopeInsertRewriteError },
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call assert_scope_invariants")]
    AssertScopeInvariantsFailed { source: AssertScopeInvariantsError },
}

#[derive(Error, Debug)]
pub enum AssertNormalizesError {
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call normalize")]
    NormalizeFailed { source: ScopeNormalizeError },
}
