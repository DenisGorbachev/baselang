use crate::{BOOLS, NATS, same_exp, scope_invariants};
use aist_core::{ContractReport, observe};
use baselang::{Exp, PlainParser, Rewrite};
use std::rc::Rc;

pub fn normalization(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False\nNot False => True\nAlias => True\nNotAlias => Not\nFirst : Bool -> Bool -> Bool\nFirst x y => x\nReturnNot : Bool -> Bool -> Bool\nReturnNot x => Not");
    let scope = observe!(report, parser.parse_scope(&source));
    [
        ("Not Alias", "False"),
        ("Not (Not True)", "True"),
        ("NotAlias True", "False"),
        ("First False True", "False"),
        ("First True", "First True"),
        ("ReturnNot True False", "True"),
    ]
    .into_iter()
    .for_each(|(input, expected)| {
        let equation = format!("normalize({input}) = {expected}");
        let input = observe!(report, parser.expression(input, &scope));
        let expected = observe!(report, parser.expression(expected, &scope));
        let actual = observe!(report, scope.normalize(&input));
        report.require(equation, same_exp(&actual, &expected));
    });
    let first = observe!(report, parser.expression("First", &scope));
    let [rule] = scope.rewrites_of(first.head()) else {
        report.require("First has one rule", false);
        return;
    };
    let [_, second] = rule.patterns.as_slice() else {
        report.require("First has two ordered patterns", false);
        return;
    };
    let truth = observe!(report, parser.expression("True", &scope));
    let input = observe!(report, scope.apply_many(&first, [second, &truth]));
    let actual = observe!(report, scope.normalize(&input));
    report.require("pattern substitution is simultaneous", same_exp(&actual, second));
    let source = format!("{NATS}\nIdentity : Nat -> Nat\nIdentity Zero => Zero\nIdentity (Succ n) => Succ (Identity n)");
    let scope = observe!(report, parser.parse_scope(&source));
    let input = observe!(report, parser.expression("Identity (Succ (Succ Zero))", &scope));
    let expected = observe!(report, parser.expression("Succ (Succ Zero)", &scope));
    let actual = observe!(report, scope.normalize(&input));
    report.require("structural descent normalizes recursively", same_exp(&actual, &expected));
    let source = format!("{BOOLS}\nLoop : Bool -> Bool\nLoop x => Loop x");
    let scope = observe!(report, parser.parse_scope(&source));
    let input = observe!(report, parser.expression("Loop True", &scope));
    report.require("recursive cycle without structural descent errors", scope.normalize(&input).is_err());
    constructor_changes(report);
}

pub fn constructor_changes(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\nAlias : Type\nValue : Alias");
    let mut scope = observe!(report, parser.parse_scope(&source));
    let bool_ = observe!(report, parser.expression("Bool", &scope));
    let alias = observe!(report, parser.expression("Alias", &scope));
    let value = observe!(report, parser.expression("Value", &scope));
    report.require(
        "constructor indexed under original terminal head",
        scope
            .constructors_of(alias.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, value.head())),
    );
    observe!(report, scope.insert_rewrite(alias.head().clone(), Rewrite::new(Vec::new(), bool_.clone())));
    report.require(
        "index follows normalized terminal type",
        scope.constructors_of(alias.head()).is_empty()
            && scope
                .constructors_of(bool_.head())
                .iter()
                .any(|sym| Rc::ptr_eq(sym, value.head())),
    );
    let truth = observe!(report, parser.expression("True", &scope));
    observe!(report, scope.insert_rewrite(value.head().clone(), Rewrite::new(Vec::new(), truth)));
    report.require(
        "defined symbols cease to be constructors",
        !scope
            .constructors_of(bool_.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, value.head())),
    );
    report.require(
        "Top-returning symbols are not constructors",
        !scope
            .constructors()
            .values()
            .flatten()
            .any(|sym| Rc::ptr_eq(sym, bool_.head())),
    );
    scope_invariants(report, &scope);
    report.require("symbol spine preserves head", Rc::ptr_eq(Exp::sol(value.head()).head(), value.head()));
}
