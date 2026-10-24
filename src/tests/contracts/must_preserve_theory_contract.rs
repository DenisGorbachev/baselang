use crate::{AssertScopeInvariantsError, PlainParserExpressionError, PlainParserParseScopeError, ScopeNewSymError};
use crate::{BOOLS, NATS, assert_scope_invariants, same_scope};
use crate::{Exp, PlainParser, Scope, Sym, SymRc, Theory, TheoryNewError, Typ};
use errgonomic::{ErrVec, handle, handle_iter};
use thiserror::Error;

#[test]
fn must_preserve_theory_contract() -> Result<(), MustPreserveTheoryContractError> {
    use MustPreserveTheoryContractError::*;
    let parser = PlainParser::default();
    let cases = [
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
    ];
    let results = cases
        .into_iter()
        .map(|(name, source, expected)| assert_theory(&parser, name, &source, expected));
    handle_iter!(results, AssertTheoryFailed);
    let free: SymRc = Sym::new_top_rc("Free");
    let mut scope = Scope::default();
    handle!(scope.new_sym("Unbound", &free), NewSymFailed);
    assert!(Theory::new(scope).is_err(), "unregistered references rejected");
    let mut scope = handle!(parser.parse_scope(BOOLS), ParseScopeFailed);
    let truth = handle!(parser.expression("True", &scope), ExpressionFailed);
    let bad = Exp::App(Box::new(truth.clone()), Box::new(truth), Box::new(Typ::Top));
    handle!(scope.new_sym("BadCache", Typ::one(bad)), NewSymFailed);
    assert!(Theory::new(scope).is_err(), "declarations with invalid cached expressions rejected");
    Ok(())
}

pub fn assert_theory(parser: &PlainParser, name: &str, source: &str, expected: bool) -> Result<(), AssertTheoryError> {
    use AssertTheoryError::*;
    let scope = handle!(parser.parse_scope(source), ParseScopeFailed);
    let before = scope.clone();
    match Theory::new(scope) {
        Ok(theory) => {
            assert!(expected, "{name}/accepted");
            assert!(same_scope(theory.scope(), &before), "theory retains declarations and rules");
            handle!(assert_scope_invariants(theory.scope()), AssertScopeInvariantsFailed);
        }
        Err(TheoryNewError::ValidateFailed {
            scope,
            ..
        }) => {
            assert!(!expected, "{name}/rejected");
            assert!(same_scope(&scope, &before), "rejected theory owns unchanged scope");
        }
    }
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveTheoryContractError {
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call expression")]
    ExpressionFailed { source: PlainParserExpressionError },
    #[error("failed to call assert_theory")]
    AssertTheoryFailed { source: ErrVec<AssertTheoryError> },
}

#[derive(Error, Debug)]
pub enum AssertTheoryError {
    #[error("failed to call parse_scope")]
    ParseScopeFailed { source: PlainParserParseScopeError },
    #[error("failed to call assert_scope_invariants")]
    AssertScopeInvariantsFailed { source: AssertScopeInvariantsError },
}
