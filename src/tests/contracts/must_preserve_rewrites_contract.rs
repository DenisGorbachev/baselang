use crate::{AlphaEq, Exp, PlainParser, Rewrite, ScopeInsertRewriteError, Sym, SymRc, Typ};
use crate::{AssertScopeInvariantsError, PlainParserExpressionError, PlainParserParseScopeError, ScopeDefineError, ScopeNewSymError};
use crate::{BOOLS, assert_scope_invariants, same_exp, same_scope};
use errgonomic::handle;
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_rewrites_contract() -> Result<(), MustPreserveRewritesContractError> {
    use MustPreserveRewritesContractError::*;
    let parser = PlainParser::default();
    let mut scope = handle!(parser.parse_scope(BOOLS), ParseScopeFailed);
    let body = handle!(parser.expression("True", &scope), ExpressionFailed);
    let defined = handle!(scope.define("Defined", &body), DefineFailed);
    assert!(defined.typ().alpha_eq(body.typ()), "define infers type");
    assert!(scope.is_registered(&defined) && matches!(scope.rewrites_of(&defined), [rule] if rule.patterns.is_empty() && same_exp(&rule.body, &body)), "define atomically registers one zero-argument rule");
    let typ = defined.typ().clone();
    let before = scope.clone();
    assert!(scope.define("Defined", &body).is_err(), "duplicate definition fails");
    assert!(same_scope(&before, &scope), "failed define preserves all scope maps");
    let invalid: SymRc = Sym::new_rc("Unknown", body.typ().clone());
    assert!(scope.define("Invalid", &invalid).is_err(), "definition with unbound body fails");
    assert!(same_scope(&before, &scope), "failed definition does not register its head");
    let foreign: SymRc = Sym::new_rc("Defined", typ.clone());
    let rewrite = Rewrite::new(Vec::new(), body.clone());
    let failed = scope.insert_rewrite(foreign.clone(), rewrite.clone());
    assert!(matches!(failed, Err(ScopeInsertRewriteError::SymbolNotFound { sym, rewrite: rejected }) if Rc::ptr_eq(&sym, &foreign) && same_exp(&rejected.body, &rewrite.body) && rejected.patterns.is_empty()), "foreign identity rejected with owned inputs");
    assert!(same_scope(&before, &scope), "failed rule leaves scope unchanged");
    let wrong_type = Rewrite::new(Vec::new(), Exp::sol(&handle!(scope.new_sym("Unrelated", Typ::Top), NewSymFailed)));
    let before = scope.clone();
    assert!(scope.insert_rewrite(defined.clone(), wrong_type).is_err(), "incorrect rule body type rejected");
    assert!(same_scope(&before, &scope), "invalid rule insertion is atomic");
    let result = scope.insert_rewrite(defined.clone(), rewrite);
    handle!(result, InsertRewriteFailed);
    assert_eq!(scope.rewrites_of(&defined).len(), 2, "successful insertion appends");
    assert!(
        scope
            .sym("Defined")
            .is_some_and(|sym| Rc::ptr_eq(sym, &defined))
            && defined.typ().alpha_eq(&typ),
        "rewriting preserves head identity and type"
    );
    assert!(scope.rewrites_of(&foreign).is_empty(), "lookup uses identity");
    let before = scope.clone();
    assert!(
        scope
            .insert_rewrite(defined.clone(), Rewrite::new(vec![body.clone()], body.clone()))
            .is_err(),
        "inconsistent rule arity rejected"
    );
    assert!(same_scope(&before, &scope), "arity rejection is atomic");
    let result = scope.new_sym("Function", Typ::fun(&Sym::new_rc("x", body.typ().clone()), body.typ().clone()));
    let fun = handle!(result, NewSymFailed);
    let forged = Exp::App(Box::new(Exp::sol(&fun)), Box::new(body.clone()), Box::new(Typ::Top));
    let before = scope.clone();
    assert!(
        scope
            .insert_rewrite(defined, Rewrite::new(Vec::new(), forged))
            .is_err(),
        "invalid cached application rejected"
    );
    assert!(same_scope(&before, &scope), "cached-type rejection is atomic");
    let alias = handle!(scope.define("FunctionAlias", &fun), DefineFailed);
    assert!(alias.typ().alpha_eq(fun.typ()), "zero-argument rules allow function bodies");
    handle!(assert_scope_invariants(&scope), AssertScopeInvariantsFailed);
    handle!(assert_rejected_patterns(), AssertRejectedPatternsFailed);
    Ok(())
}

pub fn assert_rejected_patterns() -> Result<(), AssertRejectedPatternsError> {
    use AssertRejectedPatternsError::*;
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nF : Bool -> Bool -> Bool");
    let mut scope = handle!(parser.parse_scope(&source), ParseScopeFailed);
    let fun = handle!(parser.expression("F", &scope), ExpressionFailed);
    let truth = handle!(parser.expression("True", &scope), ExpressionFailed);
    let x: SymRc = Sym::new_rc("x", truth.typ().clone());
    let y: SymRc = Sym::new_rc("y", truth.typ().clone());
    [
        ("nonlinear pattern", Rewrite::new(vec![Exp::sol(&x), Exp::sol(&x)], truth.clone())),
        ("unbound body", Rewrite::new(vec![Exp::sol(&x), truth.clone()], Exp::sol(&y))),
        ("pattern type mismatch", Rewrite::new(vec![fun.clone(), truth.clone()], truth.clone())),
    ]
    .into_iter()
    .for_each(|(name, rule)| {
        let before = scope.clone();
        let original = rule.clone();
        let failed = scope.insert_rewrite(fun.head().clone(), rule);
        assert!(matches!(failed, Err(ScopeInsertRewriteError::RuleFailed { sym, rewrite, .. }) if Rc::ptr_eq(&sym, fun.head()) && rewrite.patterns.len() == original.patterns.len() && rewrite.patterns.iter().zip(&original.patterns).all(|(a,b)| same_exp(a,b)) && same_exp(&rewrite.body, &original.body)), "{}", name);
        assert!(same_scope(&before, &scope), "pattern rejection preserves scope");
    });
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveRewritesContractError {
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
    #[error("failed to call insert_rewrite")]
    InsertRewriteFailed { source: ScopeInsertRewriteError },
    #[error("failed to call define")]
    DefineFailed { source: ScopeDefineError },
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call assert_scope_invariants")]
    AssertScopeInvariantsFailed { source: AssertScopeInvariantsError },
    #[error("failed to call assert_rejected_patterns")]
    AssertRejectedPatternsFailed { source: AssertRejectedPatternsError },
}

#[derive(Error, Debug)]
pub enum AssertRejectedPatternsError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
}
