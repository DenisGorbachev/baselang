use crate::{NATS, scope_invariants};
use aist_core::{ContractReport, observe};
use baselang::{AlphaEq, PlainParser, Rewrite, Typ};
use std::rc::Rc;

pub fn constructors(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let source = format!("{NATS}\nList : Type -> Type\nNil : (T : Type) -> List T\nAlias => Nat");
    let mut scope = observe!(report, parser.parse_scope(&source));
    let nat = observe!(report, parser.expression("Nat", &scope));
    let alias = observe!(report, parser.expression("Alias", &scope));
    let expected = observe!(report, scope.normalize_typ(&Typ::one(alias)));
    ["Zero", "Succ", "Nat", "List", "Nil"]
        .into_iter()
        .for_each(|name| {
            let exp = observe!(report, parser.expression(name, &scope));
            let terminal = observe!(report, scope.normalize_typ(exp.head().typ().last()));
            let predicate = scope.rewrites_of(exp.head()).is_empty() && matches!(terminal, Typ::One(_)) && terminal.alpha_eq(&expected);
            report.equal(format!("{name}/constructor of normalized type Nat"), predicate, matches!(name, "Zero" | "Succ"));
        });
    let nil = observe!(report, parser.expression("Nil", &scope));
    let list = observe!(report, parser.expression("List", &scope));
    report.require(
        "parameterized constructor indexed by type-family head",
        scope
            .constructors_of(list.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, nil.head())),
    );
    let zero = observe!(report, parser.expression("Zero", &scope));
    let alias_value = observe!(report, scope.new_sym("AliasValue", nat.head()));
    observe!(report, scope.insert_rewrite(alias_value.clone(), Rewrite::new(Vec::new(), zero)));
    report.require(
        "defining rule excludes a symbol from constructors",
        !scope
            .constructors_of(nat.head())
            .iter()
            .any(|sym| Rc::ptr_eq(sym, &alias_value)),
    );
    scope_invariants(report, &scope);
}
