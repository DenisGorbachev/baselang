use crate::{same_scope, scope_invariants};
use aist_core::{ContractReport, observe};
use baselang::{Nym, Scope, ScopeInsertSymError, Sym, SymRc, Typ};
use std::rc::Rc;

pub fn registration(report: &mut ContractReport) {
    let mut scope: Scope = Scope::default();
    report.require("default scope is empty", scope.bindings().is_empty() && scope.rewrites().is_empty() && scope.constructors().is_empty());
    let raw: SymRc = Sym::new_top_rc("Raw");
    report.require("raw symbol construction does not register", scope.bindings().is_empty());
    report.require("unregistered identities have no rules or constructors", scope.rewrites_of(&raw).is_empty() && scope.constructors_of(&raw).is_empty());
    let registered = observe!(report, scope.insert_sym(raw.clone()));
    report.require("insert_sym returns the supplied allocation", Rc::ptr_eq(&raw, &registered));
    let rejected: SymRc = Sym::new_top_rc("Raw");
    let before = scope.clone();
    report.require("occupied identifier returns the rejected symbol", matches!(scope.insert_sym(rejected.clone()), Err(ScopeInsertSymError::NymOccupiedInvalid { sym }) if Rc::ptr_eq(&sym, &rejected)));
    report.require("failed registration leaves scope unchanged", same_scope(&scope, &before));
    ["", ".A", "A.", "A..B", "1A", "A-B", "A.1B", " A", "A B"]
        .into_iter()
        .for_each(|name| {
            let before = scope.clone();
            report.require(format!("reject identifier {name:?}"), scope.new_sym(name, Typ::Top).is_err());
            report.require("invalid identifier leaves scope unchanged", same_scope(&scope, &before));
        });
    ["A", "_", "_A", "A0", "A.B", "Α.β", "名.字"]
        .into_iter()
        .for_each(|name| {
            let sym = observe!(report, scope.new_sym(name, Typ::Top));
            report.require(
                "new_sym returns a registered symbol",
                scope
                    .sym(name)
                    .is_some_and(|registered| Rc::ptr_eq(registered, &sym)),
            );
            let nym = Nym::from(name);
            report.equal("Nym::identifier selects canonical singular English", nym.identifier(), nym.short.en.singular.canonical.as_str());
        });
    let parameter: SymRc = Sym::new_top_rc("parameter");
    observe!(report, scope.new_sym("Function", Typ::fun(&parameter, Typ::Top)));
    report.require("binding a function parameter does not register it", scope.sym("parameter").is_none());
    scope_invariants(report, &scope);
}
