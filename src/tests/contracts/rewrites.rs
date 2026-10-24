use crate::{AlphaEq, Exp, PlainParser, Rewrite, ScopeInsertRewriteError, Sym, SymRc, Typ};
use crate::{BOOLS, same_exp, same_scope, scope_invariants};
use aist_core::{ContractReport, observe};
use std::rc::Rc;

pub fn rewrites(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let mut scope = observe!(report, parser.parse_scope(BOOLS));
    let body = observe!(report, parser.expression("True", &scope));
    let defined = observe!(report, scope.define("Defined", &body));
    report.require("define infers type", defined.typ().alpha_eq(body.typ()));
    report.require("define atomically registers one zero-argument rule", scope.is_registered(&defined) && matches!(scope.rewrites_of(&defined), [rule] if rule.patterns.is_empty() && same_exp(&rule.body, &body)));
    let typ = defined.typ().clone();
    let before = scope.clone();
    report.require("duplicate definition fails", scope.define("Defined", &body).is_err());
    report.require("failed define preserves all scope maps", same_scope(&before, &scope));
    let invalid: SymRc = Sym::new_rc("Unknown", body.typ().clone());
    report.require("definition with unbound body fails", scope.define("Invalid", &invalid).is_err());
    report.require("failed definition does not register its head", same_scope(&before, &scope));
    let foreign: SymRc = Sym::new_rc("Defined", typ.clone());
    let rewrite = Rewrite::new(Vec::new(), body.clone());
    let failed = scope.insert_rewrite(foreign.clone(), rewrite.clone());
    report.require("foreign identity rejected with owned inputs", matches!(failed, Err(ScopeInsertRewriteError::SymbolNotFound { sym, rewrite: rejected }) if Rc::ptr_eq(&sym, &foreign) && same_exp(&rejected.body, &rewrite.body) && rejected.patterns.is_empty()));
    report.require("failed rule leaves scope unchanged", same_scope(&before, &scope));
    let wrong_type = Rewrite::new(Vec::new(), Exp::sol(&observe!(report, scope.new_sym("Unrelated", Typ::Top))));
    let before = scope.clone();
    report.require("incorrect rule body type rejected", scope.insert_rewrite(defined.clone(), wrong_type).is_err());
    report.require("invalid rule insertion is atomic", same_scope(&before, &scope));
    observe!(report, scope.insert_rewrite(defined.clone(), rewrite));
    report.equal("successful insertion appends", scope.rewrites_of(&defined).len(), 2);
    report.require(
        "rewriting preserves head identity and type",
        scope
            .sym("Defined")
            .is_some_and(|sym| Rc::ptr_eq(sym, &defined))
            && defined.typ().alpha_eq(&typ),
    );
    report.require("lookup uses identity", scope.rewrites_of(&foreign).is_empty());
    let before = scope.clone();
    report.require(
        "inconsistent rule arity rejected",
        scope
            .insert_rewrite(defined.clone(), Rewrite::new(vec![body.clone()], body.clone()))
            .is_err(),
    );
    report.require("arity rejection is atomic", same_scope(&before, &scope));
    let fun = observe!(report, scope.new_sym("Function", Typ::fun(&Sym::new_rc("x", body.typ().clone()), body.typ().clone())));
    let forged = Exp::App(Box::new(Exp::sol(&fun)), Box::new(body.clone()), Box::new(Typ::Top));
    let before = scope.clone();
    report.require(
        "invalid cached application rejected",
        scope
            .insert_rewrite(defined, Rewrite::new(Vec::new(), forged))
            .is_err(),
    );
    report.require("cached-type rejection is atomic", same_scope(&before, &scope));
    let alias = observe!(report, scope.define("FunctionAlias", &fun));
    report.require("zero-argument rules allow function bodies", alias.typ().alpha_eq(fun.typ()));
    scope_invariants(report, &scope);
    rejected_patterns(report);
}

pub fn rejected_patterns(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nF : Bool -> Bool -> Bool");
    let mut scope = observe!(report, parser.parse_scope(&source));
    let fun = observe!(report, parser.expression("F", &scope));
    let truth = observe!(report, parser.expression("True", &scope));
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
        report.require(name, matches!(failed, Err(ScopeInsertRewriteError::RuleFailed { sym, rewrite, .. }) if Rc::ptr_eq(&sym, fun.head()) && rewrite.patterns.len() == original.patterns.len() && rewrite.patterns.iter().zip(&original.patterns).all(|(a,b)| same_exp(a,b)) && same_exp(&rewrite.body, &original.body)));
        report.require("pattern rejection preserves scope", same_scope(&before, &scope));
    });
}
