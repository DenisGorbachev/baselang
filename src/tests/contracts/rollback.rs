use crate::{BOOLS, NATS, same_exp, same_scope};
use crate::{PlainParser, Rewrite, ScopeInsertRewriteError, ScopeInsertSymError, Sym, SymRc, Typ};
use aist_core::{ContractReport, observe};
use std::rc::Rc;

pub fn rollback(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let source = format!("{NATS}\nLoop : Nat -> Nat\nLoop x => Loop x");
    let mut scope = observe!(report, parser.parse_scope(&source));
    let looping = observe!(report, parser.expression("Loop Zero", &scope));
    let rejected: SymRc = Sym::new_rc("Rejected", Typ::one(looping));
    let before = scope.clone();
    let result = scope.insert_sym(rejected.clone());
    report.require("constructor-index failure returns rejected symbol", matches!(result, Err(ScopeInsertSymError::BuildConstructorsFailed { sym, .. }) if Rc::ptr_eq(&sym, &rejected)));
    report.require("constructor-index failure rolls back registration", same_scope(&scope, &before));
    let source = format!("{BOOLS}\nAlias : Type\nValue : Alias");
    let mut scope = observe!(report, parser.parse_scope(&source));
    let alias = observe!(report, parser.expression("Alias", &scope));
    let before = scope.clone();
    let result = scope.insert_rewrite(alias.head().clone(), Rewrite::new(Vec::new(), alias.clone()));
    report.require("rule index failure returns both rejected inputs", matches!(result, Err(ScopeInsertRewriteError::BuildConstructorsFailed { sym, rewrite, .. }) if Rc::ptr_eq(&sym, alias.head()) && rewrite.patterns.is_empty() && same_exp(&rewrite.body, &alias)));
    report.require("constructor-index failure rolls back defining rules", same_scope(&scope, &before));
}
