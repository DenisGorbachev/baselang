use aist_core::{ContractReport, observe};
use baselang::{Scope, Sym, Typ};
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

pub fn scope_invariants(report: &mut ContractReport, scope: &Scope) {
    let bindings = scope.bindings();
    report.require(
        "binding keys equal symbol identifiers",
        bindings
            .iter()
            .all(|(name, sym)| name == sym.nym().identifier()),
    );
    report.require(
        "rewrite keys identify registered allocations",
        scope.rewrites().keys().all(|key| {
            bindings
                .values()
                .any(|sym| *key == NonNull::from(sym.as_ref()))
        }),
    );
    scope.rewrites().values().for_each(|rules| {
        report.require(
            "rules of one symbol have equal arity",
            rules
                .windows(2)
                .all(|pair| matches!(pair, [left, right] if left.patterns.len() == right.patterns.len())),
        );
    });
    bindings.values().for_each(|sym| {
        scope.rewrites_of(sym).iter().for_each(|rule| {
            let lhs = observe!(report, scope.apply_many(sym, &rule.patterns));
            let body = observe!(report, scope.infer(&rule.body));
            let valid = observe!(report, scope.convertible(lhs.typ(), &body));
            report.require("rule body has the applied head's type", valid);
        });
    });
    let mut expected: HashMap<NonNull<Sym>, HashSet<NonNull<Sym>>> = HashMap::new();
    bindings
        .values()
        .filter(|sym| scope.rewrites_of(sym).is_empty())
        .for_each(|sym| {
            let terminal = observe!(report, scope.normalize_typ(sym.typ().last()));
            if let Typ::One(exp) = terminal {
                expected
                    .entry(NonNull::from(exp.head().as_ref()))
                    .or_default()
                    .insert(NonNull::from(sym.as_ref()));
            }
        });
    let actual = scope
        .constructors()
        .iter()
        .map(|(head, syms)| {
            let unique = syms
                .iter()
                .map(|sym| NonNull::from(sym.as_ref()))
                .collect::<HashSet<_>>();
            report.equal("constructor index has no duplicate allocation", unique.len(), syms.len());
            (*head, unique)
        })
        .collect();
    report.equal("constructor index equals normalized terminal-head predicate", actual, expected);
}
