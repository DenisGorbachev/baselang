use crate::{BOOLS, NATS, same_scope, scope_invariants};
use crate::{Exp, PlainParser, Scope, Sym, SymRc, Theory, TheoryNewError, Typ};
use aist_core::{ContractReport, observe};

pub fn theory(report: &mut ContractReport) {
    let parser = PlainParser::default();
    [
        ("empty", "".to_owned(), true),
        ("constructors", BOOLS.to_owned(), true),
        ("complete coverage", format!("{BOOLS}\nF : Bool -> Bool\nF True => False\nF False => True"), true),
        ("incomplete coverage", format!("{BOOLS}\nF : Bool -> Bool\nF True => False"), false),
        ("conflicting overlap", format!("{BOOLS}\nF : Bool -> Bool\nF x => True\nF y => False"), false),
        ("structural recursion", format!("{NATS}\nF : Nat -> Nat\nF Zero => Zero\nF (Succ n) => Succ (F n)"), true),
        ("nondecreasing recursion", format!("{BOOLS}\nF : Bool -> Bool\nF x => F x"), false),
        ("nondecreasing mutual recursion", format!("{BOOLS}\nF : Bool -> Bool\nG : Bool -> Bool\nF x => G x\nG x => F x"), false),
        ("strict negativity", "Bad : Type\nMk : (Bad -> Bad) -> Bad".to_owned(), false),
        ("double negativity", "Bad : Type\nMk : ((Bad -> Bad) -> Bad) -> Bad".to_owned(), false),
        ("complete rule set changes constructors", format!("{BOOLS}\nAlias : Bool\nF : Bool -> Bool\nF True => False\nF False => True\nAlias => True"), true),
    ]
    .into_iter()
    .for_each(|(name, source, expected)| {
        let scope = observe!(report, parser.parse_scope(&source));
        let before = scope.clone();
        match Theory::new(scope) {
            Ok(theory) => {
                report.require(format!("{name}/accepted"), expected);
                report.require("theory retains declarations and rules", same_scope(theory.scope(), &before));
                scope_invariants(report, theory.scope());
            }
            Err(TheoryNewError::ValidateFailed {
                scope,
                ..
            }) => {
                report.require(format!("{name}/rejected"), !expected);
                report.require("rejected theory owns unchanged scope", same_scope(&scope, &before));
            }
        }
    });
    let free: SymRc = Sym::new_top_rc("Free");
    let mut scope = Scope::default();
    observe!(report, scope.new_sym("Unbound", &free));
    report.require("unregistered references rejected", Theory::new(scope).is_err());
    let mut scope = observe!(report, parser.parse_scope(BOOLS));
    let truth = observe!(report, parser.expression("True", &scope));
    let bad = Exp::App(Box::new(truth.clone()), Box::new(truth), Box::new(Typ::Top));
    observe!(report, scope.new_sym("BadCache", Typ::one(bad)));
    report.require("declarations with invalid cached expressions rejected", Theory::new(scope).is_err());
}
